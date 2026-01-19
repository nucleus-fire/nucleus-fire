<n:script>
    use nucleus_std::polyglot::{Polyglot, NumberStyle, DateStyle};
    use std::collections::HashMap;

    // ═══════════════════════════════════════════════════════════════════════════
    // EXAMPLE 1: Basic Translations
    // ═══════════════════════════════════════════════════════════════════════════

    fn setup_translations() -> Polyglot {
        let mut i18n = Polyglot::new("en");

        // Load from JSON
        i18n.load_json(r#"{
            "nav": {
                "home": "Home",
                "products": "Products",
                "about": "About Us",
                "contact": "Contact"
            },
            "hero": {
                "title": "Welcome to {{app_name}}",
                "subtitle": "The best solution for your needs"
            },
            "cart": {
                "items.zero": "Your cart is empty",
                "items.one": "{{count}} item in cart",
                "items.many": "{{count}} items in cart"
            }
        }"#).unwrap();

        i18n
    }

    fn render_nav(i18n: &Polyglot) -> String {
        format!(r#"
            <nav>
                <a href="/">{}</a>
                <a href="/products">{}</a>
                <a href="/about">{}</a>
            </nav>
        "#, i18n.t("nav.home"), i18n.t("nav.products"), i18n.t("nav.about"))
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // EXAMPLE 2: Dynamic Locale Switching
    // ═══════════════════════════════════════════════════════════════════════════

    fn load_locale(locale: &str) -> Polyglot {
        let mut i18n = Polyglot::new(locale);

        // Load locale-specific file
        let json = match locale {
            "es" => r#"{"greeting": "¡Hola, {{name}}!"}"#,
            "de" => r#"{"greeting": "Hallo, {{name}}!"}"#,
            "fr" => r#"{"greeting": "Bonjour, {{name}}!"}"#,
            _ => r#"{"greeting": "Hello, {{name}}!"}"#,
        };

        i18n.load_json(json).unwrap();

        // Set English as fallback
        let mut fallback = HashMap::new();
        fallback.insert("greeting".to_string(), "Hello, {{name}}!".to_string());
        fallback.insert("error".to_string(), "An error occurred".to_string());
        i18n.set_fallback("en", fallback);

        i18n
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // EXAMPLE 3: Pluralization
    // ═══════════════════════════════════════════════════════════════════════════

    fn show_cart_count(i18n: &Polyglot, count: i64) -> String {
        i18n.plural("cart.items", count)
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // EXAMPLE 4: Number & Currency Formatting
    // ═══════════════════════════════════════════════════════════════════════════

    fn format_price(locale: &str, price: f64) -> String {
        let i18n = Polyglot::new(locale);
        i18n.format_number(price, NumberStyle::Currency)
    }

    fn format_percentage(locale: &str, value: f64) -> String {
        let i18n = Polyglot::new(locale);
        i18n.format_number(value, NumberStyle::Percent)
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // EXAMPLE 5: Date Formatting by Locale
    // ═══════════════════════════════════════════════════════════════════════════

    fn format_order_date(locale: &str, timestamp: i64) -> String {
        let i18n = Polyglot::new(locale);
        i18n.format_date(timestamp, DateStyle::Long)
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // EXAMPLE 6: RTL Layout Detection
    // ═══════════════════════════════════════════════════════════════════════════

    fn get_html_dir(locale: &str) -> &'static str {
        let i18n = Polyglot::new(locale);
        if i18n.is_rtl() {
            "rtl"
        } else {
            "ltr"
        }
    }

    fn render_page(locale: &str) -> String {
        format!(r#"<!DOCTYPE html>
<html lang="{}" dir="{}">
<head><title>App</title></head>
<body>...</body>
</html>"#, locale, get_html_dir(locale))
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // EXAMPLE 7: Full E-Commerce Page
    // ═══════════════════════════════════════════════════════════════════════════

    fn render_product_card(locale: &str, name: &str, price: f64, stock: i64) -> String {
        let i18n = Polyglot::new(locale);

        // Add stock pluralization
        let mut i18n = i18n.clone();
        i18n.add("stock.zero", "Out of stock");
        i18n.add("stock.one", "{{count}} left!");
        i18n.add("stock.many", "{{count}} in stock");

        format!(r#"
            <div class="product-card">
                <h3>{}</h3>
                <p class="price">{}</p>
                <p class="stock">{}</p>
            </div>
        "#, 
            name,
            i18n.format_number(price, NumberStyle::Currency),
            i18n.plural("stock", stock)
        )
    }
</n:script>
