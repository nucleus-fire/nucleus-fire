use crate::ast::Model;
use sha2::{Sha256, Digest};

pub fn calculate_schema_hash(model: &Model) -> String {
    let mut hasher = Sha256::new();
    hasher.update(model.name.as_bytes());
    for (name, ty) in &model.fields {
        hasher.update(name.as_bytes());
        hasher.update(ty.as_bytes());
    }
    hex::encode(hasher.finalize())
}

pub fn generate_sql(model: &Model) -> String {
    let mut sql = format!("CREATE TABLE {} (\n", model.name.to_lowercase());
    
    for (i, (name, type_name)) in model.fields.iter().enumerate() {
        let sql_type = match type_name.as_str() {
            "String" => "TEXT",
            "UUID" => "UUID",
            "Integer" => "INTEGER",
            "Boolean" => "BOOLEAN",
            _ => "TEXT", // Default
        };
        
        sql.push_str(&format!("    {} {}", name, sql_type));
        
        if i < model.fields.len() - 1 {
            sql.push_str(",\n");
        } else {
            sql.push('\n');
        }
    }
    
    sql.push_str(");");
    sql
}
