use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, char, multispace0},
    combinator::{opt, recognize},
    multi::{many0, separated_list0},
    sequence::{delimited, pair},
    IResult,
};
use crate::errors::NucleusError;

#[derive(Debug, PartialEq)]
pub enum DaxField {
    Scalar(String),
    Relation(DaxQuery),
}

#[derive(Debug, PartialEq)]
pub struct DaxQuery {
    pub entity: String,
    pub filters: Vec<(String, String)>,
    pub fields: Vec<DaxField>,
}

fn parse_identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        alpha1,
        many0(alt((alphanumeric1, tag("_"))))
    ))(input)
}

fn parse_filter(input: &str) -> IResult<&str, (String, String)> {
    let (input, key) = parse_identifier(input)?;
    let (input, _) = delimited(multispace0, char(':'), multispace0)(input)?;
    let (input, value) = parse_identifier(input)?; 
    Ok((input, (key.to_string(), value.to_string())))
}

fn parse_filters(input: &str) -> IResult<&str, Vec<(String, String)>> {
    delimited(
        char('('),
        separated_list0(
            delimited(multispace0, char(','), multispace0),
            parse_filter
        ),
        char(')')
    )(input)
}

fn parse_dax_field(input: &str) -> IResult<&str, DaxField> {
    // Check if it's a relation (has braces) or scalar
    // Logic: Identifier, then optional block
    let (input, name) = parse_identifier(input)?;
    let (input, _) = multispace0(input)?;
    
    // Optional filters for relation? "posts(limit: 5) { ... }"
    let (input, filters) = opt(parse_filters)(input)?;
    let filters = filters.unwrap_or_default();
    
    let (input, _) = multispace0(input)?;

    if let Ok((input, block_fields)) = parse_field_block(input) {
         Ok((input, DaxField::Relation(DaxQuery {
             entity: name.to_string(),
             filters,
             fields: block_fields,
         })))
    } else {
        Ok((input, DaxField::Scalar(name.to_string())))
    }
}

fn parse_field_block(input: &str) -> IResult<&str, Vec<DaxField>> {
     delimited(
        char('{'),
        delimited(
            multispace0,
            separated_list0(
                delimited(multispace0, char(','), multispace0),
                parse_dax_field
            ),
            multispace0
        ),
        char('}')
    )(input)
}

pub fn parse_dax(input: &str) -> IResult<&str, DaxQuery> {
    let (input, _) = multispace0(input)?;
    let (input, entity) = parse_identifier(input)?;
    let (input, _) = multispace0(input)?;
    
    let (input, filters) = opt(parse_filters)(input)?;
    let filters = filters.unwrap_or_default();

    let (input, _) = multispace0(input)?;
    
    let (input, fields) = parse_field_block(input)?;

    Ok((input, DaxQuery {
        entity: entity.to_string(),
        filters,
        fields,
    }))
}

/// Recursively collects SQL parts
fn build_sql(query: &DaxQuery, _parent: Option<&str>) -> (Vec<String>, Vec<String>) {
    let mut columns = Vec::new();
    let mut joins = Vec::new();
    let table = &query.entity;
    
    for field in &query.fields {
        match field {
            DaxField::Scalar(name) => {
                columns.push(format!("{}.{}", table, name));
            }
            DaxField::Relation(subq) => {
                // JOIN logic: LEFT JOIN subq.entity ON subq.entity.parent_id = parent.id
                // This is a naive convention-based implementation
                let sub_table = &subq.entity;
                let join_clause = format!("LEFT JOIN {} ON {}.{}_id = {}.id", sub_table, sub_table, table.to_lowercase(), table);
                joins.push(join_clause);
                
                let (sub_cols, sub_joins) = build_sql(subq, Some(table));
                columns.extend(sub_cols);
                joins.extend(sub_joins);
            }
        }
    }
    (columns, joins)
}

pub fn compile_dax_to_sql(input: &str) -> Result<String, NucleusError> {
    let (_, query) = parse_dax(input).map_err(|e| NucleusError::ParseError {
        src: input.to_string(),
        span: (0, 0).into(),
        kind: format!("DAX syntax error: {:?}", e),
    })?;

    let (columns, joins) = build_sql(&query, None);
    
    let col_str = if columns.is_empty() { "*".to_string() } else { columns.join(", ") };
    let mut sql = format!("SELECT {} FROM {}", col_str, query.entity);
    
    if !joins.is_empty() {
        sql.push(' ');
        sql.push_str(&joins.join(" "));
    }

    if !query.filters.is_empty() {
        let conditions: Vec<String> = query.filters.iter()
            .map(|(k, _v)| format!("{}.{} = ?", query.entity, k))
            .collect();
        sql.push_str(" WHERE ");
        sql.push_str(&conditions.join(" AND "));
    }

    Ok(sql)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_nested_dax() {
        let input = "User { id, posts { title } }";
        let (_, q) = parse_dax(input).unwrap();
        assert_eq!(q.entity, "User");
        match &q.fields[1] {
            DaxField::Relation(sub) => {
                assert_eq!(sub.entity, "posts");
                assert_eq!(sub.fields[0], DaxField::Scalar("title".to_string()));
            },
            _ => panic!("Expected relation")
        }
    }

    #[test]
    fn test_compile_nested_sql() {
        let input = "User { name, posts { title } }";
        let sql = compile_dax_to_sql(input).unwrap();
        assert!(sql.contains("LEFT JOIN posts ON posts.user_id = User.id"));
        assert!(sql.contains("SELECT User.name, posts.title"));
    }
}
