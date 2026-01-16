//! Polyglot - Internationalization (i18n)
//!
//! Translation and localization support.
//!
//! # Example
//!
//! ```rust,ignore
//! use nucleus_std::polyglot::Polyglot;
//!
//! let mut i18n = Polyglot::new("en");
//! i18n.load_json(r#"{"greeting": "Hello, {{name}}!"}"#)?;
//!
//! let msg = i18n.t_with("greeting", &[("name", "World")]);
//! assert_eq!(msg, "Hello, World!");
//! ```

use serde_json::Value;
use std::collections::HashMap;

// ═══════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════

/// Date formatting style
#[derive(Debug, Clone, Copy)]
pub enum DateStyle {
    /// Short: 1/1/24
    Short,
    /// Medium: Jan 1, 2024
    Medium,
    /// Long: January 1, 2024
    Long,
}

/// Number formatting style
#[derive(Debug, Clone, Copy)]
pub enum NumberStyle {
    /// Plain number: 1234.56
    Decimal,
    /// Currency: $1,234.56
    Currency,
    /// Percentage: 12.34%
    Percent,
}

/// Pluralization rules
#[derive(Debug, Clone)]
pub struct PluralRules {
    /// Key for zero items
    pub zero: Option<String>,
    /// Key for one item
    pub one: String,
    /// Key for few items (2-4 in some languages)
    pub few: Option<String>,
    /// Key for many items
    pub many: String,
}

// ═══════════════════════════════════════════════════════════════════════════
// POLYGLOT
// ═══════════════════════════════════════════════════════════════════════════

/// Internationalization service
#[derive(Debug, Clone)]
pub struct Polyglot {
    locale: String,
    translations: HashMap<String, String>,
    fallback_locale: Option<String>,
    fallback_translations: HashMap<String, String>,
}

impl Default for Polyglot {
    fn default() -> Self {
        Self::new("en")
    }
}

impl Polyglot {
    /// Create new Polyglot with locale
    pub fn new(locale: &str) -> Self {
        Self {
            locale: locale.to_string(),
            translations: HashMap::new(),
            fallback_locale: None,
            fallback_translations: HashMap::new(),
        }
    }

    /// Get current locale
    pub fn locale(&self) -> &str {
        &self.locale
    }

    /// Set locale
    pub fn set_locale(&mut self, locale: &str) {
        self.locale = locale.to_string();
    }

    /// Set fallback locale
    pub fn set_fallback(&mut self, locale: &str, translations: HashMap<String, String>) {
        self.fallback_locale = Some(locale.to_string());
        self.fallback_translations = translations;
    }

    /// Load translations from JSON string
    pub fn load_json(&mut self, json: &str) -> Result<(), String> {
        let value: Value = serde_json::from_str(json)
            .map_err(|e| format!("Invalid JSON: {}", e))?;
        
        self.load_value("", &value);
        Ok(())
    }

    /// Load translations from TOML string
    pub fn load_toml(&mut self, toml_str: &str) -> Result<(), String> {
        let value: toml::Value = toml::from_str(toml_str)
            .map_err(|e| format!("Invalid TOML: {}", e))?;
        
        // Convert TOML to JSON Value for unified processing
        let json_str = serde_json::to_string(&value)
            .map_err(|e| format!("Conversion error: {}", e))?;
        let json_value: Value = serde_json::from_str(&json_str)
            .map_err(|e| format!("Parse error: {}", e))?;
        
        self.load_value("", &json_value);
        Ok(())
    }

    /// Add a single translation
    pub fn add(&mut self, key: &str, value: &str) {
        self.translations.insert(key.to_string(), value.to_string());
    }

    /// Get translation by key
    pub fn t(&self, key: &str) -> String {
        self.translations.get(key)
            .or_else(|| self.fallback_translations.get(key))
            .cloned()
            .unwrap_or_else(|| format!("[{}]", key))
    }

    /// Get translation with interpolation
    pub fn t_with(&self, key: &str, args: &[(&str, &str)]) -> String {
        let mut result = self.t(key);
        for (name, value) in args {
            result = result.replace(&format!("{{{{{}}}}}", name), value);
        }
        result
    }

    /// Get translation with HashMap args
    pub fn t_map(&self, key: &str, args: &HashMap<String, String>) -> String {
        let mut result = self.t(key);
        for (name, value) in args {
            result = result.replace(&format!("{{{{{}}}}}", name), value);
        }
        result
    }

    /// Get pluralized translation
    ///
    /// Expects keys like "items.one" and "items.many"
    pub fn plural(&self, key: &str, count: i64) -> String {
        let plural_key = match count {
            0 => format!("{}.zero", key),
            1 => format!("{}.one", key),
            n if (2..=4).contains(&n) => format!("{}.few", key),
            _ => format!("{}.many", key),
        };

        // Try specific key, fall back to .many, then base key
        self.translations.get(&plural_key)
            .or_else(|| self.translations.get(&format!("{}.many", key)))
            .or_else(|| self.translations.get(key))
            .cloned()
            .unwrap_or_else(|| format!("[{}]", key))
            .replace("{{count}}", &count.to_string())
    }

    /// Format number according to locale
    pub fn format_number(&self, n: f64, style: NumberStyle) -> String {
        match style {
            NumberStyle::Decimal => {
                format_with_separators(n, self.thousands_sep(), self.decimal_sep())
            }
            NumberStyle::Currency => {
                let formatted = format_with_separators(n, self.thousands_sep(), self.decimal_sep());
                format!("{}{}", self.currency_symbol(), formatted)
            }
            NumberStyle::Percent => {
                format!("{}%", format_with_separators(n * 100.0, self.thousands_sep(), self.decimal_sep()))
            }
        }
    }

    /// Format date
    pub fn format_date(&self, timestamp: i64, style: DateStyle) -> String {
        let secs = timestamp;
        let days = secs / 86400;
        let years = 1970 + days / 365;
        let day_of_year = days % 365;
        let month = (day_of_year / 30) + 1;
        let day = (day_of_year % 30) + 1;

        match style {
            DateStyle::Short => {
                if self.locale.starts_with("en") {
                    format!("{}/{}/{}", month, day, years % 100)
                } else {
                    format!("{}/{}/{}", day, month, years % 100)
                }
            }
            DateStyle::Medium => {
                let month_name = self.month_name(month as u32, true);
                format!("{} {}, {}", month_name, day, years)
            }
            DateStyle::Long => {
                let month_name = self.month_name(month as u32, false);
                format!("{} {}, {}", month_name, day, years)
            }
        }
    }

    /// Check if locale is RTL
    pub fn is_rtl(&self) -> bool {
        matches!(self.locale.as_str(), "ar" | "he" | "fa" | "ur")
    }

    /// Get all translation keys
    pub fn keys(&self) -> Vec<&str> {
        self.translations.keys().map(|k| k.as_str()).collect()
    }

    // ─────────────────────────────────────────────────────────────────────────
    // INTERNAL
    // ─────────────────────────────────────────────────────────────────────────

    fn load_value(&mut self, prefix: &str, value: &Value) {
        match value {
            Value::Object(map) => {
                for (k, v) in map {
                    let key = if prefix.is_empty() {
                        k.clone()
                    } else {
                        format!("{}.{}", prefix, k)
                    };
                    self.load_value(&key, v);
                }
            }
            Value::String(s) => {
                self.translations.insert(prefix.to_string(), s.clone());
            }
            Value::Number(n) => {
                self.translations.insert(prefix.to_string(), n.to_string());
            }
            Value::Bool(b) => {
                self.translations.insert(prefix.to_string(), b.to_string());
            }
            _ => {}
        }
    }

    fn thousands_sep(&self) -> char {
        if self.locale.starts_with("de") || self.locale.starts_with("fr") {
            '.'
        } else {
            ','
        }
    }

    fn decimal_sep(&self) -> char {
        if self.locale.starts_with("de") || self.locale.starts_with("fr") {
            ','
        } else {
            '.'
        }
    }

    fn currency_symbol(&self) -> &str {
        match self.locale.as_str() {
            "en-US" | "en" => "$",
            "en-GB" => "£",
            "de" | "de-DE" | "fr" | "fr-FR" => "€",
            "ja" | "ja-JP" => "¥",
            _ => "$",
        }
    }

    fn month_name(&self, month: u32, short: bool) -> &'static str {
        let months_short = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", 
                           "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
        let months_long = ["January", "February", "March", "April", "May", "June",
                          "July", "August", "September", "October", "November", "December"];
        
        let idx = (month.saturating_sub(1) as usize).min(11);
        if short { months_short[idx] } else { months_long[idx] }
    }
}

fn format_with_separators(n: f64, thousands: char, decimal: char) -> String {
    let int_part = n.trunc() as i64;
    let frac_part = (n.fract().abs() * 100.0).round() as i64;
    
    let int_str = int_part.abs().to_string();
    let len = int_str.len();
    
    let mut result = String::new();
    if int_part < 0 {
        result.push('-');
    }
    
    for (i, c) in int_str.chars().enumerate() {
        if i > 0 && (len - i).is_multiple_of(3) {
            result.push(thousands);
        }
        result.push(c);
    }
    
    if frac_part > 0 {
        result.push(decimal);
        result.push_str(&format!("{:02}", frac_part));
    }
    
    result
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let p = Polyglot::new("en-US");
        assert_eq!(p.locale(), "en-US");
    }

    #[test]
    fn test_set_locale() {
        let mut p = Polyglot::new("en");
        p.set_locale("fr");
        assert_eq!(p.locale(), "fr");
    }

    #[test]
    fn test_load_json() {
        let mut p = Polyglot::new("en");
        p.load_json(r#"{"greeting": "Hello"}"#).unwrap();
        assert_eq!(p.t("greeting"), "Hello");
    }

    #[test]
    fn test_load_nested_json() {
        let mut p = Polyglot::new("en");
        p.load_json(r#"{"user": {"welcome": "Welcome back"}}"#).unwrap();
        assert_eq!(p.t("user.welcome"), "Welcome back");
    }

    #[test]
    fn test_load_toml() {
        let mut p = Polyglot::new("en");
        p.load_toml(r#"greeting = "Hello""#).unwrap();
        assert_eq!(p.t("greeting"), "Hello");
    }

    #[test]
    fn test_add() {
        let mut p = Polyglot::new("en");
        p.add("key", "value");
        assert_eq!(p.t("key"), "value");
    }

    #[test]
    fn test_missing_key() {
        let p = Polyglot::new("en");
        assert_eq!(p.t("missing"), "[missing]");
    }

    #[test]
    fn test_t_with() {
        let mut p = Polyglot::new("en");
        p.add("greeting", "Hello, {{name}}!");
        assert_eq!(p.t_with("greeting", &[("name", "World")]), "Hello, World!");
    }

    #[test]
    fn test_t_with_multiple() {
        let mut p = Polyglot::new("en");
        p.add("msg", "{{a}} and {{b}}");
        assert_eq!(p.t_with("msg", &[("a", "X"), ("b", "Y")]), "X and Y");
    }

    #[test]
    fn test_t_map() {
        let mut p = Polyglot::new("en");
        p.add("greeting", "Hello, {{name}}!");
        let mut args = HashMap::new();
        args.insert("name".to_string(), "Bob".to_string());
        assert_eq!(p.t_map("greeting", &args), "Hello, Bob!");
    }

    #[test]
    fn test_plural_one() {
        let mut p = Polyglot::new("en");
        p.add("items.one", "{{count}} item");
        p.add("items.many", "{{count}} items");
        assert_eq!(p.plural("items", 1), "1 item");
    }

    #[test]
    fn test_plural_many() {
        let mut p = Polyglot::new("en");
        p.add("items.one", "{{count}} item");
        p.add("items.many", "{{count}} items");
        assert_eq!(p.plural("items", 5), "5 items");
    }

    #[test]
    fn test_plural_zero() {
        let mut p = Polyglot::new("en");
        p.add("items.zero", "No items");
        p.add("items.many", "{{count}} items");
        assert_eq!(p.plural("items", 0), "No items");
    }

    #[test]
    fn test_format_number_decimal() {
        let p = Polyglot::new("en");
        assert_eq!(p.format_number(1234.56, NumberStyle::Decimal), "1,234.56");
    }

    #[test]
    fn test_format_number_currency() {
        let p = Polyglot::new("en-US");
        assert_eq!(p.format_number(1234.0, NumberStyle::Currency), "$1,234");
    }

    #[test]
    fn test_format_number_german() {
        let p = Polyglot::new("de");
        assert_eq!(p.format_number(1234.56, NumberStyle::Decimal), "1.234,56");
    }

    #[test]
    fn test_format_number_percent() {
        let p = Polyglot::new("en");
        assert_eq!(p.format_number(0.1234, NumberStyle::Percent), "12.34%");
    }

    #[test]
    fn test_format_date_short() {
        let p = Polyglot::new("en");
        let result = p.format_date(0, DateStyle::Short);
        assert!(result.contains("/"));
    }

    #[test]
    fn test_format_date_medium() {
        let p = Polyglot::new("en");
        let result = p.format_date(0, DateStyle::Medium);
        assert!(result.contains("Jan"));
    }

    #[test]
    fn test_is_rtl() {
        let en = Polyglot::new("en");
        let ar = Polyglot::new("ar");
        let he = Polyglot::new("he");
        
        assert!(!en.is_rtl());
        assert!(ar.is_rtl());
        assert!(he.is_rtl());
    }

    #[test]
    fn test_fallback() {
        let mut p = Polyglot::new("fr");
        let mut fallback = HashMap::new();
        fallback.insert("greeting".to_string(), "Hello".to_string());
        
        p.set_fallback("en", fallback);
        
        // Missing in fr, falls back to en
        assert_eq!(p.t("greeting"), "Hello");
    }

    #[test]
    fn test_keys() {
        let mut p = Polyglot::new("en");
        p.add("a", "A");
        p.add("b", "B");
        
        let keys = p.keys();
        assert_eq!(keys.len(), 2);
    }

    #[test]
    fn test_load_json_number() {
        let mut p = Polyglot::new("en");
        p.load_json(r#"{"count": 42, "price": 9.99}"#).unwrap();
        assert_eq!(p.t("count"), "42");
        assert_eq!(p.t("price"), "9.99");
    }

    #[test]
    fn test_load_json_boolean() {
        let mut p = Polyglot::new("en");
        p.load_json(r#"{"enabled": true, "disabled": false}"#).unwrap();
        assert_eq!(p.t("enabled"), "true");
        assert_eq!(p.t("disabled"), "false");
    }

    #[test]
    fn test_load_json_invalid() {
        let mut p = Polyglot::new("en");
        let result = p.load_json("not valid json{{");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid JSON"));
    }

    #[test]
    fn test_load_toml_invalid() {
        let mut p = Polyglot::new("en");
        let result = p.load_toml("not = valid = toml");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid TOML"));
    }

    #[test]
    fn test_plural_few() {
        let mut p = Polyglot::new("en");
        p.add("items.few", "{{count}} items (few)");
        p.add("items.many", "{{count}} items");
        
        // 2-4 should use "few" if available
        assert_eq!(p.plural("items", 3), "3 items (few)");
    }

    #[test]
    fn test_format_date_non_en() {
        let p_de = Polyglot::new("de");
        let result = p_de.format_date(0, DateStyle::Short);
        // German format: day/month/year
        assert!(result.contains("/"));
    }

    #[test]
    fn test_default_trait() {
        let p: Polyglot = Default::default();
        assert_eq!(p.locale(), "en");
    }

    #[test]
    fn test_currency_symbols() {
        let gb = Polyglot::new("en-GB");
        let de = Polyglot::new("de");
        let ja = Polyglot::new("ja");
        let unknown = Polyglot::new("xx");
        
        assert_eq!(gb.format_number(100.0, NumberStyle::Currency), "£100");
        assert_eq!(de.format_number(100.0, NumberStyle::Currency), "€100");
        assert_eq!(ja.format_number(100.0, NumberStyle::Currency), "¥100");
        assert_eq!(unknown.format_number(100.0, NumberStyle::Currency), "$100");
    }

    #[test]
    fn test_format_number_negative() {
        let p = Polyglot::new("en");
        let result = p.format_number(-1234.56, NumberStyle::Decimal);
        assert!(result.starts_with("-"));
        assert!(result.contains("1,234"));
    }

    #[test]
    fn test_format_number_zero() {
        let p = Polyglot::new("en");
        assert_eq!(p.format_number(0.0, NumberStyle::Decimal), "0");
    }

    #[test]
    fn test_format_date_long() {
        let p = Polyglot::new("en");
        let result = p.format_date(0, DateStyle::Long);
        assert!(result.contains("January"));
    }

    #[test]
    fn test_rtl_all_locales() {
        assert!(Polyglot::new("fa").is_rtl()); // Farsi
        assert!(Polyglot::new("ur").is_rtl()); // Urdu
        assert!(!Polyglot::new("es").is_rtl()); // Spanish
    }

    #[test]
    fn test_plural_fallback_to_base() {
        let mut p = Polyglot::new("en");
        // Only set base key, no .one or .many
        p.add("items", "{{count}} item(s)");
        
        // Should fall back to base key
        assert_eq!(p.plural("items", 5), "5 item(s)");
    }
}
