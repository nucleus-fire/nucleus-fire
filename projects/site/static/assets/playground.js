/**
 * Nucleus Playground - Interactive NCL Editor
 */

// Modern Examples Library - Showcasing Nucleus Framework Patterns
const EXAMPLES = {
    // --- STARTERS ---
    hello: {
        ncl: `<n:view title="Hello World">
    <!-- Using Nucleus Components -->
    <main class="min-h-screen flex flex-col items-center justify-center bg-gray-900 text-white text-center p-4 font-sans">
        <Badge variant="primary">⚡ Nucleus</Badge>
        
        <h1 class="text-4xl font-bold my-6 bg-clip-text text-transparent bg-gradient-to-r from-indigo-500 to-purple-500">
            Hello, Nucleus!
        </h1>
        
        <Card variant="glass">
            <p class="text-gray-300 mb-6">
                Welcome to the fastest full-stack framework.
                This example uses <strong>Card</strong>, <strong>Button</strong>, and <strong>Badge</strong> components.
            </p>
            
            <div class="flex gap-4 justify-center">
                <Button variant="primary" href="/docs">Get Started</Button>
                <Button variant="secondary">🎉 Celebrate</Button>
            </div>
        </Card>
    </main>
    
    <n:client>
        // Client-side JavaScript using n:client
        console.log('Nucleus is ready!');
    </n:client>
</n:view>`,
        css: `/* Nucleus scoped styles */
body { margin: 0; font-family: system-ui, -apple-system, sans-serif; }`,
        files: {
            'components/Button.ncl': {
                content: `<n:component name="Button">
    <n:props>
        variant: String = "primary"
        size: String = "md"
        href: String = ""
        type: String = "button"
        onclick: String = ""
        id: String = ""
    </n:props>
    <n:if condition="href">
        <a href="{{ href }}" id="{{ id }}" onclick="{{ onclick }}" class="px-4 py-2 rounded-lg font-medium transition-colors inline-block {{ variant == 'primary' ? 'bg-indigo-600 text-white hover:bg-indigo-700' : 'bg-white text-gray-900 border border-gray-200 hover:bg-gray-50' }}">
            <n:slot />
        </a>
    </n:if>
    <n:if condition="!href">
        <button type="{{ type }}" id="{{ id }}" onclick="{{ onclick }}" class="px-4 py-2 rounded-lg font-medium transition-colors {{ variant == 'primary' ? 'bg-indigo-600 text-white hover:bg-indigo-700' : 'bg-white text-gray-900 border border-gray-200 hover:bg-gray-50' }}">
            <n:slot />
        </button>
    </n:if>
</n:component>`,
                language: 'html'
            },
            'components/Card.ncl': {
                content: `<n:component name="Card">
    <n:props>
        variant: String = "default"
    </n:props>
    <div class="p-8 rounded-2xl border {{ variant == 'glass' ? 'bg-slate-800/50 backdrop-blur border-slate-700' : 'bg-white border-gray-100' }}">
        <n:slot />
    </div>
</n:component>`,
                language: 'html'
            },
            'components/Badge.ncl': {
                content: `<n:component name="Badge">
    <n:props>variant: String = "default"</n:props>
    <span class="inline-flex items-center px-3 py-1 rounded-full text-xs font-bold uppercase tracking-wider mb-4 {{ variant == 'primary' ? 'bg-indigo-500/20 text-indigo-300' : 'bg-gray-100 text-gray-800' }}">
        <n:slot />
    </span>
</n:component>`,
                language: 'html'
            }
        }
    },

    landing: {
        ncl: `<n:view title="Landing Page">
    <div class="bg-slate-900 text-slate-200 min-h-screen font-sans">
        <!-- Hero Section -->
        <header class="container mx-auto px-6 py-20 text-center">
            <Badge variant="primary">v3.0 Now Available</Badge>
            
            <h1 class="text-5xl md:text-7xl font-bold my-8 tracking-tight text-white">
                Build faster with <span class="text-indigo-500">Nucleus</span>.
            </h1>
            
            <p class="text-xl text-slate-400 mb-10 max-w-2xl mx-auto">
                The full-stack framework for Rust that feels like magic. 
                Zero-setup, edge-ready, and blazingly fast.
            </p>
            
            <div class="flex justify-center gap-4">
                <Button variant="primary" size="lg">Start Building</Button>
                <Button variant="secondary" size="lg" href="/docs">Read Docs</Button>
            </div>
        </header>

        <!-- Features Grid using FeatureCard components -->
        <section class="container mx-auto px-6 py-20 grid md:grid-cols-3 gap-8">
            <FeatureCard 
                icon="🚀" 
                title="Blazing Fast" 
                description="Compiles to native code. Zero runtime overhead. 83,000+ requests per second." 
            />
            <FeatureCard 
                icon="🛡️" 
                title="Type Safe" 
                description="Catch bugs at compile time. Rust's safety guarantees built-in." 
            />
            <FeatureCard 
                icon="🔋" 
                title="Batteries Included" 
                description="Auth, ORM, Queues, AI, and more. Everything to ship a complete product." 
            />
        </section>
    </div>
</n:view>`,
        css: ``,
        files: {
            'components/FeatureCard.ncl': {
                content: `<n:component name="FeatureCard">
    <n:props>
        icon: String
        title: String
        description: String
    </n:props>
    <div class="p-6 bg-slate-800 rounded-xl border border-slate-700 hover:border-indigo-500 transition-colors">
        <div class="text-4xl mb-4">{{ icon }}</div>
        <h3 class="text-xl font-bold mb-2 text-white">{{ title }}</h3>
        <p class="text-slate-400">
            {{ description }}
        </p>
    </div>
</n:component>`,
                language: 'html'
            },
            'components/Button.ncl': {
                content: `<n:component name="Button">
    <n:props>
        variant: String = "primary"
        size: String = "md"
        href: String = ""
        type: String = "button"
        onclick: String = ""
        id: String = ""
    </n:props>
    <n:if condition="href">
        <a href="{{ href }}" id="{{ id }}" onclick="{{ onclick }}" class="inline-flex items-center justify-center px-6 py-3 rounded-lg font-bold text-center transition-all {{ variant == 'primary' ? 'bg-indigo-600 text-white hover:bg-indigo-500 hover:-translate-y-1 shadow-lg shadow-indigo-500/30' : 'bg-slate-800 text-white border border-slate-700 hover:bg-slate-700' }}">
            <n:slot />
        </a>
    </n:if>
    <n:if condition="!href">
        <button type="{{ type }}" id="{{ id }}" onclick="{{ onclick }}" class="inline-flex items-center justify-center px-6 py-3 rounded-lg font-bold text-center transition-all {{ variant == 'primary' ? 'bg-indigo-600 text-white hover:bg-indigo-500 hover:-translate-y-1 shadow-lg shadow-indigo-500/30' : 'bg-slate-800 text-white border border-slate-700 hover:bg-slate-700' }}">
            <n:slot />
        </button>
    </n:if>
</n:component>`,
                language: 'html'
            },
            'components/Badge.ncl': {
                content: `<n:component name="Badge">
    <n:props>variant: String = "default"</n:props>
    <span class="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-indigo-900/50 text-indigo-300 border border-indigo-500/30">
        <span class="w-2 h-2 rounded-full bg-indigo-400 mr-2 animate-pulse"></span>
        <n:slot />
    </span>
</n:component>`,
                language: 'html'
            }
        }
    },

    // --- COMPONENTS ---
    card: {
        ncl: `<n:view title="Card Component">
    <!-- Component Definition -->
    <n:component name="Card">
        <n:props>
            variant: String = "default"
            glass: String = "false"
        </n:props>
        
        <div class="card card-{{ variant }}">
            <n:slot />
        </div>
        
        <style scoped>
            .card {
                background: var(--bg-card, #fff);
                border: 1px solid var(--border, #e5e7eb);
                border-radius: 16px;
                padding: 24px;
                transition: all 0.3s ease;
            }
            .card:hover { 
                transform: translateY(-4px);
                box-shadow: 0 12px 40px rgba(0,0,0,0.1);
            }
            .card-glass {
                background: rgba(0,0,0,0.8);
                backdrop-filter: blur(10px);
                border-color: rgba(255,255,255,0.1);
            }
            .card-feature {
                background: linear-gradient(135deg, #6366f1, #a855f7);
                color: white;
                border: none;
            }
        </style>
    </n:component>

    <!-- Usage Examples -->
    <main class="p-10 bg-gray-100 min-h-screen font-sans">
        <h1 class="text-2xl font-bold mb-8 text-center">Card Component Variants</h1>
        
        <div class="grid gap-8 md:grid-cols-3 max-w-5xl mx-auto">
            <!-- Default Card -->
            <Card>
                <h3 class="font-bold text-lg mb-2">Default Card</h3>
                <p class="text-gray-600">Basic card with clean styling and hover effect.</p>
            </Card>
            
            <!-- Glass Card -->
            <Card variant="glass">
                <h3 class="font-bold text-lg mb-2 text-white">Glass Card</h3>
                <p class="text-gray-300">Frosted glass effect with backdrop blur.</p>
            </Card>
            
            <!-- Feature Card -->
            <Card variant="feature">
                <h3 class="font-bold text-lg mb-2">Feature Card</h3>
                <p class="text-white/80">Gradient background for highlighting features.</p>
            </Card>
        </div>
    </main>
</n:view>`,
        css: ``
    },
    
    button: {
        ncl: `<n:view title="Button System">
    <!-- Button Component Definition -->
    <n:component name="Button">
        <n:props>
            variant: String = "primary"
            size: String = "md"
            href: String = ""
        </n:props>
        
        {% if href %}
        <a href="{{ href }}" class="btn btn-{{ variant }} btn-{{ size }}">
            <n:slot />
        </a>
        {% endif %}
        
        {% if !href %}
        <button class="btn btn-{{ variant }} btn-{{ size }}">
            <n:slot />
        </button>
        {% endif %}
        
        <style scoped>
            .btn {
                display: inline-flex;
                align-items: center;
                gap: 8px;
                font-weight: 600;
                border-radius: 8px;
                cursor: pointer;
                transition: all 0.2s ease;
                text-decoration: none;
            }
            .btn-sm { padding: 8px 16px; font-size: 12px; }
            .btn-md { padding: 12px 24px; font-size: 14px; }
            .btn-lg { padding: 16px 32px; font-size: 16px; }
            
            .btn-primary { 
                background: linear-gradient(135deg, #6366f1, #8b5cf6);
                color: white; 
                border: none;
            }
            .btn-primary:hover { 
                transform: translateY(-2px);
                box-shadow: 0 8px 30px rgba(99,102,241,0.4);
            }
            .btn-secondary { 
                background: white;
                border: 1px solid #e5e7eb;
                color: #374151;
            }
            .btn-secondary:hover { background: #f9fafb; }
            .btn-ghost { 
                background: transparent;
                border: none;
                color: #6b7280;
            }
            .btn-ghost:hover { background: #f3f4f6; }
        </style>
    </n:component>

    <!-- Usage Examples -->
    <main class="p-10 min-h-screen bg-slate-50 flex flex-col gap-12 items-center justify-center font-sans">
        <section class="text-center">
            <h2 class="text-lg font-bold text-slate-700 mb-4">Button Variants</h2>
            <div class="flex gap-4 items-center">
                <Button variant="primary">Primary</Button>
                <Button variant="secondary">Secondary</Button>
                <Button variant="ghost">Ghost</Button>
            </div>
        </section>
        
        <section class="text-center">
            <h2 class="text-lg font-bold text-slate-700 mb-4">Button Sizes</h2>
            <div class="flex gap-4 items-center">
                <Button size="sm">Small</Button>
                <Button size="md">Medium</Button>
                <Button size="lg">Large</Button>
            </div>
        </section>
        
        <section class="text-center">
            <h2 class="text-lg font-bold text-slate-700 mb-4">With Icons</h2>
            <div class="flex gap-4 items-center">
                <Button variant="primary">🚀 Launch</Button>
                <Button variant="secondary">📥 Download</Button>
                <Button variant="ghost">⚙️ Settings</Button>
            </div>
        </section>
    </main>
</n:view>`,
        css: ``
    },

    // --- FORMS ---
    form: {
        ncl: `<n:view title="Contact Form">
    <!-- Form using Nucleus form components -->
    <main class="min-h-screen bg-gray-50 py-8 px-4 font-sans">
        <div class="max-w-3xl mx-auto">
            <div class="text-center mb-8">
                <Badge variant="primary">📝 Forms & Validation</Badge>
                <h1 class="text-3xl font-bold mt-4 text-gray-900">Complete Form Features</h1>
                <p class="text-gray-500 mt-2">Showcasing all form components, validation, and styling options.</p>
            </div>
            
            <Card>
                <n:form action="/api/submit" method="POST" validate="true">
                    <!-- SECTION 1: Input Variants & Sizes -->
                    <FormGroup legend="TextInput Variants" columns="3">
                        <TextInput 
                            name="input_default" 
                            label="Default" 
                            variant="default"
                            placeholder="Default style"
                        />
                        <TextInput 
                            name="input_filled" 
                            label="Filled" 
                            variant="filled"
                            placeholder="Filled style"
                        />
                        <TextInput 
                            name="input_underline" 
                            label="Underline" 
                            variant="underline"
                            placeholder="Underline style"
                        />
                    </FormGroup>
                    
                    <!-- Sizes -->
                    <FormGroup legend="Input Sizes" columns="3">
                        <TextInput name="size_sm" label="Small" size="small" placeholder="sm" />
                        <TextInput name="size_md" label="Medium" size="medium" placeholder="md" />
                        <TextInput name="size_lg" label="Large" size="large" placeholder="lg" />
                    </FormGroup>
                    
                    <!-- SECTION 2: Validation States -->
                    <FormGroup legend="Validation & States" columns="2">
                        <TextInput 
                            name="field_error" 
                            label="With Error" 
                            error="This field has an error"
                            value="invalid@"
                        />
                        <TextInput 
                            name="field_help" 
                            label="With Help Text" 
                            help="This is helpful information"
                            placeholder="Enter something..."
                        />
                    </FormGroup>
                    
                    <!-- Icon & Disabled -->
                    <FormGroup legend="Special Features" columns="2">
                        <TextInput 
                            name="with_icon" 
                            label="With Icon" 
                            icon="🔍"
                            placeholder="Search..."
                        />
                        <TextInput 
                            name="disabled" 
                            label="Disabled" 
                            disabled="true"
                            value="Cannot edit"
                        />
                    </FormGroup>
                    
                    <!-- SECTION 3: Input Types -->
                    <FormGroup legend="Input Types" columns="3">
                        <TextInput name="email" type="email" label="Email" placeholder="you@example.com" required="true" />
                        <TextInput name="password" type="password" label="Password" placeholder="••••••••" required="true" />
                        <TextInput name="number" type="number" label="Number" placeholder="123" />
                    </FormGroup>
                    
                    <FormGroup columns="2">
                        <TextInput name="tel" type="tel" label="Phone" placeholder="+1 (555) 123-4567" />
                        <TextInput name="url" type="url" label="Website" placeholder="https://example.com" />
                    </FormGroup>
                    
                    <!-- SECTION 4: Select Component -->
                    <FormGroup legend="Select Options" columns="2">
                        <Select name="country" label="Country" required="true">
                            <option value="">Select country...</option>
                            <option value="us">🇺🇸 United States</option>
                            <option value="uk">🇬🇧 United Kingdom</option>
                            <option value="ca">🇨🇦 Canada</option>
                            <option value="de">🇩🇪 Germany</option>
                        </Select>
                        <Select name="role" label="Role (with error)" error="Please select a role">
                            <option value="">Select role...</option>
                            <option value="admin">Administrator</option>
                            <option value="user">User</option>
                        </Select>
                    </FormGroup>
                    
                    <!-- SECTION 5: Conditional Fields -->
                    <FormGroup legend="Conditional Fields (depends_on)" columns="1">
                        <Select name="account_type" label="Account Type">
                            <option value="personal">Personal Account</option>
                            <option value="business">Business Account</option>
                        </Select>
                        <!-- This field shows only when account_type is "business" -->
                        <TextInput 
                            name="company_name" 
                            label="Company Name" 
                            depends_on="account_type:business"
                            help="Required for business accounts"
                            required="true"
                        />
                        <TextInput 
                            name="vat_number" 
                            label="VAT Number" 
                            depends_on="account_type:business"
                            placeholder="VAT12345678"
                        />
                    </FormGroup>
                    
                    <!-- SECTION 6: Checkboxes -->
                    <FormGroup legend="Checkbox Variants" columns="1">
                        <Checkbox name="terms" label="I agree to the Terms of Service" required="true" />
                        <Checkbox name="newsletter" label="Subscribe to newsletter" checked="true" />
                        <Checkbox name="notifications" label="Enable push notifications" variant="toggle" />
                        <Checkbox name="marketing" label="Receive marketing emails" variant="toggle" checked="true" />
                    </FormGroup>
                    
                    <!-- Submit -->
                    <div class="flex gap-4 justify-end mt-6 pt-6 border-t">
                        <Button variant="secondary">Cancel</Button>
                        <Button type="submit" variant="primary">Submit Form</Button>
                    </div>
                </n:form>
            </Card>
            
            <!-- Schema Reference -->
            <Card>
                <h3 class="text-lg font-bold mb-4">📋 Rust Schema Equivalent</h3>
                <pre class="bg-slate-800 text-gray-200 p-4 rounded-lg text-sm overflow-x-auto">
FormSchema::new("contact")
    .action("/api/submit")
    .field(Field::email("email").label("Email").required())
    .field(Field::password("password").min(8.0))
    .field(Field::text("company")
        .depends_on("account_type:business"))
    .field(Field::checkbox("terms").required())
    .submit("Submit Form")
                </pre>
            </Card>
        </div>
    </main>
</n:view>`,
        css: ``
    },

    wizard: {
        ncl: `<n:view title="Multi-Step Wizard">
    <!-- Wizard Form using n:step -->
    <main class="min-h-screen bg-slate-900 text-white flex items-center justify-center p-4 font-sans">
        <div class="w-full max-w-2xl">
            <!-- Progress Steps -->
            <div class="flex justify-between mb-8 relative">
                <div class="absolute top-5 left-0 w-full h-1 bg-slate-800 rounded"></div>
                <div class="flex flex-col items-center relative z-10">
                    <div class="w-10 h-10 rounded-full bg-indigo-600 flex items-center justify-center font-bold border-4 border-slate-900">1</div>
                    <span class="text-sm font-medium text-indigo-400 mt-2">Account</span>
                </div>
                <div class="flex flex-col items-center relative z-10">
                    <div class="w-10 h-10 rounded-full bg-slate-800 flex items-center justify-center font-bold border-4 border-slate-900">2</div>
                    <span class="text-sm font-medium text-slate-500 mt-2">Profile</span>
                </div>
                <div class="flex flex-col items-center relative z-10">
                    <div class="w-10 h-10 rounded-full bg-slate-800 flex items-center justify-center font-bold border-4 border-slate-900">3</div>
                    <span class="text-sm font-medium text-slate-500 mt-2">Confirm</span>
                </div>
            </div>

            <!-- Wizard Form with n:step -->
            <div class="bg-slate-800 p-8 rounded-2xl border border-slate-700">
                <n:form action="/api/register" method="POST" wizard="true">
                    <n:step id="account" title="Create Your Account">
                        <p class="text-slate-400 mb-6">Enter your credentials to get started.</p>
                        
                        <TextInput name="email" type="email" label="Email Address" required="true" />
                        <TextInput name="password" type="password" label="Password" help="At least 8 characters" required="true" />
                    </n:step>
                    
                    <n:step id="profile" title="Your Profile">
                        <FormGroup legend="Personal Information" columns="2">
                            <TextInput name="firstName" label="First Name" required="true" />
                            <TextInput name="lastName" label="Last Name" required="true" />
                        </FormGroup>
                        
                        <TextInput name="phone" type="tel" label="Phone Number" />
                    </n:step>
                    
                    <n:step id="confirm" title="Confirm & Submit">
                        <p class="text-slate-400 mb-4">Review your information and accept our terms.</p>
                        
                        <Checkbox name="terms" label="I agree to the Terms of Service" required="true" />
                        <Checkbox name="newsletter" label="Subscribe to newsletter" variant="toggle" />
                    </n:step>
                </n:form>
                
                <div class="mt-8 flex justify-end gap-3 pt-6 border-t border-slate-700">
                    <Button variant="ghost">Cancel</Button>
                    <Button variant="primary">Continue →</Button>
                </div>
            </div>
        </div>
    </main>
</n:view>`,
        css: ``,
        files: {
            'components/TextInput.ncl': {
                content: `<n:component name="TextInput">
    <n:props>
        name: String
        label: String
        type: String = "text"
        required: Boolean = false
        placeholder: String = ""
        help: String = ""
    </n:props>
    <div class="mb-4">
        <label for="{{ name }}" class="block text-sm font-medium text-slate-300 mb-1">
            {{ label }}
            <n:if condition="required"><span class="text-indigo-400">*</span></n:if>
        </label>
        <input 
            type="{{ type }}" 
            id="{{ name }}" 
            name="{{ name }}"
            class="w-full px-3 py-2 bg-slate-900 border border-slate-700 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500"
            placeholder="{{ placeholder }}"
            required="{{ required }}"
        />
        <n:if condition="help">
            <p class="mt-1 text-xs text-slate-500">{{ help }}</p>
        </n:if>
    </div>
</n:component>`,
                language: 'html'
            },
            'components/Checkbox.ncl': {
                content: `<n:component name="Checkbox">
    <n:props>
        name: String
        label: String
        required: Boolean = false
        variant: String = "default"
    </n:props>
    <div class="flex items-start mb-4">
        <div class="flex items-center h-5">
            <input 
                id="{{ name }}" 
                name="{{ name }}" 
                type="checkbox" 
                class="focus:ring-indigo-500 h-4 w-4 text-indigo-600 border-slate-700 rounded bg-slate-900"
                required="{{ required }}"
            />
        </div>
        <div class="ml-3 text-sm">
            <label for="{{ name }}" class="font-medium text-slate-300">{{ label }}</label>
        </div>
    </div>
</n:component>`,
                language: 'html'
            },
            'components/FormGroup.ncl': {
                content: `<n:component name="FormGroup">
    <n:props>
        legend: String
        columns: String = "1"
    </n:props>
    <fieldset class="mb-6 border border-slate-700 rounded-lg p-4">
        <legend class="text-sm font-medium px-2 text-slate-300">{{ legend }}</legend>
        <div class="grid grid-cols-{{ columns }} gap-4">
            <n:slot />
        </div>
    </fieldset>
</n:component>`,
                language: 'html'
            },
            'components/Button.ncl': {
                content: `<n:component name="Button">
    <n:props>
        variant: String = "primary"
        size: String = "md"
        href: String = ""
        type: String = "button"
        onclick: String = ""
        id: String = ""
    </n:props>
    <n:if condition="href">
        <a href="{{ href }}" id="{{ id }}" onclick="{{ onclick }}" class="px-4 py-2 rounded-lg font-medium transition-colors {{ variant == 'primary' ? 'bg-indigo-600 text-white hover:bg-indigo-700' : 'bg-transparent text-slate-300 hover:text-white border border-slate-700 hover:border-slate-500' }}">
            <n:slot />
        </a>
    </n:if>
    <n:if condition="!href">
        <button type="{{ type }}" id="{{ id }}" onclick="{{ onclick }}" class="px-4 py-2 rounded-lg font-medium transition-colors {{ variant == 'primary' ? 'bg-indigo-600 text-white hover:bg-indigo-700' : 'bg-transparent text-slate-300 hover:text-white border border-slate-700 hover:border-slate-500' }}">
            <n:slot />
        </button>
    </n:if>
</n:component>`,
                language: 'html'
            }
        }
    },

    // --- FULL APPS ---
    dashboard: {
        ncl: `<n:view title="Dashboard" layout="dashboard_layout">
    <!-- Dashboard using StatCard, NavItem, and layout patterns -->
    <div class="flex h-screen bg-gray-50 font-sans">
        <!-- Sidebar with NavItem components -->
        <aside class="w-64 bg-white border-r border-gray-200 hidden md:flex flex-col">
            <div class="p-6 flex items-center gap-3">
                <div class="w-8 h-8 bg-indigo-600 rounded-lg flex items-center justify-center text-white font-bold">N</div>
                <span class="font-bold text-gray-800 text-lg">Nucleus</span>
            </div>
            
            <nav class="flex-1 px-4 space-y-1">
                <NavItem href="#" icon="📊" active="true">Dashboard</NavItem>
                <NavItem href="#" icon="👥">Users</NavItem>
                <NavItem href="#" icon="📦">Products</NavItem>
                <NavItem href="#" icon="⚙️">Settings</NavItem>
            </nav>
        </aside>

        <!-- Main Content -->
        <main class="flex-1 overflow-auto p-8">
            <header class="flex justify-between items-center mb-8">
                <div>
                    <h1 class="text-2xl font-bold text-gray-900">Overview</h1>
                    <p class="text-gray-500 text-sm">Welcome back, Jane.</p>
                </div>
                <Button variant="primary">+ New Project</Button>
            </header>

            <!-- Stats Grid using StatCard components -->
            <div class="grid grid-cols-1 md:grid-cols-4 gap-6 mb-8">
                <StatCard value="$45,231" label="Total Revenue" trend="+20.1% vs last month" />
                <StatCard value="2,345" label="Active Users" trend="+15.2% vs last month" />
                <StatCard value="42.5%" label="Bounce Rate" trend="-4.1% vs last month" />
                <StatCard value="99.9%" label="Uptime" trend="Stable" />
            </div>

            <!-- Data Table -->
            <Card>
                <div class="flex justify-between items-center mb-6">
                    <h2 class="font-bold text-gray-900">Recent Transactions</h2>
                    <Button variant="ghost" size="sm">View All</Button>
                </div>
                <table class="w-full text-left">
                    <thead class="bg-gray-50 text-gray-500 text-xs uppercase">
                        <tr>
                            <th class="p-4">Customer</th>
                            <th class="p-4">Status</th>
                            <th class="p-4">Date</th>
                            <th class="p-4 text-right">Amount</th>
                        </tr>
                    </thead>
                    <tbody class="divide-y">
                        <tr class="hover:bg-gray-50">
                            <td class="p-4 font-medium">Apple Inc.</td>
                            <td class="p-4"><Badge variant="primary">Paid</Badge></td>
                            <td class="p-4 text-gray-500">Oct 24, 2025</td>
                            <td class="p-4 text-right font-medium">$4,200.00</td>
                        </tr>
                        <tr class="hover:bg-gray-50">
                            <td class="p-4 font-medium">Google LLC</td>
                            <td class="p-4"><Badge>Pending</Badge></td>
                            <td class="p-4 text-gray-500">Oct 23, 2025</td>
                            <td class="p-4 text-right font-medium">$2,500.00</td>
                        </tr>
                    </tbody>
                </table>
            </Card>
            
            <!-- Island for interactive chart -->
            <div class="mt-8">
                <n:island src="components/RevenueChart.ncl" client:visible />
            </div>
        </main>
    </div>
</n:view>`,
        css: ``,
        files: {
            'components/RevenueChart.ncl': {
                content: `<n:component name="RevenueChart">
    <!-- Interactive Island Component -->
    <div class="bg-white p-6 rounded-xl border border-gray-100 shadow-sm">
        <div class="flex justify-between items-center mb-6">
            <h3 class="font-bold text-gray-900">Revenue Overview</h3>
            <select class="text-sm border rounded-md px-2 py-1 bg-gray-50">
                <option>This Year</option>
                <option>Last Year</option>
            </select>
        </div>
        
        <!-- Mock Chart Visualization -->
        <div class="h-64 flex items-end justify-between gap-2 px-2">
            <div class="w-full bg-indigo-100 rounded-t-sm relative group h-12">
                <div class="absolute bottom-full left-1/2 -translate-x-1/2 mb-2 bg-gray-900 text-white text-xs px-2 py-1 rounded opacity-0 group-hover:opacity-100 transition">Jan</div>
            </div>
            <div class="w-full bg-indigo-200 rounded-t-sm relative group h-20"></div>
            <div class="w-full bg-indigo-300 rounded-t-sm relative group h-32"></div>
            <div class="w-full bg-indigo-400 rounded-t-sm relative group h-40"></div>
            <div class="w-full bg-indigo-500 rounded-t-sm relative group h-36"></div>
            <div class="w-full bg-indigo-600 rounded-t-sm relative group h-48">
                <div class="absolute bottom-full left-1/2 -translate-x-1/2 mb-2 bg-gray-900 text-white text-xs px-2 py-1 rounded opacity-0 group-hover:opacity-100 transition">Jun</div>
            </div>
            <div class="w-full bg-indigo-500 rounded-t-sm relative group h-56"></div>
            <div class="w-full bg-indigo-400 rounded-t-sm relative group h-44"></div>
            <div class="w-full bg-indigo-300 rounded-t-sm relative group h-32"></div>
            <div class="w-full bg-indigo-200 rounded-t-sm relative group h-24"></div>
            <div class="w-full bg-indigo-100 rounded-t-sm relative group h-16"></div>
            <div class="w-full bg-indigo-50 rounded-t-sm relative group h-20"></div>
        </div>
    </div>
</n:component>`,
                language: 'html'
            },
            'components/NavItem.ncl': {
                content: `<n:component name="NavItem">
    <n:props>
        href: String
        icon: String
        active: Boolean = false
    </n:props>
    <a href="{{ href }}" class="flex items-center gap-3 px-3 py-2 rounded-lg mb-1 transition-colors {{ active ? 'bg-indigo-50 text-indigo-600' : 'text-gray-600 hover:bg-gray-50' }}">
        <span>{{ icon }}</span>
        <span class="font-medium">{{ slot }}</span>
    </a>
</n:component>`,
                language: 'html'
            },
            'components/StatCard.ncl': {
                content: `<n:component name="StatCard">
    <n:props>
        label: String
        value: String
        trend: String = ""
    </n:props>
    <div class="bg-white p-6 rounded-xl border border-gray-100 shadow-sm">
        <div class="text-sm text-gray-500 mb-1">{{ label }}</div>
        <div class="text-2xl font-bold text-gray-900">{{ value }}</div>
        <n:if condition="trend">
            <div class="text-xs mt-2 text-green-600 font-medium">{{ trend }}</div>
        </n:if>
    </div>
</n:component>`,
                language: 'html'
            },
            'components/Card.ncl': {
                content: `<n:component name="Card">
    <div class="bg-white p-6 rounded-xl border border-gray-100 shadow-sm text-sm">
        <n:slot />
    </div>
</n:component>`,
                language: 'html'
            },
            'components/Button.ncl': {
                content: `<n:component name="Button">
    <n:props>
        variant: String = "primary"
        size: String = "md"
        href: String = ""
        type: String = "button"
        onclick: String = ""
        id: String = ""
    </n:props>
    <n:if condition="href">
        <a href="{{ href }}" id="{{ id }}" onclick="{{ onclick }}" class="px-4 py-2 rounded-lg font-medium transition-colors {{ variant == 'primary' ? 'bg-indigo-600 text-white hover:bg-indigo-700' : 'bg-transparent text-indigo-600 hover:bg-indigo-50' }}">
            <n:slot />
        </a>
    </n:if>
    <n:if condition="!href">
        <button type="{{ type }}" id="{{ id }}" onclick="{{ onclick }}" class="px-4 py-2 rounded-lg font-medium transition-colors {{ variant == 'primary' ? 'bg-indigo-600 text-white hover:bg-indigo-700' : 'bg-transparent text-indigo-600 hover:bg-indigo-50' }}">
            <n:slot />
        </button>
    </n:if>
</n:component>`,
                language: 'html'
            },
            'components/Badge.ncl': {
                content: `<n:component name="Badge">
    <n:props>variant: String = "default"</n:props>
    <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium {{ variant == 'primary' ? 'bg-green-100 text-green-800' : 'bg-yellow-100 text-yellow-800' }}">
        <n:slot />
    </span>
</n:component>`,
                language: 'html'
            }
        }
    },
    
    auth: {
        ncl: `<n:view title="Login" guard="guest_only">
    <!-- Auth page with layout pattern -->
    <div class="min-h-screen grid md:grid-cols-2 font-sans">
        <!-- Brand Section -->
        <div class="hidden md:flex flex-col bg-slate-900 text-white p-12 justify-between">
            <div class="flex items-center gap-2">
                <div class="w-10 h-10 bg-indigo-600 rounded-xl flex items-center justify-center font-bold text-lg">N</div>
                <span class="font-bold text-xl">Nucleus</span>
            </div>
            
            <div>
                <h2 class="text-4xl font-bold mb-4">"The fastest way to build modern apps."</h2>
                <p class="text-slate-400">— Developer testimonial</p>
            </div>
            
            <p class="text-sm text-slate-500">© 2025 Nucleus Framework</p>
        </div>
        
        <!-- Login Form -->
        <div class="flex items-center justify-center p-8 bg-white">
            <div class="w-full max-w-md">
                <h1 class="text-3xl font-bold text-gray-900 mb-2">Welcome back</h1>
                <p class="text-gray-500 mb-8">Enter your credentials to access your account.</p>
                
                <n:form action="/auth/login" method="POST">
                    <TextInput 
                        name="email" 
                        type="email" 
                        label="Email" 
                        placeholder="you@company.com"
                        required="true"
                    />
                    
                    <TextInput 
                        name="password" 
                        type="password" 
                        label="Password" 
                        required="true"
                    />
                    
                    <div class="flex items-center justify-between mb-6">
                        <Checkbox name="remember" label="Remember me" />
                        <n:link href="/forgot-password">Forgot password?</n:link>
                    </div>
                    
                    <Button type="submit" variant="primary">Sign In</Button>
                </n:form>
                
                <p class="mt-8 text-center text-gray-600">
                    Don't have an account? 
                    <n:link href="/register">Create one</n:link>
                </p>
            </div>
        </div>
    </div>
</n:view>`,
        css: ``,
        files: {
            'components/TextInput.ncl': {
                content: `<n:component name="TextInput">
    <n:props>
        name: String
        label: String
        type: String = "text"
        required: Boolean = false
        placeholder: String = ""
        help: String = ""
    </n:props>
    <div class="mb-4">
        <label for="{{ name }}" class="block text-sm font-medium text-gray-700 mb-1">
            {{ label }}
            <n:if condition="required"><span class="text-red-500">*</span></n:if>
        </label>
        <input 
            type="{{ type }}" 
            id="{{ name }}" 
            name="{{ name }}"
            class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500"
            placeholder="{{ placeholder }}"
            required="{{ required }}"
        />
        <n:if condition="help">
            <p class="mt-1 text-xs text-gray-500">{{ help }}</p>
        </n:if>
    </div>
</n:component>`,
                language: 'html'
            },
            'components/Checkbox.ncl': {
                content: `<n:component name="Checkbox">
    <n:props>
        name: String
        label: String
        required: Boolean = false
    </n:props>
    <div class="flex items-start mb-4">
        <div class="flex items-center h-5">
            <input 
                id="{{ name }}" 
                name="{{ name }}" 
                type="checkbox" 
                class="focus:ring-indigo-500 h-4 w-4 text-indigo-600 border-gray-300 rounded"
                required="{{ required }}"
            />
        </div>
        <div class="ml-3 text-sm">
            <label for="{{ name }}" class="font-medium text-gray-700">{{ label }}</label>
        </div>
    </div>
</n:component>`,
                language: 'html'
            },
            'components/Button.ncl': {
                content: `<n:component name="Button">
    <n:props>
        variant: String = "primary"
        size: String = "md"
        href: String = ""
        type: String = "button"
        onclick: String = ""
        id: String = ""
    </n:props>
    <n:if condition="href">
        <a href="{{ href }}" id="{{ id }}" onclick="{{ onclick }}" class="w-full px-4 py-2 rounded-lg font-medium text-white transition-colors bg-indigo-600 hover:bg-indigo-700 shadow-sm">
            <n:slot />
        </a>
    </n:if>
    <n:if condition="!href">
        <button type="{{ type }}" id="{{ id }}" onclick="{{ onclick }}" class="w-full px-4 py-2 rounded-lg font-medium text-white transition-colors bg-indigo-600 hover:bg-indigo-700 shadow-sm">
            <n:slot />
        </button>
    </n:if>
</n:component>`,
                language: 'html'
            }
        }
    },

    // --- ADVANCED PATTERNS ---
    imports: {
        ncl: `<n:view title="Component Imports">
    <!-- 
        Nucleus supports two ways to use components:
        
        1. AUTO-DISCOVERY: Components in /components are automatically available
           <Button />  ← Works if components/Button.ncl exists
        
        2. EXPLICIT IMPORT: Use n:include for clarity or external components
    -->
    
    <!-- Explicit imports with n:include -->
    <n:include src="components/Button.ncl" />
    <n:include src="components/Card.ncl" />
    <n:include src="components/Badge.ncl" />
    
    <main class="min-h-screen bg-slate-900 text-white p-8 font-sans">
        <div class="max-w-3xl mx-auto">
            <Badge variant="primary">📦 Imports Demo</Badge>
            <h1 class="text-4xl font-bold my-6">Component System</h1>
            
            <Card>
                <h2 class="text-xl font-bold mb-4">Auto-Discovery</h2>
                <p class="text-gray-400 mb-4">
                    Components placed in <code class="bg-slate-700 px-2 py-1 rounded">/components</code> 
                    are automatically available everywhere.
                </p>
                <pre class="bg-slate-800 p-4 rounded-lg text-sm overflow-x-auto">
src/
├── components/
│   ├── Button.ncl    → &lt;Button /&gt;
│   ├── Card.ncl      → &lt;Card /&gt;
│   └── ui/
│       └── Modal.ncl → &lt;Modal /&gt;
└── views/
    └── index.ncl     → Uses all components
                </pre>
            </Card>
            
            <Card>
                <h2 class="text-xl font-bold mb-4">Explicit Imports</h2>
                <p class="text-gray-400 mb-4">
                    Use <code class="bg-slate-700 px-2 py-1 rounded">&lt;n:include&gt;</code> 
                    for external or third-party components.
                </p>
                <pre class="bg-slate-800 p-4 rounded-lg text-sm overflow-x-auto">
&lt;n:include src="components/Button.ncl" /&gt;
&lt;n:include src="@nucleus/ui/Alert.ncl" /&gt;

&lt;!-- With props --&gt;
&lt;n:include src="./UserCard.ncl" name="Alice" /&gt;
                </pre>
            </Card>
            
            <div class="mt-8 flex gap-4">
                <Button variant="primary">Imported Button</Button>
                <Button variant="secondary">Also Imported</Button>
            </div>
        </div>
    </main>
</n:view>`,
        css: ``,
        files: {
            'components/Button.ncl': {
                content: `<n:component name="Button">
    <n:props>
        variant: String = "primary"
        size: String = "md"
        href: String = ""
        type: String = "button"
        onclick: String = ""
        id: String = ""
    </n:props>
    <n:if condition="href">
        <a href="{{ href }}" id="{{ id }}" onclick="{{ onclick }}" class="btn btn-{{ variant }} btn-{{ size }}">
            <n:slot />
        </a>
    </n:if>
    <n:if condition="!href">
        <button type="{{ type }}" id="{{ id }}" onclick="{{ onclick }}" class="btn btn-{{ variant }} btn-{{ size }}">
            <n:slot />
        </button>
    </n:if>
    <style scoped>
        .btn { padding: 0.5rem 1rem; border-radius: 0.5rem; font-weight: 500; cursor: pointer; border: none; }
        .btn-primary { background: #6366f1; color: white; }
        .btn-secondary { background: white; border: 1px solid #e5e7eb; color: #374151; }
    </style>
</n:component>`,
                language: 'html'
            },
            'components/Card.ncl': {
                content: `<n:component name="Card">
    <div class="bg-white p-6 rounded-xl border border-gray-100 shadow-sm">
        <n:slot />
    </div>
</n:component>`,
                language: 'html'
            },
            'components/Badge.ncl': {
                content: `<n:component name="Badge">
    <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-indigo-100 text-indigo-800">
        <n:slot />
    </span>
</n:component>`,
                language: 'html'
            },
            'UserCard.ncl': {
                content: `<n:component name="UserCard">
    <n:props>name: String</n:props>
    <div class="flex items-center gap-3 p-4 border rounded-lg">
        <div class="w-10 h-10 bg-gray-200 rounded-full"></div>
        <div class="font-bold">{{ name }}</div>
    </div>
</n:component>`,
                language: 'html'
            }
        }
    },

    data: {
        ncl: `<n:view title="Data Binding">
    <!-- 
        n:model binds server-side data to your template.
        This runs at SSR time and makes data available for rendering.
    -->
    
    <!-- Load data from database or API -->
    <n:model 
        users="db::get_users().await"
        stats="analytics::get_dashboard_stats()"
        posts="blog::get_recent_posts(5).await"
    />
    
    <main class="min-h-screen bg-gray-50 p-8 font-sans">
        <div class="max-w-4xl mx-auto">
            <Badge variant="primary">📊 Data Binding</Badge>
            <h1 class="text-4xl font-bold my-6 text-gray-900">Server Data</h1>
            
            <!-- Display stats using bound data -->
            <div class="grid md:grid-cols-3 gap-6 mb-8">
                <StatCard value="{{ stats.total_users }}" label="Total Users" trend="+12.5%" />
                <StatCard value="{{ stats.revenue }}" label="Revenue" trend="+8.3%" />
                <StatCard value="{{ stats.orders }}" label="Orders" trend="+22.1%" />
            </div>
            
            <!-- Iterate over users with n:for -->
            <Card>
                <h2 class="text-xl font-bold mb-4">User List</h2>
                
                {% for user in users %}
                    <div class="flex items-center justify-between p-4 border-b last:border-0">
                        <div class="flex items-center gap-3">
                            <n:image src="{{ user.avatar }}" alt="{{ user.name }}" width="40" height="40" />
                            <div>
                                <div class="font-medium">{{ user.name }}</div>
                                <div class="text-sm text-gray-500">{{ user.email }}</div>
                            </div>
                        </div>
                        <Badge>{{ user.role }}</Badge>
                    </div>
                {% endfor %}
                
                <!-- Empty state -->
                {% if users.len() == 0 %}
                    <p class="text-center text-gray-500 py-8">No users found</p>
                {% endif %}
            </Card>
            
            <!-- Recent posts -->
            <Card>
                <h2 class="text-xl font-bold mb-4">Recent Posts</h2>
                {% for post in posts %}
                    <article class="mb-4 pb-4 border-b last:border-0">
                        <n:link href="/blog/{{ post.slug }}">
                            <h3 class="text-lg font-semibold hover:text-indigo-600">{{ post.title }}</h3>
                        </n:link>
                        <p class="text-gray-600 text-sm mt-1">{{ post.excerpt }}</p>
                    </article>
                {% endfor %}
            </Card>
        </div>
    </main>
</n:view>`,
        css: ``,
        files: {
            'components/StatCard.ncl': {
                content: `<n:component name="StatCard">
    <n:props>
        label: String
        value: String
        trend: String = ""
    </n:props>
    <div class="bg-white p-6 rounded-xl border border-gray-100 shadow-sm">
        <div class="text-sm text-gray-500 mb-1">{{ label }}</div>
        <div class="text-2xl font-bold text-gray-900">{{ value }}</div>
        <n:if condition="trend">
            <div class="text-xs mt-2 text-green-600 font-medium">{{ trend }}</div>
        </n:if>
    </div>
</n:component>`,
                language: 'html'
            },
            'components/Card.ncl': {
                content: `<n:component name="Card">
    <div class="bg-white p-6 rounded-xl border border-gray-100 shadow-sm overflow-hidden">
        <n:slot />
    </div>
</n:component>`,
                language: 'html'
            },
            'components/Badge.ncl': {
                content: `<n:component name="Badge">
    <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-gray-100 text-gray-800">
        <n:slot />
    </span>
</n:component>`,
                language: 'html'
            },
            'components/Button.ncl': {
                content: `<n:component name="Button">
    <n:props>
        variant: String = "primary"
        size: String = "md"
        href: String = ""
        type: String = "button"
        onclick: String = ""
        id: String = ""
    </n:props>
    <n:if condition="href">
        <a href="{{ href }}" id="{{ id }}" onclick="{{ onclick }}" class="px-4 py-2 rounded-lg font-medium transition-all active:scale-95 {{ variant == 'primary' ? 'bg-indigo-600 text-white hover:bg-indigo-500' : (variant == 'ghost' ? 'text-slate-400 hover:text-white hover:bg-slate-800' : 'bg-slate-800 text-white border border-slate-700 hover:bg-slate-700') }} {{ size == 'sm' ? 'text-sm px-3 py-1' : '' }}">
            <n:slot />
        </a>
    </n:if>
    <n:if condition="!href">
        <button type="{{ type }}" id="{{ id }}" onclick="{{ onclick }}" class="px-4 py-2 rounded-lg font-medium transition-all active:scale-95 {{ variant == 'primary' ? 'bg-indigo-600 text-white hover:bg-indigo-500' : (variant == 'ghost' ? 'text-slate-400 hover:text-white hover:bg-slate-800' : 'bg-slate-800 text-white border border-slate-700 hover:bg-slate-700') }} {{ size == 'sm' ? 'text-sm px-3 py-1' : '' }}">
            <n:slot />
        </button>
    </n:if>
</n:component>`,
                language: 'html'
            }
        }
    },



    counter: {
        ncl: `<n:view title="Interactive Counter">
    <!-- 
        V3 Neutron Island with Signals.
        State is reactive and runs in WebAssembly.
    -->
    
    <main class="min-h-screen bg-slate-900 flex items-center justify-center font-sans">
        <Card variant="glass">
            <div class="text-center">
                <Badge variant="primary">⚡ Neutron V3 Island</Badge>
                
                <n:island client:load>
                    <n:script>
                        let count = Signal::new(0);
                    </n:script>

                    <h1 class="text-6xl font-bold my-8 text-white tabular-nums">
                        {count}
                    </h1>
                    
                    <p class="text-slate-400 mb-8">
                        Interactive WASM Island with Signals
                    </p>
                    
                    <div class="flex gap-4 justify-center">
                        <button onclick={count.update(|c| *c -= 1)} class="px-6 py-3 rounded-lg font-bold bg-white text-slate-900 hover:bg-slate-200 transition">
                            − Decrease
                        </button>
                        <button onclick={count.update(|c| *c += 1)} class="px-6 py-3 rounded-lg font-bold bg-indigo-600 text-white hover:bg-indigo-500 transition shadow-lg shadow-indigo-500/30">
                            + Increase
                        </button>
                    </div>
                </n:island>
            </div>
        </Card>
    </main>
</n:view>`,
        css: ``,
        files: {
            'components/Button.ncl': {
                content: `<n:component name="Button">
    <n:props>
        variant: String = "primary"
        size: String = "md"
        href: String = ""
        type: String = "button"
        onclick: String = ""
        id: String = ""
    </n:props>
    <!-- Render as anchor if href is present, otherwise button -->
    <n:if condition="href">
        <a href="{{ href }}" id="{{ id }}" onclick="{{ onclick }}" class="btn btn-{{ variant }} btn-{{ size }}">
            <n:slot />
        </a>
    </n:if>
    <n:if condition="!href">
        <button type="{{ type }}" id="{{ id }}" onclick="{{ onclick }}" class="btn btn-{{ variant }} btn-{{ size }}">
            <n:slot />
        </button>
    </n:if>
</n:component>`,
                language: 'html'
            },
            'components/Card.ncl': {
                content: `<n:component name="Card">
    <n:props>
        variant: String = "default"
    </n:props>
    <div class="p-8 rounded-2xl border {{ variant == 'glass' ? 'bg-slate-800/50 backdrop-blur border-slate-700' : 'bg-white border-gray-100' }}">
        <n:slot />
    </div>
</n:component>`,
                language: 'html'
            },
            'components/Badge.ncl': {
                content: `<n:component name="Badge">
    <n:props>variant: String = "default"</n:props>
    <span class="inline-flex items-center px-3 py-1 rounded-full text-xs font-bold uppercase tracking-wider mb-4 {{ variant == 'primary' ? 'bg-indigo-500/20 text-indigo-300' : 'bg-gray-100 text-gray-800' }}">
        <n:slot />
    </span>
</n:component>`,
                language: 'html'
            }
        } 
    },

    todo: {
        ncl: `<n:view title="Todo App">
    <!-- Full CRUD example with n:model and n:form -->
    
    <!-- Load todos from database -->
    <n:model todos="db::get_todos(user.id).await" />
    
    <main class="min-h-screen bg-gray-50 py-12 px-4 font-sans flex items-center justify-center">
        <div class="w-full max-w-md">
            <div class="text-center mb-8">
                <div class="inline-flex items-center justify-center w-12 h-12 rounded-xl bg-indigo-100 text-indigo-600 mb-4 shadow-sm">
                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-6 9l2 2 4-4"></path></svg>
                </div>
                <h1 class="text-3xl font-bold text-gray-900 tracking-tight">Tasks</h1>
                <p class="text-gray-500 mt-2">Manage your daily goals</p>
            </div>
            
            <Card class="shadow-xl bg-white/80 backdrop-blur-lg border-0 ring-1 ring-gray-200">
                <!-- Add new todo form -->
                <n:form action="handlers::add_todo" method="POST" class="border-b border-gray-100 p-2 mb-2">
                    <div class="flex items-center gap-2">
                        <TextInput 
                            name="title" 
                            placeholder="Add a new task..." 
                            required="true"
                            class="flex-1 border-0 bg-transparent focus:ring-0 text-gray-800 placeholder-gray-400 text-lg py-3 px-2"
                        />
                        <Button type="submit" variant="primary" class="rounded-full w-10 h-10 flex items-center justify-center p-0 !min-w-0 shadow-md hover:shadow-lg transition-all active:scale-95 bg-indigo-600 hover:bg-indigo-500">
                            <span class="text-xl leading-none mb-0.5">+</span>
                        </Button>
                    </div>
                </n:form>
                
                <!-- Todo list -->
                <div class="divide-y divide-gray-50 max-h-[400px] overflow-y-auto custom-scrollbar">
                    {% for todo in todos %}
                        <div class="group flex items-center gap-3 py-4 px-4 hover:bg-gray-50/50 transition-colors">
                            <!-- Toggle Form -->
                            <n:form action="handlers::toggle_todo" method="POST">
                                <input type="hidden" name="id" value="{{ todo.id }}" />
                                <button type="submit" class="w-6 h-6 rounded-full border-2 flex items-center justify-center transition-all duration-200 {{ todo.completed ? 'bg-green-500 border-green-500' : 'border-gray-300 hover:border-indigo-400' }}">
                                    <n:if condition="todo.completed">
                                        <svg class="w-3.5 h-3.5 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M5 13l4 4L19 7"></path></svg>
                                    </n:if>
                                </button>
                            </n:form>
                            
                            <span class="flex-1 text-base transition-all {{ todo.completed ? 'text-gray-400 line-through' : 'text-gray-700 font-medium' }}">
                                {{ todo.title }}
                            </span>
                            
                            <!-- Delete Form -->
                            <n:form action="handlers::delete_todo" method="DELETE">
                                <input type="hidden" name="id" value="{{ todo.id }}" />
                                <button type="submit" class="opacity-0 group-hover:opacity-100 p-2 text-gray-400 hover:text-red-500 transition-all rounded-lg hover:bg-red-50" title="Delete task">
                                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"></path></svg>
                                </button>
                            </n:form>
                        </div>
                    {% endfor %}
                </div>
                
                <!-- Footer stats -->
                <div class="bg-gray-50/50 p-4 border-t border-gray-100 text-xs font-medium text-gray-500 flex justify-between items-center rounded-b-xl">
                    <span>{{ todos.len() }} tasks remaining</span>
                    <n:link href="/clear-completed" class="text-indigo-600 hover:text-indigo-700 hover:underline">Clear completed</n:link>
                </div>
            </Card>
        </div>
    </main>
    
    <style>
        .custom-scrollbar::-webkit-scrollbar { width: 4px; }
        .custom-scrollbar::-webkit-scrollbar-thumb { background: #e2e8f0; border-radius: 4px; }
        .custom-scrollbar::-webkit-scrollbar-track { background: transparent; }
    </style>
</n:view>`,
        css: ``,
        files: {
            'components/Button.ncl': {
                content: `<n:component name="Button">
    <n:props>
        variant: String = "primary"
        size: String = "md"
        href: String = ""
        type: String = "button"
        onclick: String = ""
        id: String = ""
    </n:props>
    <n:if condition="href">
        <a href="{{ href }}" id="{{ id }}" onclick="{{ onclick }}" class="px-4 py-2 rounded-lg font-medium transition-colors inline-block {{ variant == 'primary' ? 'bg-indigo-600 text-white hover:bg-indigo-700' : 'bg-white text-gray-900 border border-gray-200 hover:bg-gray-50' }}">
            <n:slot />
        </a>
    </n:if>
    <n:if condition="!href">
        <button type="{{ type }}" id="{{ id }}" onclick="{{ onclick }}" class="px-4 py-2 rounded-lg font-medium transition-colors {{ variant == 'primary' ? 'bg-indigo-600 text-white hover:bg-indigo-700' : 'bg-white text-gray-900 border border-gray-200 hover:bg-gray-50' }}">
            <n:slot />
        </button>
    </n:if>
</n:component>`,
                language: 'html'
            },
            'components/Card.ncl': {
                content: `<n:component name="Card">
    <div class="bg-white p-6 rounded-xl border border-gray-100 shadow-sm overflow-hidden">
        <n:slot />
    </div>
</n:component>`,
                language: 'html'
            },
            'components/Badge.ncl': {
                content: `<n:component name="Badge">
    <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-gray-100 text-gray-800">
        <n:slot />
    </span>
</n:component>`,
                language: 'html'
            }
        }
    },

    blog: {
        ncl: `<n:view title="Blog" layout="main_layout">
    <!-- 
        Layouts wrap your views with shared structure.
        Use layout="name" to apply a layout from /layouts
    -->
    
    <!-- Load blog data -->
    <n:model 
        posts="blog::get_posts().await"
        featured="blog::get_featured_post().await"
    />
    
    <!-- Featured post hero -->
    <section class="bg-gradient-to-br from-indigo-600 to-purple-700 text-white py-20 px-6">
        <div class="max-w-4xl mx-auto">
            <Badge>Featured</Badge>
            <h1 class="text-5xl font-bold mt-4 mb-6">{{ featured.title }}</h1>
            <p class="text-xl text-white/80 mb-8">{{ featured.excerpt }}</p>
            <Button variant="secondary" href="/blog/{{ featured.slug }}">
                Read Article →
            </Button>
        </div>
    </section>
    
    <!-- Post grid -->
    <section class="max-w-6xl mx-auto py-16 px-6">
        <h2 class="text-3xl font-bold text-gray-900 mb-8">Latest Posts</h2>
        
        <div class="grid md:grid-cols-3 gap-8">
            {% for post in posts %}
                <Card>
                    <n:image 
                        src="{{ post.cover_image }}" 
                        alt="{{ post.title }}"
                        class="w-full h-48 object-cover rounded-lg mb-4"
                    />
                    <Badge variant="default">{{ post.category }}</Badge>
                    <h3 class="text-xl font-bold mt-2 mb-2">
                        <n:link href="/blog/{{ post.slug }}">{{ post.title }}</n:link>
                    </h3>
                    <p class="text-gray-600 text-sm mb-4">{{ post.excerpt }}</p>
                    <div class="flex items-center justify-between text-sm text-gray-500">
                        <span>{{ post.author }}</span>
                        <span>{{ post.date }}</span>
                    </div>
                </Card>
            {% endfor %}
        </div>
        
        <!-- Pagination -->
        <div class="flex justify-center gap-2 mt-12">
            <Button variant="ghost" disabled="true">← Previous</Button>
            <Button variant="ghost">Next →</Button>
        </div>
    </section>
    
    <!-- Newsletter with n:form -->
    <section class="bg-slate-900 text-white py-16 px-6">
        <div class="max-w-xl mx-auto text-center">
            <h2 class="text-3xl font-bold mb-4">Subscribe to our newsletter</h2>
            <p class="text-slate-400 mb-8">Get the latest posts delivered to your inbox.</p>
            
            <n:form action="handlers::subscribe" method="POST" class="flex gap-2">
                <TextInput name="email" type="email" placeholder="you@example.com" required="true" />
                <Button type="submit" variant="primary">Subscribe</Button>
            </n:form>
        </div>
    </section>
</n:view>`,
    css: ``,
        files: {
            'components/Button.ncl': {
                content: `<n:component name="Button">
    <n:props>
        variant: String = "primary"
        size: String = "md"
        href: String = ""
        type: String = "button"
        onclick: String = ""
        id: String = ""
    </n:props>
    <n:if condition="href">
        <a href="{{ href }}" id="{{ id }}" onclick="{{ onclick }}" class="px-4 py-2 rounded-lg font-medium transition-colors inline-block {{ variant == 'primary' ? 'bg-indigo-600 text-white hover:bg-indigo-700' : 'bg-white text-gray-900 border border-gray-200 hover:bg-gray-50' }}">
            <n:slot />
        </a>
    </n:if>
    <n:if condition="!href">
        <button type="{{ type }}" id="{{ id }}" onclick="{{ onclick }}" class="px-4 py-2 rounded-lg font-medium transition-colors {{ variant == 'primary' ? 'bg-indigo-600 text-white hover:bg-indigo-700' : 'bg-white text-gray-900 border border-gray-200 hover:bg-gray-50' }}">
            <n:slot />
        </button>
    </n:if>
</n:component>`,
                language: 'html'
            },
            'components/Card.ncl': {
                content: `<n:component name="Card">
    <div class="bg-white p-6 rounded-xl border border-gray-100 shadow-sm overflow-hidden">
        <n:slot />
    </div>
</n:component>`,
                language: 'html'
            },
            'components/Badge.ncl': {
                content: `<n:component name="Badge">
    <n:props>variant: String = "default"</n:props>
    <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium {{ variant == 'default' ? 'bg-indigo-100 text-indigo-800' : 'bg-white/20 text-white' }}">
        <n:slot />
    </span>
</n:component>`,
                language: 'html'
            }
        }
    },

    // --- STATE MANAGEMENT EXAMPLES ---
    state_client: {
        ncl: `<n:view title="State: Simple (Client)">
    <!-- 
        LEVEL 1: Simple Client State
        Use <n:client> and standard DOM events.
        Best for: Toggles, counters, modal open/close.
    -->
    
    <main class="p-10 font-sans text-center">
        <h1 class="text-2xl font-bold mb-4">Client-Side State</h1>
        
        <div class="p-6 bg-white rounded-lg shadow-lg max-w-sm mx-auto border">
            <div id="status" class="text-lg mb-4 text-gray-600">Off</div>
            
            <button 
                onclick="toggleState()" 
                class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition"
            >
                Toggle
            </button>
        </div>
        
        <p class="mt-8 text-gray-500 max-w-md mx-auto">
            This state lives entirely in the browser using standard JavaScript variables.
        </p>
    </main>
    
    <n:client>
        let isOn = false;
        
        function toggleState() {
            isOn = !isOn;
            const el = document.getElementById('status');
            
            if (isOn) {
                el.textContent = "On ✅";
                el.className = "text-lg mb-4 text-green-600 font-bold";
            } else {
                el.textContent = "Off ❌";
                el.className = "text-lg mb-4 text-gray-600";
            }
        }
    </n:client>
</n:view>`,
        css: ``
    },

    state_form: {
        ncl: `<n:view title="State: Forms (Server)">
    <!-- 
        LEVEL 2: Server State via Forms
        Use <n:form> to mutate server state.
        Best for: data mutation, saving to DB.
    -->
    
    <n:model count="db::get_count()" />
    
    <main class="p-10 font-sans text-center">
        <h1 class="text-2xl font-bold mb-4">Server-Side State</h1>
        
        <div class="p-6 bg-white rounded-lg shadow-lg max-w-sm mx-auto border">
            <h2 class="text-4xl font-bold mb-6">{{ count }}</h2>
            
            <div class="flex gap-2 justify-center">
                <!-- Forms submit to server actions -->
                <n:form action="handlers::decrement">
                    <Button type="submit" variant="secondary" size="sm">-</Button>
                </n:form>
                
                <n:form action="handlers::increment">
                    <Button type="submit" variant="primary" size="sm">+</Button>
                </n:form>
            </div>
        </div>
        
        <p class="mt-8 text-gray-500 max-w-md mx-auto">
            Clicking submits a form to the server, updates the DB, and re-renders the page.
        </p>
    </main>
</n:view>`,
        css: ``,
        files: {
            'components/Button.ncl': {
                content: `<n:component name="Button">
    <n:props>
        variant: String = "primary"
        size: String = "md"
        href: String = ""
        type: String = "button"
        onclick: String = ""
        id: String = ""
    </n:props>
    <n:if condition="href">
        <a href="{{ href }}" id="{{ id }}" onclick="{{ onclick }}" class="px-4 py-2 rounded-lg font-medium transition-colors inline-block {{ variant == 'primary' ? 'bg-indigo-600 text-white hover:bg-indigo-700' : 'bg-white text-gray-900 border border-gray-200 hover:bg-gray-50' }}">
            <n:slot />
        </a>
    </n:if>
    <n:if condition="!href">
        <button type="{{ type }}" id="{{ id }}" onclick="{{ onclick }}" class="px-4 py-2 rounded-lg font-medium transition-colors {{ variant == 'primary' ? 'bg-indigo-600 text-white hover:bg-indigo-700' : 'bg-white text-gray-900 border border-gray-200 hover:bg-gray-50' }}">
            <n:slot />
        </button>
    </n:if>
</n:component>`,
                language: 'html'
            }
        }
    },

    state_island: {
        ncl: `<n:view title="State: Neutron (Complex)">
    <!-- 
        LEVEL 3: Complex Interactive State
        Use <n:island> with Neutron signals (Rust -> ASM).
        Best for: Rich interactive apps, dashboards.
    -->
    
    <main class="p-10 font-sans">
        <h1 class="text-2xl font-bold mb-8 text-center">Neutron Signals (Island)</h1>
        
        <!-- This component hydrates on the client -->
        <n:island src="components/CounterIsland" />
        
        <div class="mt-8 p-4 bg-slate-900 text-slate-300 rounded max-w-lg mx-auto text-sm font-mono text-left">
<pre>
// components/CounterIsland.ncl
use nucleus_std::neutron::{Signal, computed};

let count = Signal::new(0);
let double = computed(count.clone(), |c| c * 2);

&lt;button onclick={|_| count.update(|c| *c += 1)}&gt;
    Count: {count} (x2: {double})
&lt;/button&gt;
</pre>
        </div>
    </main>
</n:view>`,
        css: ``,
        files: {
            'components/CounterIsland.ncl': { 
                content: `// CounterIsland.ncl - Hydrates on the client with Neutron signals
use nucleus_std::neutron::{Signal, computed};

// Reactive state
let count = Signal::new(0);
let double = computed(count.clone(), |c| c * 2);

<div class="p-6 bg-white rounded-xl shadow-lg max-w-sm mx-auto text-center">
    <h2 class="text-lg font-semibold mb-4 text-gray-800">Neutron Counter</h2>
    
    <div class="text-5xl font-bold text-indigo-600 mb-4">{count}</div>
    <div class="text-sm text-gray-500 mb-6">Double: {double}</div>
    
    <div class="flex gap-3 justify-center">
        <Button onclick={|_| count.update(|c| *c -= 1)} variant="secondary" size="sm">
            Decrease
        </Button>
        <Button onclick={|_| count.update(|c| *c += 1)} variant="primary" size="sm">
            Increase
        </Button>
    </div>
</div>`, 
                language: 'rust'
            },
            'components/Button.ncl': {
                content: `<n:component name="Button">
    <n:props>
        variant: String = "primary"
        size: String = "md"
        href: String = ""
        type: String = "button"
        onclick: String = ""
        id: String = ""
    </n:props>
    <n:if condition="href">
        <a href="{{ href }}" id="{{ id }}" onclick="{{ onclick }}" class="px-4 py-2 rounded-lg font-medium transition-colors inline-block {{ variant == 'primary' ? 'bg-indigo-600 text-white hover:bg-indigo-700' : 'bg-white text-gray-900 border border-gray-200 hover:bg-gray-50' }}">
            <n:slot />
        </a>
    </n:if>
    <n:if condition="!href">
        <button type="{{ type }}" id="{{ id }}" onclick="{{ onclick }}" class="px-4 py-2 rounded-lg font-medium transition-colors {{ variant == 'primary' ? 'bg-indigo-600 text-white hover:bg-indigo-700' : 'bg-white text-gray-900 border border-gray-200 hover:bg-gray-50' }}">
            <n:slot />
        </button>
    </n:if>
</n:component>`,
                language: 'html'
            }
        }
    }
};

// ============================================
// PROJECT SYSTEM - State Management
// ============================================ 

let editor;
let activeFilename = 'main.ncl';

// Current project state
let currentProject = {
    id: null,
    name: 'Untitled Project',
    template: null,
    forked: false,
    createdAt: null,
    modifiedAt: null,
    files: {
        'main.ncl': { content: '', language: 'html' },
        'styles.css': { content: '', language: 'css' }
    }
};

// All projects stored in localStorage
let projects = {};

// User preferences
let userState = {
    activeProjectId: null,
    favorites: [],
    recentProjects: [],
    sidebarView: 'files'
};

let resources = {
    css: [],
    js: []
};

// ============================================
// MOCK DATA & HANDLERS (Simulated Server)
// ============================================

// Hoisted Mock Data for Persistence
let mockData = {
    users: [
        { name: 'Alex', email: 'alex@example.com', role: 'Admin', avatar: 'https://ui-avatars.com/api/?name=Alex&background=6366f1&color=fff' },
        { name: 'Sam', email: 'sam@example.com', role: 'Editor', avatar: 'https://ui-avatars.com/api/?name=Sam&background=10b981&color=fff' },
        { name: 'Jordan', email: 'jordan@example.com', role: 'Viewer', avatar: 'https://ui-avatars.com/api/?name=Jordan&background=f59e0b&color=fff' }
    ],
    posts: [
        { 
            title: 'Getting Started with Nucleus', 
            slug: 'getting-started',
            excerpt: 'Learn the basics of Nucleus framework in 5 minutes.',
            author: 'Alex',
            date: 'Oct 12, 2025',
            category: 'Tutorial',
            cover_image: 'https://images.unsplash.com/photo-1498050108023-c5249f4df085?auto=format&fit=crop&w=800&q=80'
        },
        { 
            title: 'Why Rust?', 
            slug: 'why-rust',
            excerpt: 'Exploring the benefits of Rust for web development.',
            author: 'Sam',
            date: 'Oct 15, 2025',
            category: 'Opinion',
            cover_image: 'https://images.unsplash.com/photo-1518770660439-4636190af475?auto=format&fit=crop&w=800&q=80'
        },
        { 
            title: 'Islands Architecture Explained', 
            slug: 'islands-architecture',
            excerpt: 'How Partial Hydration works under the hood.',
            author: 'Jordan',
            date: 'Oct 20, 2025',
            category: 'Deep Dive',
            cover_image: 'https://images.unsplash.com/photo-1558494949-ef010cbdcc31?auto=format&fit=crop&w=800&q=80'
        }
    ],
    todos: [
        { id: 1, title: 'Learn Nucleus', completed: true },
        { id: 2, title: 'Build a project', completed: false },
        { id: 3, title: 'Deploy to production', completed: false }
    ],
    stats: {
        total_users: '12,847',
        revenue: '$48,230',
        orders: '1,429'
    },
    count: 42,
    double: 84, // For Neutron example
    user: { id: 1, name: 'Demo User', email: 'demo@nucleus.dev' }
};

// Message Listener for Simulated Server Actions (from iframe)
window.addEventListener('message', (event) => {
    if (event.data.type === 'form:submit') {
        const { action, formData } = event.data;
        handleMockServerAction(action, formData);
    }
});

function handleMockServerAction(action, formData) {
    console.log('[Mock Server] Processing action:', action, formData);
    let reRender = true;
    
    // Simulate Server Handling
    switch (action) {
        // Todo Actions
        case 'handlers::add_todo':
            const newId = Math.max(0, ...mockData.todos.map(t => t.id)) + 1;
            mockData.todos.push({
                id: newId,
                title: formData.title || 'New Todo',
                completed: false
            });
            showToast('Todo added!', 'success');
            break;
            
        case 'handlers::toggle_todo':
            const tId = parseInt(formData.id);
            const todo = mockData.todos.find(t => t.id === tId);
            if (todo) {
                todo.completed = !todo.completed;
                // Since checkboxes submit 'on' if checked, toggle logic handles the flip
            }
            break;
            
        case 'handlers::delete_todo':
            const dId = parseInt(formData.id);
            mockData.todos = mockData.todos.filter(t => t.id !== dId);
            showToast('Todo deleted', 'info');
            break;

        // Counter Actions
        case 'handlers::increment':
            mockData.count++;
            showToast('Count incremented', 'success');
            break;
            
        case 'handlers::decrement':
            mockData.count--;
            showToast('Count decremented', 'success');
            break;
            
        case 'handlers::subscribe':
            showToast(`Subscribed: ${formData.email}`, 'success');
            break;
            
        default:
            console.warn('Unknown mock handler:', action);
            showToast(`Mock action: ${action}`, 'info');
            reRender = false;
    }
    
    // Re-render "SSR" view if data changed
    if (reRender) {
        compile();
    }
}


// Shorthand for current project files
function getFiles() {
    return currentProject.files;
}

// Generate unique ID
function generateId() {
    return 'proj_' + Date.now().toString(36) + Math.random().toString(36).substr(2, 9);
}

// ============================================
// INITIALIZATION
// ============================================

let playgroundInitialized = false;

/**
 * Initialize the playground - can be called on initial load or SPA navigation
 */
async function initPlayground() {
    // Skip if already initialized (prevents double-init on SPA nav)
    if (playgroundInitialized && editor) {
        return;
    }
    
    // Check if we're on the playground page
    if (!document.getElementById('monaco-container')) {
        return;
    }
    
    await require.config({ paths: { vs: 'https://cdn.jsdelivr.net/npm/monaco-editor@0.45.0/min/vs' } });
    
    require(['vs/editor/editor.main'], function() {
        // Core Theme Definition (Nucleus Dark)
        monaco.editor.defineTheme('nucleus-dark', {
            base: 'vs-dark',
            inherit: true,
            rules: [
                { token: 'tag', foreground: '6366f1' },
                { token: 'attribute.name', foreground: 'a855f7' },
                { token: 'attribute.value', foreground: '22c55e' },
            ],
            colors: {
                'editor.background': '#0f0f1a',
            }
        });

        // Initialize Main Editor
        editor = monaco.editor.create(document.getElementById('monaco-container'), {
            value: '',
            language: 'html',
            theme: 'nucleus-dark',
            minimap: { enabled: false },
            fontSize: 14,
            padding: { top: 16 },
            automaticLayout: true
        });

        // Bind Change Events with Auto-Fork
        editor.onDidChangeModelContent(() => {
            if (currentProject.files[activeFilename]) {
                currentProject.files[activeFilename].content = editor.getValue();
                currentProject.modifiedAt = Date.now();
                
                // Auto-fork logic: if editing an example template
                if (currentProject.template && !currentProject.forked) {
                    currentProject.forked = true;
                    currentProject.name = `My ${getTemplateName(currentProject.template)}`;
                    updateProjectHeader();
                    showToast('Project forked! Your changes are saved separately.', 'info');
                }
                
                saveCurrentProject();
                debouncedCompile();
            }
        });

        // Initialize project system
        initProjectSystem();
        playgroundInitialized = true;
    });

    // Setup UI Interactions
    setupSidebar();
    setupSidebarTabs();
    setupProjectDropdown();
    setupTemplateButtons();
    setupFileSearch();
    setupPreviewTabs();
    setupResizer();
    setupFileActions();
    loadPreferences();
    
    // Expose functions needed by inline onclick handlers
    window.toggleSection = toggleSection;
    window.toggleFavorite = toggleFavorite;
    window.deleteFile = deleteFile;
    window.addResource = addResource;
}

// Initial load
document.addEventListener('DOMContentLoaded', initPlayground);

// SPA navigation handler - reinitialize when navigated via n:link
// SPA navigation handler - reinitialize when navigated via n:link
window.addEventListener('nucleus:navigate', () => {
    // Reset state for re-initialization
    playgroundInitialized = false;
    editor = null;

    // Reinitialize if we navigated to playground
    // Use polling instead of setTimeout to wait for DOM update
    let attempts = 0;
    const maxAttempts = 50; // 50 * 16ms ≈ 800ms timeout

    function checkAndInit() {
        const container = document.getElementById('monaco-container');
        if (container) {
            initPlayground();
        } else if (attempts < maxAttempts) {
            attempts++;
            requestAnimationFrame(checkAndInit);
        }
    }

    checkAndInit();
});

// ============================================
// PROJECT SYSTEM
// ============================================

function initProjectSystem() {
    // Load all projects from localStorage
    const savedProjects = localStorage.getItem('playground-projects');
    if (savedProjects) {
        try {
            projects = JSON.parse(savedProjects);
        } catch (e) {
            console.warn('Failed to parse saved projects, resetting');
            projects = {};
        }
    }
    
    // Load user state
    const savedUserState = localStorage.getItem('playground-user-state');
    if (savedUserState) {
        try {
            const parsed = JSON.parse(savedUserState);
            userState = { ...userState, ...parsed };
        } catch (e) {
            console.warn('Failed to parse user state, resetting');
        }
    }
    
    // CLEANUP: Remove orphaned recents (projects that no longer exist)
    userState.recentProjects = userState.recentProjects.filter(id => projects[id]);
    
    // CLEANUP: Remove orphaned favorites
    userState.favorites = userState.favorites.filter(id => projects[id]);
    
    // CLEANUP: Remove duplicates from recents (keep first occurrence)
    userState.recentProjects = [...new Set(userState.recentProjects)];
    
    // Load last active project or create default
    if (userState.activeProjectId && projects[userState.activeProjectId]) {
        loadProject(userState.activeProjectId);
    } else {
        // Create default project from hello template
        createProjectFromTemplate('hello', 'Hello World');
    }
    
    renderFileTree();
    renderTabs();
    compile();
}

function createProjectFromTemplate(templateKey, name) {
    const template = EXAMPLES[templateKey];
    if (!template) return null;
    
    const id = generateId();
    const now = Date.now();
    
    currentProject = {
        id,
        name: name || getTemplateName(templateKey),
        template: templateKey,
        forked: false,
        createdAt: now,
        modifiedAt: now,
        files: {
            'main.ncl': { content: template.ncl, language: 'html' },
            'styles.css': { content: template.css || '', language: 'css' },
            // Include additional files if template defines them
            ...(template.files || {})
        }
    };
    
    // Save to projects
    projects[id] = { ...currentProject };
    userState.activeProjectId = id;
    
    // Add to recents
    addToRecents(id);
    
    saveAllProjects();
    saveUserState();
    
    // Update UI & Editor
    activeFilename = 'main.ncl';
    updateEditorContent();
    
    updateProjectHeader();
    renderFileTree();
    renderTabs();
    compile();
    
    return id;
}

// Helper to safely update editor content
function updateEditorContent() {
    if (!editor) return;
    const file = currentProject.files[activeFilename];
    if (!file) return;
    
    // Temporarily disable change handler to prevent auto-fork trigger
    const model = editor.getModel();
    if (model) {
        editor.setValue(file.content);
        monaco.editor.setModelLanguage(model, file.language);
    }
}

function createNewProject() {
    const name = prompt('Project name:', 'New Project');
    if (!name) return;
    
    const id = generateId();
    const now = Date.now();
    
    currentProject = {
        id,
        name,
        template: null,
        forked: false,
        createdAt: now,
        modifiedAt: now,
        files: {
            'main.ncl': { content: '<n:view title="' + name + '">\n    <main>\n        <h1>Hello!</h1>\n    </main>\n</n:view>', language: 'html' },
            'styles.css': { content: '/* Add your styles here */\n', language: 'css' }
        }
    };
    
    projects[id] = { ...currentProject };
    userState.activeProjectId = id;
    addToRecents(id);
    
    saveAllProjects();
    saveUserState();
    
    activeFilename = 'main.ncl';
    updateEditorContent();
    
    updateProjectHeader();
    renderFileTree();
    renderTabs();
    renderProjectsList();
    compile();
    
    closeProjectDropdown();
    showToast('Project created!', 'success');
}

function loadProject(projectId) {
    if (!projects[projectId]) return;
    
    // Save current project first
    if (currentProject.id) {
        projects[currentProject.id] = { ...currentProject };
    }
    
    currentProject = { ...projects[projectId] };
    userState.activeProjectId = projectId;
    addToRecents(projectId);
    
    saveUserState();
    
    // Reset active file
    activeFilename = 'main.ncl';
    updateEditorContent();
    
    updateProjectHeader();
    renderFileTree();
    renderTabs();
    compile();
}

function saveCurrentProject() {
    if (currentProject.id) {
        projects[currentProject.id] = { ...currentProject };
        saveAllProjects();
    }
}

function saveAllProjects() {
    localStorage.setItem('playground-projects', JSON.stringify(projects));
}

function saveUserState() {
    localStorage.setItem('playground-user-state', JSON.stringify(userState));
}

function deleteProject(projectId, e) {
    if (e) e.stopPropagation();
    if (!confirm('Delete this project?')) return;
    
    delete projects[projectId];
    userState.favorites = userState.favorites.filter(id => id !== projectId);
    userState.recentProjects = userState.recentProjects.filter(id => id !== projectId);
    
    saveAllProjects();
    saveUserState();
    
    // If deleted current project, switch to another
    if (currentProject.id === projectId) {
        const projectIds = Object.keys(projects);
        if (projectIds.length > 0) {
            loadProject(projectIds[0]);
        } else {
            createProjectFromTemplate('hello', 'Hello World');
        }
    }
    
    renderProjectsList();
    renderFavorites();
    renderRecents();
}

// ============================================
// FAVORITES & RECENTS
// ============================================

function toggleFavorite(projectId, e) {
    if (e) e.stopPropagation();
    
    const idx = userState.favorites.indexOf(projectId);
    if (idx === -1) {
        userState.favorites.push(projectId);
        showToast('Added to favorites!', 'success');
    } else {
        userState.favorites.splice(idx, 1);
    }
    
    saveUserState();
    renderFavorites();
    renderProjectsList();
}

function addToRecents(projectId) {
    // Remove if already exists
    userState.recentProjects = userState.recentProjects.filter(id => id !== projectId);
    // Add to front
    userState.recentProjects.unshift(projectId);
    // Keep only last 10
    userState.recentProjects = userState.recentProjects.slice(0, 10);
}

function getTemplateName(key) {
    const names = {
        hello: 'Hello World',
        landing: 'Landing Page',
        card: 'Card Component',
        button: 'Button System',
        form: 'Forms & Inputs',
        wizard: 'Multi-Step Form',
        dashboard: 'Dashboard',
        auth: 'Auth Pages'
    };
    return names[key] || key;
}

// ============================================
// UI RENDERING
// ============================================

function updateProjectHeader() {
    const nameEl = document.getElementById('current-project-name');
    if (nameEl) {
        nameEl.textContent = currentProject.name;
    }
}

function renderFileTree() {
    const container = document.getElementById('file-tree');
    if (!container) return;
    
    container.innerHTML = '';
    
    const files = currentProject.files;
    const sortedFiles = Object.keys(files).sort((a, b) => {
        if (a === 'main.ncl') return -1;
        if (b === 'main.ncl') return 1;
        return a.localeCompare(b);
    });
    
    sortedFiles.forEach(filename => {
        const item = document.createElement('div');
        item.className = `tree-item ${filename === activeFilename ? 'active' : ''}`;
        item.onclick = () => selectFile(filename);
        
        let icon = '📄';
        if (filename.endsWith('.ncl')) icon = '⚡';
        else if (filename.endsWith('.css')) icon = '🎨';
        else if (filename.endsWith('.js')) icon = '📜';
        
        item.innerHTML = `
            <span class="tree-item-icon">${icon}</span>
            <span class="tree-item-name">${filename}</span>
            <div class="tree-item-actions">
                ${filename !== 'main.ncl' ? `<button class="tree-item-btn delete" onclick="deleteFile('${filename}', event)" title="Delete">×</button>` : ''}
            </div>
        `;
        
        container.appendChild(item);
    });
}

function renderProjectsList() {
    const container = document.getElementById('projects-list');
    if (!container) return;
    
    container.innerHTML = '';
    
    Object.values(projects).forEach(proj => {
        const item = document.createElement('button');
        item.className = `dropdown-item ${proj.id === currentProject.id ? 'active' : ''}`;
        item.onclick = () => {
            loadProject(proj.id);
            closeProjectDropdown();
        };
        
        const isFav = userState.favorites.includes(proj.id);
        
        item.innerHTML = `
            <span>📁</span>
            <span style="flex:1; text-align:left;">${proj.name}</span>
            <span onclick="toggleFavorite('${proj.id}', event)" style="cursor:pointer;">${isFav ? '★' : '☆'}</span>
        `;
        
        container.appendChild(item);
    });
}

function renderFavorites() {
    const container = document.getElementById('favorites-list');
    const emptyState = document.getElementById('favorites-empty');
    if (!container) return;
    
    const favProjects = userState.favorites.map(id => projects[id]).filter(Boolean);
    
    if (favProjects.length === 0) {
        emptyState?.classList.remove('hidden');
        container.innerHTML = '';
        return;
    }
    
    emptyState?.classList.add('hidden');
    container.innerHTML = '';
    
    favProjects.forEach(proj => {
        container.appendChild(createProjectCard(proj, true));
    });
}

function renderRecents() {
    const container = document.getElementById('recents-list');
    const emptyState = document.getElementById('recents-empty');
    if (!container) return;
    
    const recentProjs = userState.recentProjects.map(id => projects[id]).filter(Boolean);
    
    if (recentProjs.length === 0) {
        emptyState?.classList.remove('hidden');
        container.innerHTML = '';
        return;
    }
    
    emptyState?.classList.add('hidden');
    container.innerHTML = '';
    
    recentProjs.slice(0, 5).forEach(proj => {
        container.appendChild(createProjectCard(proj, false));
    });
}

function createProjectCard(proj, showFav) {
    const card = document.createElement('div');
    card.className = 'project-card';
    card.onclick = () => {
        loadProject(proj.id);
        switchSidebarView('files');
    };
    
    const isFav = userState.favorites.includes(proj.id);
    const timeAgo = getTimeAgo(proj.modifiedAt);
    
    card.innerHTML = `
        <span class="project-card-icon">📁</span>
        <div class="project-card-info">
            <div class="project-card-name">${proj.name}</div>
            <div class="project-card-meta">${timeAgo}</div>
        </div>
        ${showFav ? `<span class="project-card-fav ${isFav ? 'active' : ''}" onclick="toggleFavorite('${proj.id}', event)">★</span>` : ''}
    `;
    
    return card;
}

function getTimeAgo(timestamp) {
    if (!timestamp) return '';
    const diff = Date.now() - timestamp;
    const mins = Math.floor(diff / 60000);
    if (mins < 1) return 'Just now';
    if (mins < 60) return `${mins}m ago`;
    const hours = Math.floor(mins / 60);
    if (hours < 24) return `${hours}h ago`;
    const days = Math.floor(hours / 24);
    return `${days}d ago`;
}

// ============================================
// SIDEBAR INTERACTIONS
// ============================================

function setupSidebarTabs() {
    document.querySelectorAll('.sidebar-tab').forEach(tab => {
        tab.addEventListener('click', () => {
            const view = tab.dataset.view;
            switchSidebarView(view);
        });
    });
}

function switchSidebarView(view) {
    userState.sidebarView = view;
    
    // Update tabs
    document.querySelectorAll('.sidebar-tab').forEach(tab => {
        tab.classList.toggle('active', tab.dataset.view === view);
        tab.setAttribute('aria-selected', tab.dataset.view === view);
    });
    
    // Update views
    document.querySelectorAll('.sidebar-view').forEach(v => {
        v.classList.toggle('active', v.id === `${view}-view`);
        v.classList.toggle('hidden', v.id !== `${view}-view`);
    });
    
    // Render content
    if (view === 'favorites') renderFavorites();
    if (view === 'recents') renderRecents();
}

function setupProjectDropdown() {
    const btn = document.getElementById('project-dropdown-btn');
    const dropdown = document.getElementById('project-dropdown');
    
    if (btn && dropdown) {
        btn.addEventListener('click', () => {
            dropdown.classList.toggle('hidden');
            if (!dropdown.classList.contains('hidden')) {
                renderProjectsList();
            }
        });
        
        // Close on outside click
        document.addEventListener('click', (e) => {
            if (!btn.contains(e.target) && !dropdown.contains(e.target)) {
                dropdown.classList.add('hidden');
            }
        });
    }
}

function closeProjectDropdown() {
    document.getElementById('project-dropdown')?.classList.add('hidden');
}

function setupTemplateButtons() {
    document.querySelectorAll('.template-item').forEach(btn => {
        btn.addEventListener('click', () => {
            const templateKey = btn.dataset.template;
            if (templateKey) {
                // Create new project from template
                createProjectFromTemplate(templateKey, getTemplateName(templateKey));
                compile();
                showToast(`Loaded ${getTemplateName(templateKey)}`, 'success');
            }
        });
    });
}

function setupFileSearch() {
    const input = document.getElementById('file-search');
    if (input) {
        input.addEventListener('input', (e) => {
            const query = e.target.value.toLowerCase();
            filterFileTree(query);
        });
    }
}

function setupPreviewTabs() {
    const previewTabs = document.querySelectorAll('[data-preview]');
    previewTabs.forEach(tab => {
        tab.addEventListener('click', () => {
            const mode = tab.dataset.preview;
            
            // Update tab active states
            previewTabs.forEach(t => {
                t.classList.toggle('active', t.dataset.preview === mode);
                t.setAttribute('aria-selected', t.dataset.preview === mode);
            });
            
            // Toggle views
            const previewFrame = document.getElementById('preview-frame');
            const htmlOutput = document.getElementById('html-output');
            
            if (mode === 'render') {
                previewFrame?.classList.remove('hidden');
                htmlOutput?.classList.add('hidden');
            } else if (mode === 'html') {
                previewFrame?.classList.add('hidden');
                htmlOutput?.classList.remove('hidden');
            }
        });
    });
}

function filterFileTree(query) {
    const items = document.querySelectorAll('#file-tree .tree-item');
    items.forEach(item => {
        const name = item.querySelector('.tree-item-name')?.textContent?.toLowerCase() || '';
        item.style.display = name.includes(query) ? '' : 'none';
    });
}

function toggleSection(sectionId) {
    const section = document.getElementById(`${sectionId}-section`);
    const icon = document.getElementById(`${sectionId}-collapse-icon`);
    
    if (section) {
        section.classList.toggle('hidden');
        if (icon) {
            icon.textContent = section.classList.contains('hidden') ? '▸' : '▾';
        }
    }
}

// Toast notifications
function showToast(message, type = 'info') {
    // Remove existing toast
    document.querySelectorAll('.toast').forEach(t => t.remove());
    
    const toast = document.createElement('div');
    toast.className = `toast ${type}`;
    toast.textContent = message;
    document.body.appendChild(toast);
    
    requestAnimationFrame(() => {
        toast.classList.add('show');
    });
    
    setTimeout(() => {
        toast.classList.remove('show');
        setTimeout(() => toast.remove(), 300);
    }, 3000);
}

function setupFileActions() {
    const dialog = document.getElementById('new-file-dialog');
    const input = document.getElementById('new-filename');
    const createBtn = document.getElementById('btn-create-file');

    if (!createBtn) return;

    createBtn.onclick = (e) => {
        e.preventDefault();
        const name = input.value.trim();
        if (!name) return;
        
        // Simple extension check
        let lang = 'text';
        if (name.endsWith('.ncl')) lang = 'html';
        else if (name.endsWith('.css')) lang = 'css';
        else if (name.endsWith('.js')) lang = 'javascript';
        else if (name.endsWith('.ts')) lang = 'typescript';
        else if (name.endsWith('.json')) lang = 'json';
        
        if (currentProject.files[name]) {
            alert('File already exists!');
            return;
        }

        currentProject.files[name] = { content: '', language: lang };
        selectFile(name);
        renderFileTree();
        saveCurrentProject();
        dialog.close();
        input.value = '';
        showToast(`Created ${name}`, 'success');
    };
}


// ... Helper Functions ...

function setupSidebar() {
    const tailwindToggle = document.getElementById('use-tailwind');
    if (tailwindToggle) {
        tailwindToggle.addEventListener('change', () => {
            savePreferences();
            compile();
        });
    }
}

function savePreferences() {
    const preferences = {
        useTailwind: document.getElementById('use-tailwind')?.checked || false,
        resources: resources
    };
    localStorage.setItem('playground-prefs', JSON.stringify(preferences));
}

function loadPreferences() {
    const saved = localStorage.getItem('playground-prefs');
    if (saved) {
        const prefs = JSON.parse(saved);
        const tailwindToggle = document.getElementById('use-tailwind');
        if (tailwindToggle) {
            tailwindToggle.checked = prefs.useTailwind;
        }
        if (prefs.resources) {
            resources = prefs.resources;
            renderResourceList('css');
            renderResourceList('js');
        }
        setTimeout(compile, 500);
    } else {
        const tailwindToggle = document.getElementById('use-tailwind');
        if (tailwindToggle) {
            tailwindToggle.checked = true; // Default to Tailwind ON
        }
    }
}

// Helper to open Add Resource Dialog
function openResourceDialog(type) {
    const dialog = document.getElementById('resource-dialog');
    const input = document.getElementById('resource-input');
    const title = document.getElementById('resource-dialog-title');
    const btn = document.getElementById('btn-add-resource');
    
    // Configure
    title.textContent = `Add External ${type.toUpperCase()}`;
    input.value = '';
    
    // Handle Add
    const handleAdd = (e) => {
        e.preventDefault();
        const url = input.value.trim();
        if (url) {
            resources[type].push(url);
            renderResourceList(type);
            savePreferences();
            compile();
            dialog.close();
        }
    };
    
    // Clean up old listener (simple way: clone button)
    const newBtn = btn.cloneNode(true);
    btn.parentNode.replaceChild(newBtn, btn);
    newBtn.onclick = handleAdd;

    dialog.showModal();
}

// Replaced addResource stub
function addResource(type) {
    openResourceDialog(type);
}

function removeResource(type, index) {
    resources[type].splice(index, 1);
    renderResourceList(type);
    savePreferences();
    compile();
}

function renderResourceList(type) {
    const list = document.getElementById(`${type}-resources`);
    list.innerHTML = '';
    resources[type].forEach((url, i) => {
        const div = document.createElement('div');
        div.className = 'resource-item';
        div.innerHTML = `
            <div class="resource-url" title="${url}">${url.split('/').pop()}</div>
            <button class="resource-remove" onclick="removeResource('${type}', ${i})">×</button>
        `;
        list.appendChild(div);
    });
}


// Persistence
function saveState() {
    if (!editor) return;
    localStorage.setItem('playground-files', JSON.stringify(files));
    localStorage.setItem('playground-active', activeFilename);
}

function loadState() {
    const savedFiles = localStorage.getItem('playground-files');
    const savedActive = localStorage.getItem('playground-active');
    
    if (savedFiles) {
        files = JSON.parse(savedFiles);
        activeFilename = savedActive || Object.keys(files)[0];
        
        // Restore editor content
        if (files[activeFilename]) {
             editor.setValue(files[activeFilename].content);
             monaco.editor.setModelLanguage(editor.getModel(), files[activeFilename].language);
        }
    }
}

// File Explorer Logic
function renderFileExplorer() {
    const list = document.getElementById('file-list');
    if (!list) return;
    
    list.innerHTML = '';
    
    Object.keys(files).sort().forEach(filename => {
        const item = document.createElement('div');
        item.className = `file-item ${filename === activeFilename ? 'active' : ''}`;
        
        let icon = '📄';
        if (filename.endsWith('.css')) icon = '🎨';
        else if (filename.endsWith('.js')) icon = '📜';
        else if (filename.endsWith('.ncl')) icon = '⚡';

        item.innerHTML = `
            <span class="file-icon">${icon}</span>
            <span class="file-name">${filename}</span>
            ${filename !== 'main.ncl' ? `<button class="file-delete" onclick="deleteFile('${filename}', event)">×</button>` : ''}
        `;
        
        item.onclick = () => selectFile(filename);
        list.appendChild(item);
    });
    
    renderTabs();
}

function selectFile(filename) {
    if (!currentProject.files[filename]) return;
    
    activeFilename = filename;
    const file = currentProject.files[filename];
    
    // Update Editor
    if (editor) {
        const model = editor.getModel();
        if (model) {
            // Only update if content differs (to avoid cursor jump)
            if(editor.getValue() !== file.content) {
                editor.setValue(file.content);
            }
            monaco.editor.setModelLanguage(model, file.language);
        }
    }
    
    // Update UI
    renderFileTree();
    renderTabs();
}

function deleteFile(filename, e) {
    if (e) e.stopPropagation();
    if (filename === 'main.ncl') {
        alert('Cannot delete main entry file.');
        return;
    }
    
    if (confirm(`Delete ${filename}?`)) {
        delete currentProject.files[filename];
        if (activeFilename === filename) {
            selectFile('main.ncl');
        } else {
            renderFileTree();
            renderTabs();
        }
        saveCurrentProject();
        showToast(`Deleted ${filename}`, 'info');
    }
}

function renderTabs() {
    const tabsContainer = document.getElementById('editor-tabs');
    if (!tabsContainer) return;
    
    tabsContainer.innerHTML = '';
    
    const filenames = Object.keys(currentProject.files).sort((a,b) => {
        if(a==='main.ncl') return -1;
        if(b==='main.ncl') return 1;
        return a.localeCompare(b);
    });

    filenames.forEach(name => {
         const btn = document.createElement('button');
         btn.className = `panel-tab ${name === activeFilename ? 'active' : ''}`;
         btn.setAttribute('role', 'tab');
         btn.setAttribute('aria-selected', name === activeFilename);
         
         let icon = '📄';
         if (name.endsWith('.ncl')) icon = '⚡';
         else if (name.endsWith('.css')) icon = '🎨';
         else if (name.endsWith('.js')) icon = '📜';
         
         btn.innerHTML = `<span class="tab-icon">${icon}</span> ${name}`;
         btn.onclick = () => selectFile(name);
         tabsContainer.appendChild(btn);
    });
}

function setupResizer() {
    const resizer = document.getElementById('resizer');
    const editorPanel = document.getElementById('editor-panel');
    let isResizing = false;

    if(!resizer || !editorPanel) return;

    resizer.addEventListener('mousedown', () => {
         isResizing = true;
         document.body.style.cursor = 'col-resize';
         document.body.style.userSelect = 'none';
    });
    
    document.addEventListener('mousemove', (e) => {
        if (!isResizing) return;
        const workspace = document.querySelector('.playground-workspace');
        if(!workspace) return;
        
        const percentage = ((e.clientX - workspace.getBoundingClientRect().left) / workspace.offsetWidth) * 100;
        if (percentage > 20 && percentage < 80) {
            editorPanel.style.flex = `0 0 ${percentage}%`;
        }
    });

    document.addEventListener('mouseup', () => {
        isResizing = false;
        document.body.style.cursor = '';
        document.body.style.userSelect = '';
        if(editor) editor.layout();
    });
}

// Template Logic
function loadTemplate(key, useSaved) {
    if (!editor) return;

    let nclCode = EXAMPLES[key].ncl;
    let cssCode = EXAMPLES[key].css;

    if (useSaved) {
        const saved = getSavedVersion(key);
        if (saved) {
            nclCode = saved.ncl;
            cssCode = saved.css;
        }
    }
    
    // Reset Files for Template
    files = {
        'main.ncl': { content: nclCode, language: 'html' },
        'styles.css': { content: cssCode, language: 'css' }
    };
    
    activeFilename = 'main.ncl';
    selectFile('main.ncl');
    
    // Ensure Tailwind
    const tailwindToggle = document.getElementById('use-tailwind');
    if (tailwindToggle && !tailwindToggle.checked) {
        tailwindToggle.checked = true;
        savePreferences();
    }
    
    saveState();
    
    // Wait for editor update then compile
    setTimeout(compile, 50);
}

// Compilation Logic
let debounceTimer;
function debouncedCompile() {
    clearTimeout(debounceTimer);
    updateStatus('compiling');
    debounceTimer = setTimeout(compile, 800);
}


// --- Neutron Mock Transpiler ---
function transpileNeutron(source) {
    // 1. Extract State (Signals)
    // let count = Signal::new(0);
    const signals = {};
    let scriptInit = '';
    
    // Regex for Signal::new
    const signalRegex = /let\s+(\w+)\s*=\s*Signal::new\(([^)]+)\);/g;
    let match;
    while ((match = signalRegex.exec(source)) !== null) {
        const [_, name, val] = match;
        signals[name] = { type: 'signal', initial: val.trim() };
        scriptInit += `state.${name} = ${val.trim()};\n`;
    }

    // 2. Extract Computed
    // let double = computed(count.clone(), |c| c * 2);
    // Simple mock: assume simple dependency and arrow function expression
    const computedRegex = /let\s+(\w+)\s*=\s*computed\s*\(\s*(\w+)\.clone\(\)\s*,\s*\|[^|]+\|\s*([^)]+)\);/g;
    while ((match = computedRegex.exec(source)) !== null) {
        const [_, name, dep, expr] = match;
        // Transform rust expr "c * 2" to JS "state.count * 2" loosely
        // ensuring we replace the closure var with state lookup is tricky in regex
        // valid assumption for playground: expression uses the variable name or closure arg
        // Simplified: computed is updated in the update() loop
        signals[name] = { type: 'computed', dep, expr: expr.replace(/[a-z]\s/g, `state.${dep} `) }; 
        // Note: very rough approximation
    }

    // 3. Process Template & Bindings
    // Remove rust-specific imports/setup lines (everything before first proper tag)
    // heuristic: find first < followed by a letter
    const tagMatch = source.match(/<[a-zA-Z]/);
    const firstTagIndex = tagMatch ? tagMatch.index : -1;
    let html = firstTagIndex >= 0 ? source.substring(firstTagIndex) : '';

    // Replace {variable} -> <span data-n-bind="variable"></span>
    // Safe guard: don't replace if it looks like a rust closure or struct { var: val }
    // Simple heuristic: {var} where var is alphanumeric
    // We iterate to avoid replacing inside attributes (roughly)
    // Actually, just replacing {var} is usually safe in NCL text content if it's alphanumeric.
    // Attributes usually use { assignment } or { |c| ... }
    
    html = html.replace(/\{(\w+)\}/g, (match, varName) => {
        // heuristic: ignore if it looks like part of a larger rust expression within {}
        // but the regex only matched {word}, so it's likely a binding.
        return `<span data-n-bind="${varName}"></span>`;
    });

    // Replace onclick closures
    // Handles both syntaxes:
    //   onclick={count.update(|c| *c += 1)}
    //   onclick={|_| count.update(|c| *c += 1)}
    // Uses [\s\S] instead of . to match across newlines
    html = html.replace(/onclick\s*=\s*\{(?:\|_\|\s*)?[\s\S]*?(\w+)\.update[\s\S]*?(\+=|-=|=)\s*(\d+)[\s\S]*?\}/gi, 
        (m, sig, op, val) => `onclick="return false" data-n-action="${sig}:${op}:${val}"`);

    // Clean up Component imports/usage in rust file if any (ignored for now)

    // Generate Runtime Script
    const script = `
    (function(elRoot) {
        const state = {};
        ${scriptInit}

        function update() {
            // Update bindings
            elRoot.querySelectorAll('[data-n-bind]').forEach(el => {
                const key = el.dataset.nBind;
                if (state[key] !== undefined) el.textContent = state[key];
                
                // Handle computed mocks (very naive)
                if (key === 'double' && state.count !== undefined) el.textContent = state.count * 2; 
            });
            
            // Check specific class-toggling logic from original example (hardcoded hack for demo fidelity if generic fails)
            const countEl = elRoot.querySelector('[data-n-bind="count"]');
            if(countEl && state.count !== undefined) {
                 countEl.parentElement.className = 'text-5xl font-bold mb-4 transition-all transform ' + (state.count % 2 === 0 ? 'text-indigo-600' : 'text-violet-600');
            }
        }

        // Attach listeners
        elRoot.querySelectorAll('[data-n-action]').forEach(btn => {
            if(btn.dataset.nAttached) return;
            btn.dataset.nAttached = 'true';
            btn.addEventListener('click', (e) => {
                e.preventDefault();
                const [sig, op, val] = btn.dataset.nAction.split(':');
                const numVal = parseInt(val, 10);
                
                if (state[sig] !== undefined) {
                    if (op === '+=') state[sig] += numVal;
                    if (op === '-=') state[sig] -= numVal;
                    if (op === '=') state[sig] = numVal;
                    update();
                }
            });
        });

        // Initial render
        update();
    })(document.currentScript.parentElement);
    `;

    return { html, script };
}

async function compile() {
    if (!editor) return;
    
    // Gather content from VFS
    let ncl = '';
    let css = '';
    let js = '';

    const files = currentProject.files;
    Object.keys(files).forEach(name => {
        if (name.endsWith('.ncl')) ncl += files[name].content + '\n';
        else if (name.endsWith('.css')) css += files[name].content + '\n';
        else if (name.endsWith('.js')) js += files[name].content + '\n';
    });

    const useTailwind = document.getElementById('use-tailwind')?.checked || false;

    try {
        // --- Enhanced Mock Compiler ---
        let html = ncl;
        
        // Mock data for template variable substitution
        // Mock data now uses the hoisted 'mockData' variable
        
        const componentDefs = {};
        html = html.replace(/<n:component\s+name="(\w+)">([\s\S]*?)<\/n:component>/g, (match, name, body) => {
            componentDefs[name] = body;
            return ''; // Remove definition from output
        });
        
        // 2. Process n:view (extract title for comments)
        html = html.replace(/<n:view[^>]*title="([^"]*)"[^>]*>/, '<!-- View: $1 -->');
        html = html.replace(/<n:view[^>]*>/, '');
        html = html.replace(/<\/n:view>/, '');
        
        // 3. Process n:layout  
        html = html.replace(/<n:layout\s+name="([^"]*)"[^>]*>/g, '<!-- Layout: $1 -->');
        html = html.replace(/<\/n:layout>/g, '');
        
        // 4. Process n:slot
        html = html.replace(/<n:slot\s*\/>/g, '<!-- slot content -->');
        html = html.replace(/<n:slot\s+name="([^"]*)"\s*\/>/g, '<!-- slot: $1 -->');
        
        // 5. Process n:props (remove from output, props are design-time)
        html = html.replace(/<n:props>[\s\S]*?<\/n:props>/g, '');
        
        // 6. Process scoped styles - keep the CSS
        html = html.replace(/<style\s+scoped>([\s\S]*?)<\/style>/g, '<style>$1</style>');
        
        // 7. Process n:for loops
        html = html.replace(/<n:for\s+item="(\w+)"\s+in="(\w+)">([\s\S]*?)<\/n:for>/g, 
            (match, item, collection, body) => {
                const data = mockData[collection] || [];
                if (data.length === 0) return `<!-- Loop: ${collection} (empty) -->`;
                
                return data.map(itemData => {
                    let result = body;
                    // Replace {{ item.property }}
                    result = result.replace(new RegExp(`\\{\\{\\s*${item}\\.(\\w+)\\s*\\}\\}`, 'g'), 
                        (m, prop) => itemData[prop] !== undefined ? itemData[prop] : `[${item}.${prop}]`);
                    return result;
                }).join('\n');
            });
        
        // 8. Process {% for %} (Jinja-style) with mock data
        html = html.replace(/\{%\s*for\s+(\w+)\s+in\s+(\w+)\s*%\}([\s\S]*?)\{%\s*endfor\s*%\}/g,
            (match, item, collection, body) => {
                const data = mockData[collection] || [{}, {}];
                return data.map((itemData, idx) => {
                    let result = body;
                    // Replace {{ item.property }} with mock values
                    result = result.replace(new RegExp(`\\{\\{\\s*${item}\\.(\\w+)\\s*\\}\\}`, 'g'), 
                        (m, prop) => itemData[prop] !== undefined ? itemData[prop] : `[${item}.${prop}]`);
                    return result;
                }).join('\n');
            });
        
        // 9. Process n:if and {% if %}
        html = html.replace(/<n:if\s+condition="([^"]*)">([\s\S]*?)<\/n:if>/g, '$2');

        // Enhanced {% if %} processing: Handle basic empty checks
        html = html.replace(/\{%\s*if\s+([\s\S]+?)\s*%\}([\s\S]*?)\{%\s*endif\s*%\}/g, (match, condition, body) => {
            condition = condition.trim();
            // Heuristic: If condition checks for == 0 or empty, assume false (hide content)
            if (condition.includes('== 0') || condition.includes('is empty')) {
                return ''; 
            }
            // Otherwise assume true (show content)
            return body;
        });
        
        // 9b. Substitute remaining {{ variable.property }} or {{ variable.method() }} with mock data
        html = html.replace(/(\{\{|\{)\s*(\w+)\.(\w+)(?:\(\))?\s*(\}\}|\})/g, (match, open, obj, prop, close) => {
            if (mockData[obj]) {
                // Handle .len() specifically for arrays
                if (prop === 'len' && Array.isArray(mockData[obj])) {
                        return mockData[obj].length;
                }
                // Handle standard properties
                if (mockData[obj][prop] !== undefined) {
                    return mockData[obj][prop];
                }
            }
            return match; // Keep original if no mock data
        });
        
        // 9c. Substitute simple {{ variable }} or { variable } (Neutron style)
        html = html.replace(/(\{\{|\{)\s*(\w+)\s*(\}\}|\})/g, (match, open, varName, close) => {
            if (mockData[varName] !== undefined && typeof mockData[varName] !== 'object') {
                return mockData[varName];
            }
            return match;
        });
        
        // 10a. Process INLINE n:island tags (V3 Syntax)
        // This handles <n:island client:load>...<n:script>...</n:script>...</n:island>
        html = html.replace(/<n:island([^>]*)>([\s\S]*?)<\/n:island>/g, (match, attrs, content) => {
            try {
                // The content includes n:script, HTML with {bindings}, and onclick handlers
                // Wrap it as a minimal Neutron source for transpileNeutron
                const fakeSource = `use nucleus_std::neutron::*;\n${content}`;
                const { html: islandHtml, script } = transpileNeutron(fakeSource);
                
                const hydrate = attrs.match(/client:(\w+)/)?.[1] || 'load';
                return `<div data-island="inline" data-hydrate="${hydrate}">${islandHtml}<script>${script}</script></div>`;
            } catch (e) {
                console.error('Island compile error:', e);
                return `<div class="p-4 bg-red-50 text-red-600 rounded border border-red-200">Error compiling island: ${e.message}</div>`;
            }
        });
        
        // 10b. Process n:island with src attribute (External File Reference)
        html = html.replace(/<n:island\s+src="([^"]*)"([^>]*)\/>/g, (match, src, attrs) => {
             // 1. Try to find the file in the project
             let fileKey = src;
             if (!files[fileKey] && files[fileKey + '.ncl']) fileKey += '.ncl';
             if (!files[fileKey] && files[fileKey + '.rs']) fileKey += '.rs';
             
             const file = files[fileKey];
             if (file && (fileKey.endsWith('.ncl') || fileKey.endsWith('.rs')) && file.content.includes('nucleus_std::neutron')) {
                 try {
                     const { html: islandsHtml, script } = transpileNeutron(file.content);
                     const safeSrc = src.replace(/\//g, '-');
                     return `<div data-island="${safeSrc}">${islandsHtml}<script>${script}</script></div>`;
                 } catch (e) {
                     return `<div class="p-4 bg-red-50 text-red-600 rounded">Error compiling island: ${e.message}</div>`;
                 }
             }

             // Fallback to Generic Placeholder
             const hydrate = attrs.match(/client:(\w+)/)?.[1];
             const hydrateAttr = hydrate ? ` data-hydrate="${hydrate}"` : '';
             const comment = hydrate ? ` (hydrate: ${hydrate})` : '';
             return `<div data-island="${src}"${hydrateAttr}><!-- Island: ${src}${comment} --></div>`;
        });
        
        // 11. Process n:link
        html = html.replace(/<n:link\s+href="([^"]*)"[^>]*>([\s\S]*?)<\/n:link>/g, 
            '<a href="$1" data-prefetch="true">$2</a>');
        
        // 12. Process n:image
        html = html.replace(/<n:image\s+src="([^"]*)"[^>]*alt="([^"]*)"[^>]*\/>/g,
            '<img src="$1" alt="$2" loading="lazy" decoding="async" />');
        
        // 13. Process n:model (comment only, server-side)
        html = html.replace(/<n:model[^>]*\/>/g, '<!-- data model binding -->');
        
        // 14. Process n:client (client-side scripts)
        html = html.replace(/<n:client>/g, '<script>');
        html = html.replace(/<\/n:client>/g, '</script>');
        
        // 15. Process n:script (server-side, remove)
        html = html.replace(/<n:script>[\s\S]*?<\/n:script>/g, '');
        
        // 16. Process n:load (server data loading, remove)
        html = html.replace(/<n:load>[\s\S]*?<\/n:load>/g, '');
        
        // 17. Process n:form and n:step
        // Injects a script that intercepts submitting and posts a message to parent
        html = html.replace(/<n:form([^>]*)>/g, (match, attrs) => {
            const action = attrs.match(/action="([^"]*)"/)?.[1] || '';
            const onsubmit = `event.preventDefault(); 
            const formData = Object.fromEntries(new FormData(event.target));
            window.parent.postMessage({ type: 'form:submit', action: '${action}', formData }, '*');`;
            
            return `<form${attrs} onsubmit="${onsubmit.replace(/\n/g, '')}">`;
        });
        html = html.replace(/<\/n:form>/g, '</form>');
        html = html.replace(/<n:step\s+id="([^"]*)"\s+title="([^"]*)">/g, 
            '<fieldset class="wizard-step" data-step="$1"><legend class="text-lg font-semibold mb-4">$2</legend>');
        html = html.replace(/<\/n:step>/g, '</fieldset>');
        
        // 18. Process n:field
        html = html.replace(/<n:field[^>]*label="([^"]*)"[^>]*>([\s\S]*?)<\/n:field>/g, 
            '<div class="form-group mb-4"><label class="block text-sm font-medium mb-1">$1</label>$2</div>');
        html = html.replace(/<n:field[^>]*label="([^"]*)"[^>]*\/>/g, 
            '<div class="form-group mb-4"><label class="block text-sm font-medium mb-1">$1</label><input class="w-full border border-gray-300 rounded-lg px-3 py-2 focus:ring-2 focus:ring-indigo-500 outline-none"></div>');
        
        // --- Component Rendering ---
        
        // 19. StatCard component
        html = html.replace(/<StatCard\s+([^>]*)\/>/g, (match, attrs) => {
            const value = attrs.match(/value="([^"]*)"/)?.[1] || '';
            const label = attrs.match(/label="([^"]*)"/)?.[1] || '';
            const trend = attrs.match(/trend="([^"]*)"/)?.[1] || '';
            const trendClass = trend.startsWith('+') ? 'text-emerald-500' : trend.startsWith('-') ? 'text-rose-500' : 'text-gray-500';
            return `<div class="bg-white p-6 rounded-xl border border-gray-100 shadow-sm">
                <div class="text-gray-500 text-xs mb-1 uppercase font-semibold tracking-wider">${label}</div>
                <div class="text-3xl font-bold text-gray-900">${value}</div>
                ${trend ? `<div class="${trendClass} text-sm mt-2 font-medium">${trend}</div>` : ''}
            </div>`;
        });
        
        // 20. FeatureCard component
        html = html.replace(/<FeatureCard\s+([^>]*)\/>/g, (match, attrs) => {
            const icon = attrs.match(/icon="([^"]*)"/)?.[1] || '🚀';
            const title = attrs.match(/title="([^"]*)"/)?.[1] || '';
            const description = attrs.match(/description="([^"]*)"/)?.[1] || '';
            return `<div class="p-6 bg-slate-800/50 rounded-2xl border border-slate-700/50">
                <div class="w-12 h-12 bg-indigo-500/20 rounded-xl flex items-center justify-center text-2xl mb-4">${icon}</div>
                <h3 class="text-xl font-bold mb-2 text-white">${title}</h3>
                <p class="text-slate-400">${description}</p>
            </div>`;
        });
        
        // 21. Badge component
        html = html.replace(/<Badge\s+([^>]*)>([\s\S]*?)<\/Badge>/g, (match, attrs, content) => {
            const variant = attrs.match(/variant="([^"]*)"/)?.[1] || 'default';
            const icon = attrs.match(/icon="([^"]*)"/)?.[1] || '';
            const bgClass = variant === 'primary' ? 'bg-indigo-500/10 text-indigo-400' : 'bg-gray-500/10 text-gray-400';
            return `<span class="inline-block py-1 px-3 rounded-full ${bgClass} text-sm font-medium">${icon ? icon + ' ' : ''}${content}</span>`;
        });
        
        // 22. Card component (with children)
        html = html.replace(/<Card\s*([^>]*)>([\s\S]*?)<\/Card>/g, (match, attrs, content) => {
            const variant = attrs.match(/variant="([^"]*)"/)?.[1] || 'default';
            const glass = attrs.match(/glass="true"/);
            let classes = 'bg-white rounded-xl shadow-sm p-6 border border-gray-100';
            if (variant === 'glass' || glass) classes = 'bg-black/90 backdrop-blur-xl rounded-xl p-6 text-white border border-white/10';
            if (variant === 'feature') classes = 'bg-gradient-to-br from-purple-600 to-blue-600 rounded-xl p-6 text-white';
            return `<div class="${classes}">${content}</div>`;
        });
        
        // 23. Button component (with children)
        html = html.replace(/<Button\s*([^>]*)>([\s\S]*?)<\/Button>/g, (match, attrs, content) => {
            const variant = attrs.match(/variant="([^"]*)"/)?.[1] || 'primary';
            const size = attrs.match(/size="([^"]*)"/)?.[1] || 'md';
            const href = attrs.match(/href="([^"]*)"/)?.[1] || '';
            const type = attrs.match(/type="([^"]*)"/)?.[1] || 'button';
            const onclick = attrs.match(/onclick="([^"]*)"/)?.[1] || '';
            const id = attrs.match(/id="([^"]*)"/)?.[1] || '';
            
            let sizeClass = size === 'sm' ? 'px-3 py-1 text-sm' : size === 'lg' ? 'px-6 py-3 text-lg' : 'px-4 py-2';
            let variantClass = variant === 'primary' ? 'bg-indigo-600 text-white hover:bg-indigo-700' :
                               variant === 'secondary' ? 'bg-white text-gray-700 border border-gray-300 hover:bg-gray-50' :
                               variant === 'ghost' ? 'text-gray-600 hover:bg-gray-100' :
                               variant === 'gradient' ? 'bg-gradient-to-r from-indigo-500 to-purple-500 text-white' :
                               'bg-indigo-600 text-white hover:bg-indigo-700';
            
            const classes = `${sizeClass} ${variantClass} rounded-lg font-medium transition shadow-sm inline-flex items-center gap-2`;
            const onclickAttr = onclick ? ` onclick="${onclick}"` : '';
            const idAttr = id ? ` id="${id}"` : '';
            
            if (href) {
                return `<a href="${href}"${idAttr} class="${classes}"${onclickAttr}>${content}</a>`;
            }
            return `<button type="${type}"${idAttr} class="${classes}"${onclickAttr}>${content}</button>`;
        });
        
        // 24. TextInput component  
        html = html.replace(/<TextInput\s+([^>]*)\/>/g, (match, attrs) => {
            const name = attrs.match(/name="([^"]*)"/)?.[1] || '';
            const type = attrs.match(/type="([^"]*)"/)?.[1] || 'text';
            const label = attrs.match(/label="([^"]*)"/)?.[1] || name;
            const placeholder = attrs.match(/placeholder="([^"]*)"/)?.[1] || '';
            const required = attrs.includes('required="true"') ? 'required' : '';
            const help = attrs.match(/help="([^"]*)"/)?.[1] || '';
            const error = attrs.match(/error="([^"]*)"/)?.[1] || '';
            const variant = attrs.match(/variant="([^"]*)"/)?.[1] || 'default';
            const size = attrs.match(/size="([^"]*)"/)?.[1] || 'medium';
            const icon = attrs.match(/icon="([^"]*)"/)?.[1] || '';
            const disabled = attrs.includes('disabled="true"') ? 'disabled' : '';
            const value = attrs.match(/value="([^"]*)"/)?.[1] || '';
            const dependsOn = attrs.match(/depends_on="([^"]*)"/)?.[1] || '';
            
            // Size classes
            const sizeClasses = {
                small: 'px-3 py-1.5 text-sm',
                medium: 'px-4 py-2',
                large: 'px-5 py-3 text-lg'
            };
            const sizeClass = sizeClasses[size] || sizeClasses.medium;
            
            // Variant classes
            let inputClasses = `w-full ${sizeClass} focus:ring-2 focus:ring-indigo-500 outline-none transition`;
            if (variant === 'filled') {
                inputClasses += ' bg-gray-100 border-0 rounded-lg focus:bg-white';
            } else if (variant === 'underline') {
                inputClasses += ' border-0 border-b-2 border-gray-300 rounded-none bg-transparent focus:border-indigo-500';
            } else {
                inputClasses += ` border ${error ? 'border-red-300' : 'border-gray-300'} rounded-lg`;
            }
            
            if (disabled) inputClasses += ' bg-gray-100 text-gray-500 cursor-not-allowed';
            
            const dependsAttr = dependsOn ? `data-depends-on="${dependsOn}"` : '';
            const iconHtml = icon ? `<span class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400">${icon}</span>` : '';
            const inputPadding = icon ? 'pl-10' : '';
            
            return `<div class="mb-4" ${dependsAttr}>
                <label class="block text-sm font-medium mb-1 ${error ? 'text-red-600' : ''}">${label}${required ? ' *' : ''}</label>
                <div class="relative">
                    ${iconHtml}
                    <input type="${type}" name="${name}" placeholder="${placeholder}" ${required} ${disabled}
                        ${value ? `value="${value}"` : ''}
                        class="${inputClasses} ${inputPadding}" />
                </div>
                ${help ? `<p class="mt-1 text-sm text-gray-500">${help}</p>` : ''}
                ${error ? `<p class="mt-1 text-sm text-red-600">${error}</p>` : ''}
            </div>`;
        });

        
        // 25. Select component
        html = html.replace(/<Select\s+([^>]*)>([\s\S]*?)<\/Select>/g, (match, attrs, options) => {
            const name = attrs.match(/name="([^"]*)"/)?.[1] || '';
            const label = attrs.match(/label="([^"]*)"/)?.[1] || name;
            const required = attrs.includes('required="true"') ? 'required' : '';
            const error = attrs.match(/error="([^"]*)"/)?.[1] || '';
            
            const borderClass = error ? 'border-red-300' : 'border-gray-300';
            
            return `<div class="mb-4">
                <label class="block text-sm font-medium mb-1 ${error ? 'text-red-600' : ''}">${label}${required ? ' *' : ''}</label>
                <select name="${name}" ${required} class="w-full px-4 py-2 border ${borderClass} rounded-lg focus:ring-2 focus:ring-indigo-500 outline-none bg-white">
                    ${options}
                </select>
                ${error ? `<p class="mt-1 text-sm text-red-600">${error}</p>` : ''}
            </div>`;
        });

        
        // 26. Checkbox component
        html = html.replace(/<Checkbox\s+([^>]*)\/>/g, (match, attrs) => {
            const name = attrs.match(/name="([^"]*)"/)?.[1] || '';
            const label = attrs.match(/label="([^"]*)"/)?.[1] || '';
            const required = attrs.includes('required="true"') ? 'required' : '';
            const checked = attrs.includes('checked="true"') ? 'checked' : '';
            const variant = attrs.match(/variant="([^"]*)"/)?.[1] || 'default';
            
            if (variant === 'toggle') {
                return `<label class="flex items-center gap-3 cursor-pointer mb-4">
                    <div class="relative">
                        <input type="checkbox" name="${name}" ${required} ${checked} class="sr-only peer" />
                        <div class="w-11 h-6 bg-gray-200 rounded-full peer peer-checked:bg-indigo-600 transition"></div>
                        <div class="absolute left-1 top-1 w-4 h-4 bg-white rounded-full transition peer-checked:translate-x-5"></div>
                    </div>
                    <span class="text-sm">${label}</span>
                </label>`;
            }
            return `<label class="flex items-center gap-2 mb-4 cursor-pointer">
                <input type="checkbox" name="${name}" ${required} ${checked} class="w-4 h-4 rounded text-indigo-600 focus:ring-indigo-500 border-gray-300" />
                <span class="text-sm">${label}</span>
            </label>`;
        });
        
        // 27. FormGroup component
        html = html.replace(/<FormGroup\s+([^>]*)>([\s\S]*?)<\/FormGroup>/g, (match, attrs, content) => {
            const legend = attrs.match(/legend="([^"]*)"/)?.[1] || '';
            const columns = attrs.match(/columns="([^"]*)"/)?.[1] || '1';
            
            return `<fieldset class="mb-6">
                ${legend ? `<legend class="text-lg font-semibold mb-4">${legend}</legend>` : ''}
                <div class="grid grid-cols-${columns} gap-4">${content}</div>
            </fieldset>`;
        });
        
        // 28. NavItem component
        html = html.replace(/<NavItem\s+([^>]*)>([\s\S]*?)<\/NavItem>/g, (match, attrs, content) => {
            const href = attrs.match(/href="([^"]*)"/)?.[1] || '#';
            const icon = attrs.match(/icon="([^"]*)"/)?.[1] || '';
            const active = attrs.includes('active="true"');
            const classes = active 
                ? 'flex items-center gap-3 px-4 py-3 bg-indigo-50 text-indigo-700 rounded-lg font-medium'
                : 'flex items-center gap-3 px-4 py-3 text-gray-600 hover:bg-gray-50 rounded-lg font-medium transition';
            return `<a href="${href}" class="${classes}">${icon ? `<span>${icon}</span>` : ''}${content}</a>`;
        });
        
        // 29. Process template interpolation {{ var }}
        html = html.replace(/\{\{\s*(\w+)\s*\}\}/g, '[$1]');
        html = html.replace(/\{\{\s*(\w+)\.(\w+)\s*\}\}/g, '[$1.$2]');
        
        // 30. Process n:include (component imports - show as comments)
        html = html.replace(/<n:include\s+src="([^"]*)"[^>]*\/>/g, 
            '<!-- Import: $1 -->');
        
        // 33. Process n:model with multi-line attributes
        html = html.replace(/<n:model\s+[\s\S]*?\/>/g, '<!-- data model binding -->');
        
        // 34. Process single-brace variable interpolation {var.prop} with demo values
        const demoValues = {
            'stats.total_users': '1,234',
            'stats.revenue': '$45,678',
            'stats.orders': '892',
            'user.name': 'Alice Johnson',
            'user.email': 'alice@example.com',
            'user.avatar': 'https://i.pravatar.cc/40?1',
            'user.role': 'Admin',
            'post.title': 'Getting Started with Nucleus',
            'post.slug': 'getting-started',
            'post.excerpt': 'Learn how to build modern web apps...',
            'post.author': 'John Doe',
            'post.date': 'Jan 8, 2026',
            'post.cover_image': 'https://picsum.photos/400/200',
            'post.category': 'Tutorial',
            'featured.title': 'Build Faster with Nucleus',
            'featured.slug': 'nucleus-v3-release',
            'featured.excerpt': 'Discover the new features in Nucleus...',
            'todo.id': '1',
            'todo.title': 'Learn Nucleus',
            'todo.completed': 'false',
            'count': '0'
        };
        
        // Replace known variables with demo values
        Object.entries(demoValues).forEach(([key, value]) => {
            html = html.replace(new RegExp(`\\{${key.replace('.', '\\.')}\\}`, 'g'), value);
        });
        
        // Replace remaining {var.prop} with [var.prop] placeholder
        html = html.replace(/\{(\w+)\.(\w+)\}/g, '[$1.$2]');
        // Replace remaining {var} with [var] placeholder  
        html = html.replace(/\{(\w+)\}/g, '[$1]');

        const fullHtml = `
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    ${useTailwind ? `<script>
    (function() {
        const originalWarn = console.warn;
        console.warn = function(...args) {
            if (args[0] && typeof args[0] === 'string' && args[0].includes('cdn.tailwindcss.com')) return;
            originalWarn.apply(console, args);
        };
    })();
    </script>
    <script src="https://cdn.tailwindcss.com"></script>` : ''}
    ${resources.css.map(url => `<link rel="stylesheet" href="${url}">`).join('\n')}
    <style>
        ${css}
        body { background: transparent; }
        .form-group { margin-bottom: 1rem; }
        
        /* Island Placeholder Styles */
        [data-island] {
            padding: 1.5rem;
            border: 2px dashed #6366f1;
            border-radius: 0.75rem;
            background: #eef2ff;
            color: #4338ca;
            text-align: center;
            font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
            margin: 1.5rem 0;
            position: relative;
        }
        [data-island]::before {
            content: "Neutron Island: " attr(data-island);
            display: block;
            font-weight: 700;
            font-size: 0.75rem;
            margin-bottom: 0.5rem;
            text-transform: uppercase;
            letter-spacing: 0.05em;
            opacity: 0.8;
        }
        [data-island][data-hydrate]::after {
            content: "Hydration Strategy: " attr(data-hydrate);
            display: block;
            font-size: 0.75rem;
            margin-top: 0.5rem;
            opacity: 0.7;
        }
    </style>
    ${resources.js.map(url => `<script src="${url}"></script>`).join('\n')}
</head>
<body>
    ${html}
    <script>
        ${js}
    </script>
</body>
</html>`;

        const frame = document.getElementById('preview-frame');
        // Prevent refresh flicker if frame is same? No, always update for now.
        frame.srcdoc = fullHtml;
        
        const outputCode = document.getElementById('html-output').querySelector('code');
        if (outputCode) outputCode.textContent = fullHtml;
        
        updateStatus('ready');
    } catch (err) {
        console.error(err);
        updateStatus('error');
    }
}

function updateStatus(status) {
    const indicator = document.getElementById('status-indicator');
    if (indicator) {
        indicator.className = `status-indicator status-${status}`;
        indicator.textContent = status.charAt(0).toUpperCase() + status.slice(1);
    }
}

// Global Keybinds
document.addEventListener('keydown', (e) => {
    if ((e.metaKey || e.ctrlKey) && e.key === 's') {
        e.preventDefault();
        saveState();
        const footerStatus = document.getElementById('compile-status');
        const original = footerStatus.textContent;
        footerStatus.textContent = "Saved!";
        setTimeout(() => footerStatus.textContent = original, 2000);
    }
    if ((e.metaKey || e.ctrlKey) && e.key === 'Enter') {
        e.preventDefault();
        compile();
    }
});

// Modal Helpers
window.closeModal = function(id) {
    const el = document.getElementById(id);
    if (el.tagName === 'DIALOG') el.close();
    else el.classList.add('hidden');
}

window.openModal = function(id) {
    const el = document.getElementById(id);
    if (el.tagName === 'DIALOG') el.showModal();
    else el.classList.remove('hidden');
}

// Verification Helpers (Expanded)
window.runTests = async function() {
    console.log("🧪 Running Playground Tests...");
    const results = [];
    
    // Test 1: VFS Logic
    files['test.txt'] = { content: 'hello', language: 'text' };
    results.push({ test: "VFS: Create File", passed: files['test.txt'].content === 'hello' });
    
    // Test 2: VFS Persistence
    saveState();
    const savedFiles = JSON.parse(localStorage.getItem('playground-files'));
    results.push({ test: "VFS: Persistence", passed: savedFiles['test.txt'].content === 'hello' });
    
    // Cleanup
    delete files['test.txt'];
    saveState();

    // Test 3: Resource Persistence
    const safeCss = 'data:text/css,.test{color:red}';
    resources.css.push(safeCss);
    savePreferences();
    const loaded = JSON.parse(localStorage.getItem('playground-prefs'));
    results.push({ test: "Resources Persist", passed: loaded.resources.css.includes(safeCss) });
    resources.css.pop(); // cleanup

    // Test 4: Versioning
    saveVersion('test-template', '<h1>Test</h1>', '.test{}');
    const version = getSavedVersion('test-template');
    results.push({ test: "Versioning Logic", passed: version && version.ncl === '<h1>Test</h1>' });
    
    // Test 5: Template Integrity
    try {
        if (EXAMPLES.hello.ncl.length > 0) {
             results.push({ test: "Template Library Integrity", passed: true });
        }
    } catch(e) { results.push({ test: "Template Library Error", passed: false, error: e }); }
    
    console.table(results);
    return results;
};



