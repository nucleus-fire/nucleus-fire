use crate::dax::{DaxQuery, DaxField};
use std::collections::HashSet;

pub struct MockSchema {
    pub tables: std::collections::HashMap<String, HashSet<String>>,
}

impl Default for MockSchema {
    fn default() -> Self {
        Self::new()
    }
}

impl MockSchema {
    pub fn new() -> Self {
        Self { tables: std::collections::HashMap::new() }
    }
    
    pub fn add_table(&mut self, name: &str, fields: Vec<&str>) {
        let field_set: HashSet<String> = fields.iter().map(|s| s.to_string()).collect();
        self.tables.insert(name.to_string(), field_set);
    }
}

pub fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let m = s1.len();
    let n = s2.len();
    let mut dp = vec![vec![0; n + 1]; m + 1];

    for (i, row) in dp.iter_mut().enumerate().take(m + 1) { row[0] = i; }
    for (j, item) in dp[0].iter_mut().enumerate().take(n + 1) { *item = j; }

    for i in 1..=m {
        for j in 1..=n {
            let cost = if s1.chars().nth(i - 1) == s2.chars().nth(j - 1) { 0 } else { 1 };
            dp[i][j] = std::cmp::min(
                std::cmp::min(dp[i - 1][j] + 1, dp[i][j - 1] + 1),
                dp[i - 1][j - 1] + cost,
            );
        }
    }
    dp[m][n]
}

pub fn find_closest_match<'a>(target: &str, candidates: &'a HashSet<String>) -> Option<&'a String> {
    let mut best_dist = usize::MAX;
    let mut best_match = None;

    for candidate in candidates {
        let dist = levenshtein_distance(target, candidate);
        if dist < best_dist && dist <= 3 { // Threshold
            best_dist = dist;
            best_match = Some(candidate);
        }
    }
    best_match
}

pub fn validate_query(query: &DaxQuery, schema: &MockSchema) -> Result<(), String> {
    // 1. Check Table
    if !schema.tables.contains_key(&query.entity) {
        // Suggest table?
        let known_tables: HashSet<String> = schema.tables.keys().cloned().collect();
        if let Some(suggestion) = find_closest_match(&query.entity, &known_tables) {
            return Err(format!("Table '{}' not found. Did you mean '{}'?", query.entity, suggestion));
        }
        return Err(format!("Table '{}' not found in schema.", query.entity));
    }

    let table_fields = schema.tables.get(&query.entity).unwrap();

    // 2. Check Fields
    for field in &query.fields {
        match field {
            DaxField::Scalar(name) => {
                if !table_fields.contains(name) {
                     if let Some(suggestion) = find_closest_match(name, table_fields) {
                        return Err(format!("Field '{}' not found in table '{}'. Did you mean '{}'?", name, query.entity, suggestion));
                    }
                    return Err(format!("Field '{}' not found in table '{}'.", name, query.entity));
                }
            },
            DaxField::Relation(subq) => {
                 // Relations refer to other tables, usually mapped.
                 // For now, check if subq.entity is a valid table
                 validate_query(subq, schema)?;
            }
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuzzy_suggestion() {
        let mut schema = MockSchema::new();
        schema.add_table("User", vec!["username", "email", "active"]);

        let query = DaxQuery {
            entity: "User".to_string(),
            filters: vec![],
            fields: vec![DaxField::Scalar("usrname".to_string())], // Typo
        };

        let err = validate_query(&query, &schema).unwrap_err();
        assert_eq!(err, "Field 'usrname' not found in table 'User'. Did you mean 'username'?");
    }
    
    #[test]
    fn test_table_typo() {
        let mut schema = MockSchema::new();
        schema.add_table("User", vec!["id"]);
        
        let query = DaxQuery {
            entity: "Usr".to_string(), // Typo
            filters: vec![],
            fields: vec![],
        };
        
        let err = validate_query(&query, &schema).unwrap_err();
        assert_eq!(err, "Table 'Usr' not found. Did you mean 'User'?");
    }
}
