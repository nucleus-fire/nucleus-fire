use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn server(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    
    let fn_attrs = &input_fn.attrs;
    let fn_vis = &input_fn.vis;
    let fn_sig = &input_fn.sig;
    let fn_name = &fn_sig.ident;
    let fn_body = &input_fn.block;
    
    // Extract arguments for the RPC call
    let args = fn_sig.inputs.iter().map(|arg| {
        match arg {
            syn::FnArg::Typed(pat) => &pat.pat,
            _ => panic!("Self arguments not supported in server actions"),
        }
    });

    let args_tuple = quote! { (#(#args),*) };

    let output = quote! {
        // Server Implementation: Keep original body
        #[cfg(not(target_arch = "wasm32"))]
        #(#fn_attrs)*
        #fn_vis #fn_sig {
            #fn_body
        }

        // Client Implementation: RPC Stub
        #[cfg(target_arch = "wasm32")]
        #(#fn_attrs)*
        #fn_vis #fn_sig {
            let fn_name_str = stringify!(#fn_name);
            nucleus_std::rpc::call(fn_name_str, #args_tuple).await
        }

    };

    output.into()
}

#[proc_macro_attribute]
pub fn store(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::ItemStruct);
    let struct_name = &input.ident;
    let vis = &input.vis;
    let attrs = &input.attrs;

    // Process named fields
    let fields = if let syn::Fields::Named(ref named) = input.fields {
        named
    } else {
        panic!("#[store] only supports structs with named fields");
    };

    let mut signal_fields = Vec::new();
    let mut init_logic = Vec::new();
    let mut default_logic = Vec::new();

    for field in &fields.named {
        let field_name = &field.ident;
        let field_vis = &field.vis;
        let field_ty = &field.ty;

        // Check for default value using a hacky comment convention or attribute would be hard
        // without a custom parser. But standard Rust structs don't have inline defaults.
        // The macro_rules! version allowed `field: type = value`.
        // A proc_macro on a standard struct definition `struct Foo { x: i32 }` doesn't have defaults.
        // We will just assume Default::default() or require user to call new() with all args?
        // To strictly match "Easy syntax", we can generate a constructor that takes initial values.
        
        signal_fields.push(quote! {
            #field_vis #field_name: nucleus_std::neutron::Signal<#field_ty>
        });

        init_logic.push(quote! {
            #field_name: nucleus_std::neutron::Signal::new(#field_name)
        });
        
        default_logic.push(quote! {
             #field_name: nucleus_std::neutron::Signal::new(Default::default())
        });
    }
    
    // Extract constructor args
    let ctor_args = fields.named.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! { #name: #ty }
    });

    let output = quote! {
        #(#attrs)*
        #vis struct #struct_name {
            #(#signal_fields),*
        }

        impl #struct_name {
            /// Create a new store with initial values.
            pub fn new(#(#ctor_args),*) -> Self {
                Self {
                    #(#init_logic),*
                }
            }
        }

        impl Default for #struct_name {
            fn default() -> Self {
                <Self as nucleus_std::neutron::Store>::init()
            }
        }
        
        impl nucleus_std::neutron::Store for #struct_name {
             fn init() -> Self {
                 Self {
                     #(#default_logic),*
                 }
             }
        }
    };

    output.into()
}
