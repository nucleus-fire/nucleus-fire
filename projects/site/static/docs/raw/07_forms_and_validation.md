# Forms & Validation Guide

> Build complex, accessible forms with schema-driven validation, multi-step wizards, and reusable components.

## Quick Start

### Simple Form

```xml
<n:form action="/login" method="POST">
  <TextInput name="email" type="email" label="Email" required="true" />
  <TextInput name="password" type="password" label="Password" required="true" />
  <Button type="submit">Sign In</Button>
</n:form>
```

### Schema-Driven Form

```rust
use nucleus_std::forms::{FormSchema, Field};

let form = FormSchema::new("registration")
    .action("/register")
    .field(Field::email("email").label("Email").required())
    .field(Field::password("password").label("Password").required().min(8.0))
    .field(Field::number("age").label("Age").min(18.0).max(120.0))
    .submit("Create Account");

// Render to HTML
let html = form.render();

// Validate form data
let result = nucleus_std::forms::validate(&form, &form_data);
```

---

## Form Components

Nucleus provides ready-to-use form components that integrate seamlessly with `<n:form>`:

### TextInput

Enhanced text input with validation states, icons, and variants.

```xml
<TextInput 
  name="email" 
  type="email" 
  label="Email Address"
  placeholder="you@example.com"
  required="true"
  help="We'll never share your email"
/>

<!-- With error state -->
<TextInput 
  name="username" 
  label="Username"
  error="Username is already taken"
/>

<!-- With icon -->
<TextInput 
  name="search" 
  icon="🔍"
  placeholder="Search..."
/>

<!-- Variants -->
<TextInput name="field1" variant="default" />
<TextInput name="field2" variant="filled" />
<TextInput name="field3" variant="underline" />

<!-- Sizes -->
<TextInput name="small" size="small" />
<TextInput name="medium" size="medium" />
<TextInput name="large" size="large" />
```

**Props:**

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `name` | string | required | Field name |
| `type` | string | "text" | Input type (text, email, password, number, tel, url) |
| `label` | string | - | Field label |
| `placeholder` | string | - | Placeholder text |
| `required` | boolean | false | Mark as required |
| `disabled` | boolean | false | Disable input |
| `error` | string | - | Error message to display |
| `help` | string | - | Help text below input |
| `icon` | string | - | Icon to show in input |
| `size` | string | "medium" | Size: small, medium, large |
| `variant` | string | "default" | Style: default, filled, underline |

---

### Select

Dropdown selection with accessibility features.

```xml
<Select name="country" label="Country" required="true">
  <option value="us">United States</option>
  <option value="uk">United Kingdom</option>
  <option value="ca">Canada</option>
</Select>

<!-- With error -->
<Select name="role" label="Role" error="Please select a role">
  <option value="admin">Administrator</option>
  <option value="user">User</option>
</Select>
```

**Props:**

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `name` | string | required | Field name |
| `label` | string | - | Field label |
| `placeholder` | string | "Select an option..." | Placeholder |
| `required` | boolean | false | Mark as required |
| `disabled` | boolean | false | Disable select |
| `error` | string | - | Error message |
| `multiple` | boolean | false | Allow multiple selection |

---

### Checkbox

Styled checkbox with toggle variant.

```xml
<!-- Standard checkbox -->
<Checkbox name="terms" label="I agree to the Terms of Service" required="true" />

<!-- Toggle switch -->
<Checkbox name="notifications" label="Enable notifications" variant="toggle" />

<!-- Checked by default -->
<Checkbox name="newsletter" label="Subscribe to newsletter" checked="true" />
```

**Props:**

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `name` | string | required | Field name |
| `label` | string | required | Checkbox label |
| `checked` | boolean | false | Default checked state |
| `disabled` | boolean | false | Disable checkbox |
| `error` | string | - | Error message |
| `variant` | string | "default" | Style: default, toggle |

---

### FormGroup

Layout wrapper for organizing fields in grids.

```xml
<FormGroup legend="Personal Information" columns="2">
  <TextInput name="firstName" label="First Name" />
  <TextInput name="lastName" label="Last Name" />
</FormGroup>

<FormGroup legend="Address" columns="3" gap="1.5rem">
  <TextInput name="city" label="City" />
  <TextInput name="state" label="State" />
  <TextInput name="zip" label="ZIP Code" />
</FormGroup>
```

**Props:**

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `legend` | string | - | Group title |
| `columns` | number | 1 | Number of columns (1-4) |
| `gap` | string | "1rem" | Gap between fields |

---

## Multi-Step Wizard

Create multi-step forms with progress tracking and validation.

### Template Syntax

```xml
<n:form action="/register" wizard="true">
  <n:step id="account" title="Account">
    <TextInput name="email" type="email" label="Email" required="true" />
    <TextInput name="password" type="password" label="Password" required="true" />
  </n:step>
  
  <n:step id="profile" title="Profile">
    <TextInput name="name" label="Full Name" required="true" />
    <TextInput name="phone" type="tel" label="Phone" />
  </n:step>
  
  <n:step id="confirm" title="Confirm">
    <p>Review your information and submit.</p>
    <Checkbox name="terms" label="I agree to the terms" required="true" />
  </n:step>
</n:form>
```

### Schema-Driven Wizard

```rust
use nucleus_std::forms::{FormSchema, Field, WizardStep};

let wizard = FormSchema::new("onboarding")
    .action("/onboard")
    .step(WizardStep::new("account", "Create Account")
        .description("Enter your login credentials")
        .field(Field::email("email").label("Email").required())
        .field(Field::password("password").label("Password").required().min(8.0)))
    .step(WizardStep::new("profile", "Your Profile")
        .field(Field::text("name").label("Full Name").required())
        .field(Field::component("avatar", "AvatarUpload")))  // Custom component!
    .step(WizardStep::new("preferences", "Preferences")
        .field(Field::new("theme", FieldType::Select)
            .label("Theme")
            .options(vec![("light", "Light"), ("dark", "Dark")])));

let html = wizard.render();
```

### Wizard Features

- ✅ Automatic progress indicator
- ✅ Step-by-step validation (validates before next)
- ✅ SessionStorage persistence (survives refresh)
- ✅ Accessible (ARIA attributes, keyboard navigation)
- ✅ Custom step conditions

---

## Validation

### Built-in Rules

| Rule | Usage | Description |
|------|-------|-------------|
| `required` | `.required()` | Field must have value |
| `email` | `.validate(Email)` | Valid email format |
| `min` | `.min(18.0)` | Minimum value or length |
| `max` | `.max(100.0)` | Maximum value or length |
| `pattern` | `.pattern("^[A-Z]", Some("Must start with capital"))` | Regex match |
| `in` | `.validate(In { values: vec!["a", "b"] })` | Value in list |
| `confirmed` | `.validate(Confirmed)` | Matches `{field}_confirmation` |

### Server-Side Validation

```rust
use nucleus_std::forms::{validate, FormSchema, Field};

async fn handle_register(Form(data): Form<HashMap<String, String>>) -> impl IntoResponse {
    let schema = FormSchema::new("register")
        .field(Field::email("email").required())
        .field(Field::password("password").required().min(8.0));
    
    let result = validate(&schema, &data);
    
    if !result.valid {
        // Return errors to form
        return render_form_with_errors(&schema, &data, &result.errors);
    }
    
    // Process valid data...
    Redirect::to("/dashboard")
}
```

### Client-Side Validation

Forms automatically include client-side validation using HTML5 attributes:
- `required`
- `minlength` / `maxlength`
- `min` / `max`
- `pattern`
- `type="email"` / `type="url"` etc.

For custom client validation, add JavaScript:

```xml
<n:form action="/submit" id="myForm">
  <TextInput name="username" label="Username" />
  <script>
    document.getElementById('myForm').addEventListener('submit', (e) => {
      const username = document.getElementById('username').value;
      if (username.includes(' ')) {
        e.preventDefault();
        showError('username', 'Username cannot contain spaces');
      }
    });
  </script>
</n:form>
```

---

## Styling Forms

### CSS Variables

Customize form appearance using CSS variables:

```css
:root {
  /* Colors */
  --primary: #6366f1;
  --error: #ef4444;
  --border: #e2e8f0;
  --bg-input: #ffffff;
  --bg-disabled: #f1f5f9;
  --text-primary: #1a1a2e;
  --text-muted: #64748b;
  
  /* Sizing */
  --input-padding: 0.75rem 1rem;
  --input-radius: 0.5rem;
  --input-font-size: 1rem;
}
```

### Form Classes

| Class | Description |
|-------|-------------|
| `.nucleus-form` | Base form styling |
| `.nucleus-wizard` | Wizard form styling |
| `.form-field` | Individual field wrapper |
| `.form-field.has-error` | Field with error |
| `.field-error` | Error message |
| `.field-help` | Help text |
| `.wizard-progress` | Wizard progress bar |
| `.wizard-step` | Wizard step container |
| `.wizard-nav` | Wizard navigation buttons |

### Custom Component Styling

Override component styles with scoped CSS or global styles:

```css
/* Global override */
.nucleus-form .text-input {
  border-radius: 0;
  border-width: 2px;
}

/* Specific form */
#registration-form .text-input {
  background: #f8fafc;
}
```

---


---

## Advanced Features

### Repeater Fields (Array Support)

Create dynamic forms where users can add multiple items of the same type.

```rust
Field::repeater("addresses", "address-template")
```

The repeater field renders a container that interacts with client-side logic to duplicate the template.

### Field Dependencies

Show or hide fields based on the values of other fields.

```rust
// Show 'company_name' only when 'account_type' is 'business'
Field::text("company_name")
    .depends_on("account_type:business")
```

### Conditional Validation

Apply validation rules only when specific conditions are met.

```rust
Field::text("vat_number")
    .validate_if("required_if:account_type:business")
```

### File Uploads

Enhanced file upload with built-in preview support.

```rust
Field::new("profile_pic", FieldType::File)
    .label("Profile Picture")
    .required()
```

---

## Schema Format

### JSON Schema

```json
{
  "name": "contact",
  "action": "/contact",
  "fields": [
    {
      "name": "email",
      "field_type": "email",
      "label": "Email Address",
      "validations": [
        { "type": "required" },
        { "type": "email" }
      ]
    },
    {
      "name": "message",
      "field_type": "textarea",
      "label": "Message",
      "validations": [
        { "type": "required" },
        { "type": "minLength", "value": 10 }
      ]
    }
  ],
  "submit_text": "Send Message"
}
```

### TOML Schema

```toml
name = "contact"
action = "/contact"
submit_text = "Send Message"

[[fields]]
name = "email"
field_type = "email"
label = "Email Address"

[[fields.validations]]
type = "required"

[[fields.validations]]
type = "email"

[[fields]]
name = "message"
field_type = "textarea"
label = "Message"
```

### Loading Schemas

```rust
use nucleus_std::forms::{load_from_json, load_from_toml};

// From JSON
let schema = load_from_json(include_str!("forms/contact.json"))?;

// From TOML
let schema = load_from_toml(include_str!("forms/contact.toml"))?;

let html = schema.render();
```

---

## Accessibility

Forms are built with accessibility in mind:

- ✅ Proper `<label>` associations via `for` attribute
- ✅ `aria-describedby` linking inputs to errors
- ✅ `aria-invalid` for error states
- ✅ `aria-live="polite"` on error containers for announcements
- ✅ `role="alert"` on error messages
- ✅ Keyboard navigable wizard steps
- ✅ Focus management on step changes
- ✅ Required field indicators

---

## CSRF Protection

CSRF tokens are automatically injected and validated:

```rust
// Config (nucleus.config)
[security]
csrf = true

// Token is auto-injected in <n:form>
// <input type="hidden" name="_csrf" value="..." />
```

---

## reCAPTCHA Integration

Protect forms from bots with Google reCAPTCHA v2 or v3.

### Adding reCAPTCHA to Forms

```rust
use nucleus_std::forms::{FormSchema, Field};

let form = FormSchema::new("contact")
    .action("/contact")
    .field(Field::email("email").required())
    .field(Field::text("message").required())
    .field(Field::recaptcha("recaptcha")
        .prop("siteKey", "YOUR_SITE_KEY")
        .prop("version", "v2")  // or "v3"
        .prop("theme", "light"))  // light or dark
    .submit("Send");
```

### reCAPTCHA v2 (Checkbox)

User clicks "I'm not a robot" checkbox.

```rust
Field::recaptcha("captcha")
    .prop("siteKey", "6LeIxAcTAAAAAJcZVRqyHh71UMIEGNQ_MXjiZKhI")  // Test key
    .prop("version", "v2")
    .prop("theme", "dark")
```

### reCAPTCHA v3 (Invisible)

Score-based verification (no user interaction).

```rust
Field::recaptcha("captcha")
    .prop("siteKey", "YOUR_V3_KEY")
    .prop("version", "v3")
```

### Server-Side Verification

```rust
use nucleus_std::forms::verify_recaptcha;

async fn handle_contact(Form(data): Form<HashMap<String, String>>) -> impl IntoResponse {
    let secret = std::env::var("RECAPTCHA_SECRET").unwrap();
    let token = data.get("recaptcha").unwrap();
    
    let result = verify_recaptcha(&secret, token, None).await?;
    
    if !result.success {
        return Err("reCAPTCHA verification failed");
    }
    
    // For v3, check the score (0.0 = bot, 1.0 = human)
    if let Some(score) = result.score {
        if score < 0.5 {
            return Err("Bot detected");
        }
    }
    
    // Process form...
    Ok(Redirect::to("/thanks"))
}
```

### Environment Variables

```bash
RECAPTCHA_SITE_KEY=your-site-key
RECAPTCHA_SECRET=your-secret-key
```

---

## Styling with Tailwind CSS

Forms can be fully styled with Tailwind CSS classes.

### Using Tailwind Classes

Pass Tailwind classes via the `class` prop on forms and fields:

```rust
let form = FormSchema::new("signup")
    .action("/signup")
    .class("space-y-6 max-w-md mx-auto p-6 bg-white shadow-lg rounded-xl")
    .field(Field::email("email")
        .label("Email")
        .class("focus:ring-2 focus:ring-indigo-500"))
    .submit("Sign Up");
```

### Template Syntax with Tailwind

```xml
<n:form action="/contact" class="space-y-4 p-6 bg-gray-50 rounded-lg">
  <div class="space-y-2">
    <label for="email" class="block text-sm font-medium text-gray-700">
      Email
    </label>
    <input 
      type="email" 
      name="email" 
      id="email"
      class="w-full px-4 py-2 border border-gray-300 rounded-md 
             focus:ring-2 focus:ring-indigo-500 focus:border-transparent
             transition duration-200"
      required
    />
  </div>
  
  <button 
    type="submit" 
    class="w-full py-3 px-4 bg-indigo-600 text-white font-semibold 
           rounded-lg hover:bg-indigo-700 focus:ring-4 focus:ring-indigo-300
           transition duration-200"
  >
    Subscribe
  </button>
</n:form>
```

### Tailwind Form Components

Override default component styles with Tailwind:

```xml
<TextInput 
  name="name" 
  label="Name"
  class="border-2 border-gray-200 rounded-lg focus:border-blue-500"
/>

<Select 
  name="country" 
  label="Country"
  class="bg-gray-100 border-0 rounded-xl"
>
  <option>USA</option>
  <option>UK</option>
</Select>

<Checkbox 
  name="terms" 
  label="I agree"
  class="accent-purple-600"
/>
```

### Tailwind Wizard Styling

```rust
let wizard = FormSchema::new("onboarding")
    .class("max-w-2xl mx-auto")
    .step(WizardStep::new("step1", "Account")
        .field(Field::email("email")
            .class("border-indigo-200 focus:ring-indigo-500")));
```

### CSS Variables with Tailwind

Combine CSS variables with Tailwind's JIT:

```css
/* In your CSS */
:root {
  --form-primary: theme('colors.indigo.600');
  --form-error: theme('colors.red.500');
}

.nucleus-form input:focus {
  @apply ring-2 ring-[var(--form-primary)];
}

.field-error {
  @apply text-sm text-[var(--form-error)] mt-1;
}
```

---

## Complete Example

```xml
<n:view title="Registration">
  <main class="container">
    <h1>Create Account</h1>
    
    <n:form action="/register" method="POST" wizard="true">
      <n:step id="account" title="Account">
        <FormGroup legend="Login Credentials" columns="1">
          <TextInput 
            name="email" 
            type="email" 
            label="Email" 
            placeholder="you@example.com"
            required="true" 
          />
          <TextInput 
            name="password" 
            type="password" 
            label="Password" 
            help="At least 8 characters"
            required="true" 
          />
        </FormGroup>
      </n:step>
      
      <n:step id="profile" title="Profile">
        <FormGroup legend="Personal Info" columns="2">
          <TextInput name="firstName" label="First Name" required="true" />
          <TextInput name="lastName" label="Last Name" required="true" />
        </FormGroup>
        
        <FormGroup legend="Contact" columns="2">
          <TextInput name="phone" type="tel" label="Phone" />
          <Select name="country" label="Country">
            <option value="us">United States</option>
            <option value="uk">United Kingdom</option>
          </Select>
        </FormGroup>
      </n:step>
      
      <n:step id="confirm" title="Confirm">
        <p>Please review and accept our terms.</p>
        <Checkbox name="terms" label="I agree to the Terms of Service" required="true" />
        <Checkbox name="newsletter" label="Subscribe to newsletter" variant="toggle" />
      </n:step>
    </n:form>
  </main>
</n:view>
```

---

## API Reference

### FormSchema

| Method | Description |
|--------|-------------|
| `new(name)` | Create new schema |
| `action(url)` | Set form action URL |
| `field(field)` | Add a field |
| `step(step)` | Add wizard step |
| `submit(text)` | Set submit button text |
| `class(class)` | Set form CSS class |
| `render()` | Generate HTML string |
| `is_wizard()` | Check if wizard mode |
| `all_fields()` | Get all fields |

### Field

| Method | Description |
|--------|-------------|
| `new(name, type)` | Create field |
| `text(name)` | Text field shortcut |
| `email(name)` | Email field shortcut |
| `password(name)` | Password field shortcut |
| `number(name)` | Number field shortcut |
| `component(name, component)` | Custom component field |
| `label(text)` | Set label |
| `placeholder(text)` | Set placeholder |
| `help(text)` | Set help text |
| `default(value)` | Set default value |
| `required()` | Mark required |
| `min(value)` | Set min value/length |
| `max(value)` | Set max value/length |
| `pattern(regex, msg)` | Add regex validation |
| `options(opts)` | Add select options |
| `prop(key, val)` | Add component prop |

### WizardStep

| Method | Description |
|--------|-------------|
| `new(id, title)` | Create step |
| `description(text)` | Set description |
| `field(field)` | Add field to step |
| `when(condition)` | Conditional display |

### Validation

| Function | Description |
|----------|-------------|
| `validate(schema, data)` | Validate form data |
| `load_from_json(json)` | Load schema from JSON |
| `load_from_toml(toml)` | Load schema from TOML |
