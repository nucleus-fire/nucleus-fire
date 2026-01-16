# Internationalization (i18n) Guide

Translate your app with Polyglot - supporting multiple locales, pluralization, and formatting.

---

## Quick Start

```rust
use nucleus_std::polyglot::Polyglot;

let mut i18n = Polyglot::new("en");

// Add translations
i18n.add("greeting", "Hello!");
i18n.add("welcome", "Welcome, {{name}}!");

// Use translations
let msg = i18n.t("greeting"); // "Hello!"
let welcome = i18n.t_with("welcome", &[("name", "Alice")]); // "Welcome, Alice!"
```

---

## Loading Translations

### From JSON

```rust
let mut i18n = Polyglot::new("en");

i18n.load_json(r#"{
    "nav": {
        "home": "Home",
        "about": "About Us",
        "contact": "Contact"
    },
    "greeting": "Hello, {{name}}!"
}"#)?;

// Access nested keys with dot notation
let home = i18n.t("nav.home"); // "Home"
```

### From TOML

```rust
i18n.load_toml(r#"
[nav]
home = "Home"
about = "About Us"

[messages]
welcome = "Welcome back!"
"#)?;
```

### From Files

```rust
let json = std::fs::read_to_string("locales/en.json")?;
i18n.load_json(&json)?;
```

---

## Interpolation

```rust
i18n.add("order", "Order #{{id}} for {{amount}}");

// With tuple array
let msg = i18n.t_with("order", &[
    ("id", "12345"),
    ("amount", "$99.00")
]);
// "Order #12345 for $99.00"

// With HashMap
let mut args = HashMap::new();
args.insert("id".to_string(), "12345".to_string());
args.insert("amount".to_string(), "$99.00".to_string());

let msg = i18n.t_map("order", &args);
```

---

## Pluralization

Define rules for singular, plural, and special cases.

```rust
i18n.add("items.zero", "No items");
i18n.add("items.one", "{{count}} item");
i18n.add("items.few", "{{count}} items");  // 2-4 in some languages
i18n.add("items.many", "{{count}} items");

i18n.plural("items", 0);  // "No items"
i18n.plural("items", 1);  // "1 item"
i18n.plural("items", 5);  // "5 items"
```

---

## Number Formatting

```rust
use nucleus_std::polyglot::{Polyglot, NumberStyle};

let en = Polyglot::new("en");
let de = Polyglot::new("de");

// Decimal
en.format_number(1234.56, NumberStyle::Decimal);  // "1,234.56"
de.format_number(1234.56, NumberStyle::Decimal);  // "1.234,56"

// Currency
en.format_number(1234.56, NumberStyle::Currency); // "$1,234.56"

// Percentage
en.format_number(0.1234, NumberStyle::Percent);   // "12.34%"
```

---

## Date Formatting

```rust
use nucleus_std::polyglot::{Polyglot, DateStyle};

let i18n = Polyglot::new("en");
let timestamp = 1703500800; // Unix timestamp

i18n.format_date(timestamp, DateStyle::Short);  // "12/25/23"
i18n.format_date(timestamp, DateStyle::Medium); // "Dec 25, 2023"
i18n.format_date(timestamp, DateStyle::Long);   // "December 25, 2023"
```

---

## Fallback Locales

```rust
let mut i18n = Polyglot::new("fr");

// Set English as fallback
let mut fallback = HashMap::new();
fallback.insert("greeting".to_string(), "Hello".to_string());
i18n.set_fallback("en", fallback);

// If key missing in French, uses English
let msg = i18n.t("greeting"); // "Hello"
```

---

## RTL Support

```rust
let ar = Polyglot::new("ar");
let en = Polyglot::new("en");

ar.is_rtl(); // true
en.is_rtl(); // false

// Use in templates
if i18n.is_rtl() {
    html_dir = "rtl";
}
```

---

## Locale Switching

```rust
let mut i18n = Polyglot::new("en");
// Load English translations...

// User changes language
i18n.set_locale("es");
// Load Spanish translations...

// Check current locale
println!("Current: {}", i18n.locale()); // "es"
```

---

## Organization Pattern

```
locales/
├── en.json
├── es.json
├── fr.json
└── de.json
```

```rust
fn load_locale(name: &str) -> Result<Polyglot, Error> {
    let path = format!("locales/{}.json", name);
    let content = std::fs::read_to_string(&path)?;
    
    let mut i18n = Polyglot::new(name);
    i18n.load_json(&content)?;
    
    Ok(i18n)
}
```

---

## Supported Locales

| Locale | Separators | Currency | RTL |
|--------|-----------|----------|-----|
| en | 1,234.56 | $ | No |
| en-GB | 1,234.56 | £ | No |
| de | 1.234,56 | € | No |
| fr | 1.234,56 | € | No |
| ja | 1,234.56 | ¥ | No |
| ar | - | - | Yes |
| he | - | - | Yes |
