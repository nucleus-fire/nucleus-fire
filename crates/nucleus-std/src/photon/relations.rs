//! Relationships for Photon Models
//!
//! Provides trait-based relationship definitions for eager loading.
//!
//! # Example
//!
//! ```rust,ignore
//! use nucleus_std::photon::{Model, HasMany, BelongsTo};
//!
//! struct User { id: i64, name: String }
//! struct Post { id: i64, user_id: i64, title: String }
//!
//! impl HasMany<Post> for User {
//!     fn foreign_key() -> &'static str { "user_id" }
//! }
//!
//! impl BelongsTo<User> for Post {
//!     fn foreign_key() -> &'static str { "user_id" }
//! }
//! ```

use crate::photon::query::Model;

// ═══════════════════════════════════════════════════════════════════════════
// RELATIONSHIP TRAITS
// ═══════════════════════════════════════════════════════════════════════════

/// Trait for "has many" relationships (one-to-many)
///
/// Example: User has many Posts
pub trait HasMany<T: Model> {
    /// The foreign key column on the related table
    fn foreign_key() -> &'static str;
    
    /// Get the ID of this record for the relationship
    fn get_id(&self) -> i64;
}

/// Trait for "belongs to" relationships (many-to-one)
///
/// Example: Post belongs to User
pub trait BelongsTo<T: Model> {
    /// The foreign key column on this table
    fn foreign_key() -> &'static str;
    
    /// Get the foreign key value
    fn get_foreign_key_value(&self) -> i64;
}

/// Trait for "has one" relationships (one-to-one)
///
/// Example: User has one Profile
pub trait HasOne<T: Model> {
    /// The foreign key column on the related table
    fn foreign_key() -> &'static str;
    
    /// Get the ID of this record for the relationship
    fn get_id(&self) -> i64;
}

// ═══════════════════════════════════════════════════════════════════════════
// EAGER LOADING HELPERS
// ═══════════════════════════════════════════════════════════════════════════

/// Load related records for a collection using HasMany (SQLite)
pub async fn load_has_many_sqlite<Parent, Child>(
    parents: &[Parent],
) -> Result<std::collections::HashMap<i64, Vec<Child>>, sqlx::Error>
where
    Parent: HasMany<Child>,
    Child: Model + for<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> + Send + Unpin,
{
    use crate::photon::db::db;
    
    if parents.is_empty() {
        return Ok(std::collections::HashMap::new());
    }
    
    let ids: Vec<i64> = parents.iter().map(|p| p.get_id()).collect();
    let placeholders: Vec<String> = (0..ids.len()).map(|_| "?".to_string()).collect();
    let sql = format!(
        "SELECT * FROM {} WHERE {} IN ({})",
        Child::table_name(),
        Parent::foreign_key(),
        placeholders.join(", ")
    );
    
    let pool = db();
    let sqlite_pool = pool.as_sqlite()
        .ok_or_else(|| sqlx::Error::Configuration("Not a SQLite database".into()))?;
    
    let mut query = sqlx::query_as::<_, Child>(&sql);
    for id in &ids {
        query = query.bind(*id);
    }
    
    let children: Vec<Child> = query.fetch_all(sqlite_pool).await?;
    
    let mut map = std::collections::HashMap::new();
    if !children.is_empty() && !ids.is_empty() {
        map.insert(ids[0], children);
    }
    
    Ok(map)
}

/// Load related records for a collection using HasMany (PostgreSQL)
pub async fn load_has_many_postgres<Parent, Child>(
    parents: &[Parent],
) -> Result<std::collections::HashMap<i64, Vec<Child>>, sqlx::Error>
where
    Parent: HasMany<Child>,
    Child: Model + for<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
{
    use crate::photon::db::db;
    
    if parents.is_empty() {
        return Ok(std::collections::HashMap::new());
    }
    
    let ids: Vec<i64> = parents.iter().map(|p| p.get_id()).collect();
    let placeholders: Vec<String> = (1..=ids.len()).map(|i| format!("${}", i)).collect();
    let sql = format!(
        "SELECT * FROM {} WHERE {} IN ({})",
        Child::table_name(),
        Parent::foreign_key(),
        placeholders.join(", ")
    );
    
    let pool = db();
    let pg_pool = pool.as_postgres()
        .ok_or_else(|| sqlx::Error::Configuration("Not a PostgreSQL database".into()))?;
    
    let mut query = sqlx::query_as::<_, Child>(&sql);
    for id in &ids {
        query = query.bind(*id);
    }
    
    let children: Vec<Child> = query.fetch_all(pg_pool).await?;
    
    let mut map = std::collections::HashMap::new();
    if !children.is_empty() && !ids.is_empty() {
        map.insert(ids[0], children);
    }
    
    Ok(map)
}

/// Load related records for a collection using HasMany (MySQL)
pub async fn load_has_many_mysql<Parent, Child>(
    parents: &[Parent],
) -> Result<std::collections::HashMap<i64, Vec<Child>>, sqlx::Error>
where
    Parent: HasMany<Child>,
    Child: Model + for<'r> sqlx::FromRow<'r, sqlx::mysql::MySqlRow> + Send + Unpin,
{
    use crate::photon::db::db;
    
    if parents.is_empty() {
        return Ok(std::collections::HashMap::new());
    }
    
    let ids: Vec<i64> = parents.iter().map(|p| p.get_id()).collect();
    let placeholders: Vec<String> = (0..ids.len()).map(|_| "?".to_string()).collect();
    let sql = format!(
        "SELECT * FROM {} WHERE {} IN ({})",
        Child::table_name(),
        Parent::foreign_key(),
        placeholders.join(", ")
    );
    
    let pool = db();
    let mysql_pool = pool.as_mysql()
        .ok_or_else(|| sqlx::Error::Configuration("Not a MySQL database".into()))?;
    
    let mut query = sqlx::query_as::<_, Child>(&sql);
    for id in &ids {
        query = query.bind(*id);
    }
    
    let children: Vec<Child> = query.fetch_all(mysql_pool).await?;
    
    let mut map = std::collections::HashMap::new();
    if !children.is_empty() && !ids.is_empty() {
        map.insert(ids[0], children);
    }
    
    Ok(map)
}

/// Load a related record for a collection using BelongsTo (SQLite)
pub async fn load_belongs_to_sqlite<Child, Parent>(
    children: &[Child],
) -> Result<std::collections::HashMap<i64, Parent>, sqlx::Error>
where
    Child: BelongsTo<Parent>,
    Parent: Model + for<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> + Send + Unpin,
{
    use crate::photon::db::db;
    
    if children.is_empty() {
        return Ok(std::collections::HashMap::new());
    }
    
    let ids: Vec<i64> = children.iter().map(|c| c.get_foreign_key_value()).collect();
    let unique_ids: std::collections::HashSet<i64> = ids.into_iter().collect();
    let id_vec: Vec<i64> = unique_ids.into_iter().collect();
    
    let placeholders: Vec<String> = (0..id_vec.len()).map(|_| "?".to_string()).collect();
    let sql = format!(
        "SELECT * FROM {} WHERE id IN ({})",
        Parent::table_name(),
        placeholders.join(", ")
    );
    
    let pool = db();
    let sqlite_pool = pool.as_sqlite()
        .ok_or_else(|| sqlx::Error::Configuration("Not a SQLite database".into()))?;
    
    let mut query = sqlx::query_as::<_, Parent>(&sql);
    for id in &id_vec {
        query = query.bind(*id);
    }
    
    let parents: Vec<Parent> = query.fetch_all(sqlite_pool).await?;
    
    let mut map = std::collections::HashMap::new();
    for (i, parent) in parents.into_iter().enumerate() {
        if i < id_vec.len() {
            map.insert(id_vec[i], parent);
        }
    }
    
    Ok(map)
}

/// Load a related record for a collection using BelongsTo (PostgreSQL)
pub async fn load_belongs_to_postgres<Child, Parent>(
    children: &[Child],
) -> Result<std::collections::HashMap<i64, Parent>, sqlx::Error>
where
    Child: BelongsTo<Parent>,
    Parent: Model + for<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
{
    use crate::photon::db::db;
    
    if children.is_empty() {
        return Ok(std::collections::HashMap::new());
    }
    
    let ids: Vec<i64> = children.iter().map(|c| c.get_foreign_key_value()).collect();
    let unique_ids: std::collections::HashSet<i64> = ids.into_iter().collect();
    let id_vec: Vec<i64> = unique_ids.into_iter().collect();
    
    let placeholders: Vec<String> = (1..=id_vec.len()).map(|i| format!("${}", i)).collect();
    let sql = format!(
        "SELECT * FROM {} WHERE id IN ({})",
        Parent::table_name(),
        placeholders.join(", ")
    );
    
    let pool = db();
    let pg_pool = pool.as_postgres()
        .ok_or_else(|| sqlx::Error::Configuration("Not a PostgreSQL database".into()))?;
    
    let mut query = sqlx::query_as::<_, Parent>(&sql);
    for id in &id_vec {
        query = query.bind(*id);
    }
    
    let parents: Vec<Parent> = query.fetch_all(pg_pool).await?;
    
    let mut map = std::collections::HashMap::new();
    for (i, parent) in parents.into_iter().enumerate() {
        if i < id_vec.len() {
            map.insert(id_vec[i], parent);
        }
    }
    
    Ok(map)
}

/// Load a related record for a collection using BelongsTo (MySQL)
pub async fn load_belongs_to_mysql<Child, Parent>(
    children: &[Child],
) -> Result<std::collections::HashMap<i64, Parent>, sqlx::Error>
where
    Child: BelongsTo<Parent>,
    Parent: Model + for<'r> sqlx::FromRow<'r, sqlx::mysql::MySqlRow> + Send + Unpin,
{
    use crate::photon::db::db;
    
    if children.is_empty() {
        return Ok(std::collections::HashMap::new());
    }
    
    let ids: Vec<i64> = children.iter().map(|c| c.get_foreign_key_value()).collect();
    let unique_ids: std::collections::HashSet<i64> = ids.into_iter().collect();
    let id_vec: Vec<i64> = unique_ids.into_iter().collect();
    
    let placeholders: Vec<String> = (0..id_vec.len()).map(|_| "?".to_string()).collect();
    let sql = format!(
        "SELECT * FROM {} WHERE id IN ({})",
        Parent::table_name(),
        placeholders.join(", ")
    );
    
    let pool = db();
    let mysql_pool = pool.as_mysql()
        .ok_or_else(|| sqlx::Error::Configuration("Not a MySQL database".into()))?;
    
    let mut query = sqlx::query_as::<_, Parent>(&sql);
    for id in &id_vec {
        query = query.bind(*id);
    }
    
    let parents: Vec<Parent> = query.fetch_all(mysql_pool).await?;
    
    let mut map = std::collections::HashMap::new();
    for (i, parent) in parents.into_iter().enumerate() {
        if i < id_vec.len() {
            map.insert(id_vec[i], parent);
        }
    }
    
    Ok(map)
}

/// Legacy function - uses SQLite
#[deprecated(since = "0.2.0", note = "Use load_has_many_sqlite, load_has_many_postgres, or load_has_many_mysql")]
pub async fn load_has_many<Parent, Child>(
    parents: &[Parent],
) -> Result<std::collections::HashMap<i64, Vec<Child>>, sqlx::Error>
where
    Parent: HasMany<Child>,
    Child: Model + for<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> + Send + Unpin,
{
    load_has_many_sqlite(parents).await
}

/// Legacy function - uses SQLite
#[deprecated(since = "0.2.0", note = "Use load_belongs_to_sqlite, load_belongs_to_postgres, or load_belongs_to_mysql")]
pub async fn load_belongs_to<Child, Parent>(
    children: &[Child],
) -> Result<std::collections::HashMap<i64, Parent>, sqlx::Error>
where
    Child: BelongsTo<Parent>,
    Parent: Model + for<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> + Send + Unpin,
{
    load_belongs_to_sqlite(children).await
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    
    // Mock types for testing
    struct MockUser { id: i64 }
    struct MockPost { _id: i64, user_id: i64 }
    
    impl Model for MockUser {
        fn table_name() -> &'static str { "users" }
    }
    
    impl Model for MockPost {
        fn table_name() -> &'static str { "posts" }
    }
    
    impl HasMany<MockPost> for MockUser {
        fn foreign_key() -> &'static str { "user_id" }
        fn get_id(&self) -> i64 { self.id }
    }
    
    impl BelongsTo<MockUser> for MockPost {
        fn foreign_key() -> &'static str { "user_id" }
        fn get_foreign_key_value(&self) -> i64 { self.user_id }
    }
    
    // HasOne mock
    struct MockProfile { _id: i64, user_id: i64 }
    
    impl Model for MockProfile {
        fn table_name() -> &'static str { "profiles" }
    }
    
    impl HasOne<MockProfile> for MockUser {
        fn foreign_key() -> &'static str { "user_id" }
        fn get_id(&self) -> i64 { self.id }
    }
    
    #[test]
    fn test_has_many_trait() {
        let user = MockUser { id: 1 };
        assert_eq!(<MockUser as HasMany<MockPost>>::foreign_key(), "user_id");
        assert_eq!(<MockUser as HasMany<MockPost>>::get_id(&user), 1);
    }
    
    #[test]
    fn test_belongs_to_trait() {
        let post = MockPost { _id: 1, user_id: 42 };
        assert_eq!(<MockPost as BelongsTo<MockUser>>::foreign_key(), "user_id");
        assert_eq!(post.get_foreign_key_value(), 42);
    }
    
    #[test]
    fn test_has_one_trait() {
        let user = MockUser { id: 5 };
        assert_eq!(<MockUser as HasOne<MockProfile>>::foreign_key(), "user_id");
        assert_eq!(<MockUser as HasOne<MockProfile>>::get_id(&user), 5);
    }
    
    #[test]
    fn test_model_table_names() {
        assert_eq!(MockUser::table_name(), "users");
        assert_eq!(MockPost::table_name(), "posts");
        assert_eq!(MockProfile::table_name(), "profiles");
    }
    
    #[test]
    fn test_multiple_parents_ids() {
        let users = vec![
            MockUser { id: 1 },
            MockUser { id: 2 },
            MockUser { id: 3 },
        ];
        
        let ids: Vec<i64> = users.iter().map(|u| <MockUser as HasMany<MockPost>>::get_id(u)).collect();
        assert_eq!(ids, vec![1, 2, 3]);
    }
    
    #[test]
    fn test_multiple_foreign_keys() {
        let posts = vec![
            MockPost { _id: 1, user_id: 10 },
            MockPost { _id: 2, user_id: 10 },
            MockPost { _id: 3, user_id: 20 },
        ];
        
        let fk_values: Vec<i64> = posts.iter().map(|p| p.get_foreign_key_value()).collect();
        assert_eq!(fk_values, vec![10, 10, 20]);
        
        // Test deduplication
        let unique: std::collections::HashSet<i64> = fk_values.into_iter().collect();
        assert_eq!(unique.len(), 2); // Only 10 and 20
    }
    
    #[test]
    fn test_empty_users_handling() {
        let users: Vec<MockUser> = vec![];
        assert!(users.is_empty());
    }
    
    #[test]
    fn test_foreign_key_static_str() {
        // Foreign keys should be static strings for efficiency
        let fk: &'static str = <MockUser as HasMany<MockPost>>::foreign_key();
        assert_eq!(fk.len(), 7); // "user_id" is 7 chars
    }
}

