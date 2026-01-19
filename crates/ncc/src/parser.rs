use crate::ast::{Component, Element, Node, Prop};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while1},
    character::complete::{char, multispace0, multispace1, space0},
    multi::{many0, many_till}, // Added many_till
    sequence::{delimited, preceded, tuple},
    IResult,
};

fn is_tag_char(c: char) -> bool {
    c.is_alphanumeric() || c == ':' || c == '-' || c == '_'
}

fn parse_identifier(input: &str) -> IResult<&str, &str> {
    take_while1(is_tag_char)(input)
}

fn parse_qstring(input: &str) -> IResult<&str, &str> {
    delimited(char('"'), take_until("\""), char('"'))(input)
}

fn parse_bracketed(input: &str) -> IResult<&str, &str> {
    delimited(char('{'), take_until("}"), char('}'))(input)
}

fn parse_attribute(input: &str) -> IResult<&str, (String, String)> {
    let (input, key) = parse_identifier(input)?;
    let (input, val) = alt((
        preceded(tuple((space0, char('='), space0)), parse_attr_value),
        |i| Ok((i, "true".to_string())), // Boolean attribute
    ))(input)?;
    Ok((input, (key.to_string(), val)))
}

fn parse_attributes(input: &str) -> IResult<&str, Vec<(String, String)>> {
    many0(preceded(multispace1, parse_attribute))(input)
}

#[allow(clippy::type_complexity)]
fn parse_open_tag(input: &str) -> IResult<&str, (String, Vec<(String, String)>, bool)> {
    let (input, _) = char('<')(input)?;
    let (input, name) = parse_identifier(input)?;
    let (input, attrs) = parse_attributes(input)?;
    let (input, _) = multispace0(input)?;

    let (input, closing) = alt((tag("/>"), tag(">")))(input)?;

    Ok((input, (name.to_string(), attrs, closing == "/>")))
}

fn parse_close_tag<'a>(input: &'a str, tag_name: &str) -> IResult<&'a str, &'a str> {
    delimited(
        tuple((multispace0, tag("</"))),
        tag(tag_name),
        tuple((multispace0, char('>'))),
    )(input)
}

// Special handling for <n:script> to capture raw content
fn parse_script_element(input: &str) -> IResult<&str, Node> {
    let (input, (tag_name, attributes, _)) = parse_open_tag(input)?;
    if tag_name != "n:script" {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }
    let (input, content) = take_until("</n:script>")(input)?;
    let (input, _) = tag("</n:script>")(input)?;
    Ok((
        input,
        Node::Script {
            content: content.to_string(),
            attributes,
        },
    ))
}

fn parse_html_script(input: &str) -> IResult<&str, Node> {
    let (input, (tag_name, attributes, _)) = parse_open_tag(input)?;
    if tag_name != "script" {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }
    let (input, content) = take_until("</script>")(input)?;
    let (input, _) = tag("</script>")(input)?;

    // Return as Element to fit AST, but skipping parsing children
    Ok((
        input,
        Node::Element(Element {
            tag_name: "script".to_string(),
            attributes,
            children: vec![Node::Text(content.to_string())],
        }),
    ))
}

fn parse_style_element(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag("<n:style>")(input)?;
    let (input, content) = take_until("</n:style>")(input)?;
    let (input, _) = tag("</n:style>")(input)?;
    Ok((input, Node::Style(content.to_string())))
}

fn parse_html_style(input: &str) -> IResult<&str, Node> {
    let (input, (tag_name, attributes, _)) = parse_open_tag(input)?;
    if tag_name != "style" {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }
    let (input, content) = take_until("</style>")(input)?;
    let (input, _) = tag("</style>")(input)?;

    Ok((
        input,
        Node::Element(Element {
            tag_name: "style".to_string(),
            attributes,
            children: vec![Node::Text(content.to_string())],
        }),
    ))
}

fn parse_spec_element(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag("<n:spec>")(input)?;
    let (input, content) = take_until("</n:spec>")(input)?;
    let (input, _) = tag("</n:spec>")(input)?;
    Ok((input, Node::Spec(content.to_string())))
}

fn parse_test_element(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag("<n:test>")(input)?;
    let (input, content) = take_until("</n:test>")(input)?;
    let (input, _) = tag("</n:test>")(input)?;
    Ok((input, Node::Test(content.to_string())))
}

fn parse_client_element(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag("<n:client>")(input)?;
    let (input, content) = take_until("</n:client>")(input)?;
    let (input, _) = tag("</n:client>")(input)?;
    Ok((input, Node::Client(content.to_string())))
}

fn parse_element_node(input: &str) -> IResult<&str, Node> {
    let (input, (tag_name, attributes, self_closing)) = parse_open_tag(input)?;

    // If explicit self-closing (e.g. <div />) or known void tag, don't parse children
    let is_void = [
        "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param",
        "source", "track", "wbr",
    ]
    .contains(&tag_name.as_str());

    if self_closing || is_void {
        return Ok((
            input,
            Node::Element(Element {
                tag_name,
                attributes,
                children: vec![],
            }),
        ));
    }

    let (input, children) = many0(parse_node)(input)?;
    let (input, _) = parse_close_tag(input, &tag_name)?;

    Ok((
        input,
        Node::Element(Element {
            tag_name,
            attributes,
            children,
        }),
    ))
}

fn parse_text_node(input: &str) -> IResult<&str, Node> {
    let (input, text) = take_while1(|c| c != '<' && c != '{')(input)?;
    Ok((input, Node::Text(text.to_string())))
}

fn parse_doctype(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag("<!DOCTYPE")(input)?;
    let (input, content) = take_until(">")(input)?;
    let (input, _) = tag(">")(input)?;
    Ok((input, Node::Text(format!("<!DOCTYPE{}>", content))))
}

fn parse_comment(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag("<!--")(input)?;
    let (input, content) = take_until("-->")(input)?;
    let (input, _) = tag("-->")(input)?;
    Ok((input, Node::Text(format!("<!--{}-->", content))))
}

// {{ expression }}
fn parse_interpolation(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag("{{")(input)?;
    let (input, content) = take_until("}}")(input)?;
    let (input, _) = tag("}}")(input)?;
    Ok((input, Node::Interpolation(content.trim().to_string())))
}

// {% for item in items %} ... {% endfor %}
fn parse_for(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag("{%")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("for")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, variable) = parse_identifier(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("in")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, iterable) = take_until("%}")(input)?;
    let (input, _) = tag("%}")(input)?;

    // Parse children until {% endfor %}. many_till consumes the end tag.
    let (input, (children, _)) = many_till(
        parse_node,
        preceded(
            multispace0,
            tuple((
                tag("{%"),
                multispace0,
                tag("endfor"),
                multispace0,
                tag("%}"),
            )),
        ),
    )(input)?;

    Ok((
        input,
        Node::For {
            variable: variable.to_string(),
            iterable: iterable.trim().to_string(),
            children,
        },
    ))
}

// {% if condition %} ... {% endif %}
fn parse_if(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag("{%")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("if")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, condition) = take_until("%}")(input)?;
    let (input, _) = tag("%}")(input)?;

    // Parse children until {% endif %}
    let (input, (children, _)) = many_till(
        parse_node,
        preceded(
            multispace0,
            tuple((tag("{%"), multispace0, tag("endif"), multispace0, tag("%}"))),
        ),
    )(input)?;

    Ok((
        input,
        Node::If {
            condition: condition.trim().to_string(),
            children,
        },
    ))
}

fn parse_attr_value(input: &str) -> IResult<&str, String> {
    alt((
        map(parse_qstring, |s| s.to_string()),
        map(parse_bracketed, |s| format!("{{{}}}", s)),
        map(
            delimited(tag("{{"), take_until("}}"), tag("}}")),
            |s: &str| format!("{{{}}}", s.trim()),
        ),
    ))(input)
}

fn map<I, O, E, F, O2>(parser: F, f: impl Fn(O) -> O2) -> impl FnMut(I) -> IResult<I, O2, E>
where
    F: FnMut(I) -> IResult<I, O, E>,
    E: nom::error::ParseError<I>,
{
    nom::combinator::map(parser, f)
}

fn parse_loose_brace(input: &str) -> IResult<&str, Node> {
    let (input, c) = char('{')(input)?;
    Ok((input, Node::Text(c.to_string())))
}

fn parse_include(input: &str) -> IResult<&str, Node> {
    let (input, (tag_name, attributes, self_closing)) = parse_open_tag(input)?;
    if tag_name != "n:include" {
        // Backtrack if not n:include
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }

    if !self_closing {
        let (input, _) = take_until("</n:include>")(input)?;
        let (input, _) = tag("</n:include>")(input)?;
        Ok((
            input,
            Node::Include {
                path: attributes
                    .iter()
                    .find(|(k, _)| k == "src")
                    .map(|(_, v)| v.clone())
                    .unwrap_or_default(),
                attributes,
            },
        ))
    } else {
        Ok((
            input,
            Node::Include {
                path: attributes
                    .iter()
                    .find(|(k, _)| k == "src")
                    .map(|(_, v)| v.clone())
                    .unwrap_or_default(),
                attributes,
            },
        ))
    }
}

fn parse_island(input: &str) -> IResult<&str, Node> {
    let (input, (tag_name, attributes, self_closing)) = parse_open_tag(input)?;
    if tag_name != "n:island" {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }

    // Parse Directive
    let directive = attributes
        .iter()
        .find(|(k, _)| k.starts_with("client:"))
        .map(|(k, _)| k.strip_prefix("client:").unwrap_or("").to_string())
        .unwrap_or_else(|| "load".to_string()); // Default to load

    let path = attributes
        .iter()
        .find(|(k, _)| k == "src")
        .map(|(_, v)| v.clone())
        .unwrap_or_default();

    let input = if !self_closing {
        let (input, _) = take_until("</n:island>")(input)?;
        let (input, _) = tag("</n:island>")(input)?;
        input
    } else {
        input
    };

    Ok((
        input,
        Node::Island {
            path,
            directive,
            attributes,
        },
    ))
}

fn parse_loader(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag("<n:loader")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char('>')(input)?;
    let (input, content) = take_until("</n:loader>")(input)?;
    let (input, _) = tag("</n:loader>")(input)?;
    Ok((input, Node::Loader(content.to_string())))
}

fn parse_action(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag("<n:action")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char('>')(input)?;
    let (input, content) = take_until("</n:action>")(input)?;
    let (input, _) = tag("</n:action>")(input)?;
    Ok((input, Node::Action(content.to_string())))
}

fn parse_outlet(input: &str) -> IResult<&str, Node> {
    let (input, (tag_name, _, self_closing)) = parse_open_tag(input)?;
    if tag_name != "n:outlet" {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }

    let input = if !self_closing {
        let (input, _) = take_until("</n:outlet>")(input)?;
        let (input, _) = tag("</n:outlet>")(input)?;
        input
    } else {
        input
    };

    Ok((input, Node::Outlet))
}

// Parse <n:slot /> or <n:slot name="header" />
fn parse_slot(input: &str) -> IResult<&str, Node> {
    let (input, (tag_name, attributes, self_closing)) = parse_open_tag(input)?;
    if tag_name != "n:slot" {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }

    let name = attributes
        .iter()
        .find(|(k, _)| k == "name")
        .map(|(_, v)| v.clone());

    let input = if !self_closing {
        let (input, _) = take_until("</n:slot>")(input)?;
        let (input, _) = tag("</n:slot>")(input)?;
        input
    } else {
        input
    };

    Ok((input, Node::Slot { name }))
}

// Parse <style scoped> ... </style>
fn parse_scoped_style(input: &str) -> IResult<&str, Node> {
    let (input, (tag_name, attributes, _)) = parse_open_tag(input)?;
    if tag_name != "style" {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }

    let is_scoped = attributes.iter().any(|(k, _)| k == "scoped");
    if !is_scoped {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }

    let (input, content) = take_until("</style>")(input)?;
    let (input, _) = tag("</style>")(input)?;

    // Generate a simple scope ID from content hash
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    content.hash(&mut hasher);
    let scope_id = format!("nc{:x}", hasher.finish() & 0xFFFFFF);

    Ok((
        input,
        Node::ScopedStyle {
            content: content.to_string(),
            scope_id,
        },
    ))
}

// Parse <n:props> declarations inside component
fn parse_props_block(input: &str) -> IResult<&str, Vec<Prop>> {
    let (input, _) = tag("<n:props>")(input)?;
    let (input, content) = take_until("</n:props>")(input)?;
    let (input, _) = tag("</n:props>")(input)?;

    let mut props = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        // Format: name: Type = "default" or name: Type (required)
        if let Some((name_part, rest)) = trimmed.split_once(':') {
            let name = name_part.trim().to_string();
            let (prop_type, default) = if let Some((type_part, default_part)) = rest.split_once('=')
            {
                (
                    type_part.trim().to_string(),
                    Some(default_part.trim().trim_matches('"').to_string()),
                )
            } else {
                (rest.trim().to_string(), None)
            };
            let required = default.is_none();

            props.push(Prop {
                name,
                prop_type,
                default,
                required,
            });
        }
    }

    Ok((input, props))
}

// Parse <n:component name="Button"> ... </n:component>
fn parse_component(input: &str) -> IResult<&str, Node> {
    let (input, (tag_name, attributes, _)) = parse_open_tag(input)?;
    if tag_name != "n:component" {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }

    let name = attributes
        .iter()
        .find(|(k, _)| k == "name")
        .map(|(_, v)| v.clone())
        .unwrap_or_else(|| "Anonymous".to_string());

    // Get content between open and close tags
    let (input, content) = take_until("</n:component>")(input)?;
    let (input, _) = tag("</n:component>")(input)?;

    // Parse props if present
    let props = if let Ok((_, props)) = parse_props_block(content) {
        props
    } else {
        Vec::new()
    };

    // Parse children (rest of content after props)
    let children_content = if content.contains("</n:props>") {
        content.split("</n:props>").nth(1).unwrap_or("")
    } else {
        content
    };

    let children = if let Ok((_, nodes)) = parse_root(children_content) {
        nodes
    } else {
        Vec::new()
    };

    // Check for scoped style
    let scoped = content.contains("scoped");
    let styles = if let Some(start) = content.find("<style") {
        if let Some(end) = content.find("</style>") {
            let style_content = &content[start..end + 8];
            style_content.find('>').map(|inner_start| {
                style_content[inner_start + 1..style_content.len() - 8].to_string()
            })
        } else {
            None
        }
    } else {
        None
    };

    Ok((
        input,
        Node::Component(Component {
            name,
            props,
            children,
            styles,
            scoped,
        }),
    ))
}

// Parse PascalCase component usage: <Button variant="primary">Click</Button>
fn parse_component_use(input: &str) -> IResult<&str, Node> {
    let (input, (tag_name, attributes, self_closing)) = parse_open_tag(input)?;

    // Component names must be PascalCase (start with uppercase)
    if !tag_name
        .chars()
        .next()
        .map(|c| c.is_uppercase())
        .unwrap_or(false)
    {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }

    // Don't match n: prefixed tags
    if tag_name.starts_with("n:") {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }

    let children = if self_closing {
        Vec::new()
    } else {
        let close_tag = format!("</{}>", tag_name);
        let (remaining, content) = take_until(close_tag.as_str())(input)?;
        let (input, _) = tag(close_tag.as_str())(remaining)?;

        if let Ok((_, nodes)) = parse_root(content) {
            return Ok((
                input,
                Node::ComponentUse {
                    name: tag_name,
                    props: attributes,
                    children: nodes,
                },
            ));
        } else {
            Vec::new()
        }
    };

    Ok((
        input,
        Node::ComponentUse {
            name: tag_name,
            props: attributes,
            children,
        },
    ))
}

pub fn parse_node(input: &str) -> IResult<&str, Node> {
    preceded(
        multispace0,
        alt((
            // Group 1: Special elements and preprocessing
            alt((
                parse_doctype,
                parse_comment,
                parse_script_element,
                parse_html_script,
                parse_scoped_style,
                parse_style_element,
                parse_html_style,
            )),
            // Group 2: Nucleus special elements
            alt((
                parse_spec_element,
                parse_test_element,
                parse_client_element,
                parse_interpolation,
                parse_for,
                parse_if,
                parse_include,
            )),
            // Group 3: More Nucleus elements and components
            alt((
                parse_island,
                parse_loader,
                parse_action,
                parse_outlet,
                parse_slot,
                parse_component,
                parse_component_use,
            )),
            // Group 4: Fallback parsers
            alt((
                parse_model_node,
                parse_element_node,
                parse_text_node,
                parse_loose_brace,
            )),
        )),
    )(input)
}

fn parse_model_node(input: &str) -> IResult<&str, Node> {
    let (input, attributes) = delimited(
        tag("<n:model"),
        parse_attributes,
        tuple((multispace0, char('>'))),
    )(input)?;

    let (input, content) = take_until("</n:model>")(input)?;
    let (input, _) = tag("</n:model>")(input)?;

    let name = attributes
        .iter()
        .find(|(k, _)| k == "name")
        .map(|(_, v)| v.clone())
        .unwrap_or_else(|| "Unknown".to_string());

    let mut fields = Vec::new();
    let mut methods = Vec::new();
    let mut attributes = Vec::new();

    let mut method_block = String::new();
    let mut in_method = false;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        if in_method {
            method_block.push_str(line);
            method_block.push('\n');
            if trimmed.contains('}') {
                methods.push(method_block.clone());
                method_block.clear();
                in_method = false;
            }
            continue;
        }

        if trimmed.starts_with("#[") {
            attributes.push(trimmed.to_string());
        } else if trimmed.starts_with("fn ") || trimmed.contains("fn ") {
            in_method = true;
            method_block.push_str(line);
            method_block.push('\n');
        } else if let Some((field_name, field_type)) = line.split_once(':') {
            let t = field_type.trim();
            let rust_type = match t {
                "string" | "String" => "String",
                "int" | "i32" => "i32",
                "long" | "i64" => "i64",
                "float" | "f64" => "f64",
                "bool" | "Boolean" => "bool",
                _ => t,
            };
            fields.push((field_name.trim().to_string(), rust_type.to_string()));
        }
    }

    Ok((
        input,
        Node::Model(crate::ast::Model {
            name,
            fields,
            methods,
            attributes,
        }),
    ))
}

pub fn parse_root(input: &str) -> IResult<&str, Vec<Node>> {
    many0(parse_node)(input)
}

pub fn parse_code(input: &str) -> Result<Vec<Node>, crate::errors::NucleusError> {
    match parse_root(input) {
        Ok((remainder, nodes)) => {
            if !remainder.trim().is_empty() {
                let offset = input.len() - remainder.len();
                return Err(crate::errors::NucleusError::ParseError {
                    src: input.to_string(),
                    span: (offset, remainder.len()).into(),
                    kind: "Unexpected content after parsing".to_string(),
                });
            }
            Ok(nodes)
        }
        Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => {
            let offset = input.len() - e.input.len();
            Err(crate::errors::NucleusError::ParseError {
                src: input.to_string(),
                span: (offset, 0).into(), // Point to exact location
                kind: format!("{:?}", e.code),
            })
        }
        Err(nom::Err::Incomplete(_)) => Err(crate::errors::NucleusError::ParseError {
            src: input.to_string(),
            span: (input.len(), 0).into(),
            kind: "Incomplete input".to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_view() {
        let input = r#"
            <n:view>
                <n:h1 class="title">Hello</n:h1>
            </n:view>
        "#;
        let (_, nodes) = parse_root(input).unwrap();
        assert_eq!(nodes.len(), 1);
        if let Node::Element(el) = &nodes[0] {
            assert_eq!(el.tag_name, "n:view");
            assert_eq!(el.children.len(), 1);
            if let Node::Element(child) = &el.children[0] {
                assert_eq!(child.tag_name, "n:h1");
                assert_eq!(child.children.len(), 1);
            }
        } else {
            panic!("Expected Element");
        }
    }

    #[test]
    fn test_parse_script() {
        let input = r#"
            <n:script>
                fn main() { println!("Hello"); }
            </n:script>
        "#;
        let (_, nodes) = parse_root(input).unwrap();
        if let Node::Script { content, .. } = &nodes[0] {
            assert!(content.contains("fn main"));
        } else {
            panic!("Expected Script");
        }
    }

    #[test]
    fn test_parse_spec() {
        let input = r#"
            <n:spec>
                fn test_something() { assert!(true); }
            </n:spec>
        "#;
        let (_, nodes) = parse_root(input).unwrap();
        if let Node::Spec(content) = &nodes[0] {
            assert!(content.contains("fn test_something"));
        } else {
            panic!("Expected Spec");
        }
    }

    #[test]
    fn test_parse_model() {
        let input = r#"
            <n:model name="User">
                username: String
                age: Integer
            </n:model>
        "#;
        let (_, nodes) = parse_root(input).unwrap();
        if let Node::Model(model) = &nodes[0] {
            assert_eq!(model.name, "User");
            assert_eq!(model.fields.len(), 2);
            assert_eq!(
                model.fields[0],
                ("username".to_string(), "String".to_string())
            );
        } else {
            panic!("Expected Model");
        }
    }

    #[test]
    fn test_parse_rich_model() {
        let input = r#"
            <n:model name="RichUser">
                #[derive(Debug)]
                username: string
                
                fn is_cool(&self) -> bool {
                    true
                }
            </n:model>
        "#;
        let (_, nodes) = parse_root(input).unwrap();
        if let Node::Model(model) = &nodes[0] {
            assert_eq!(model.name, "RichUser");
            assert_eq!(
                model.fields[0],
                ("username".to_string(), "String".to_string())
            ); // Checked aliasing
            assert_eq!(model.attributes.len(), 1);
            assert!(model.attributes[0].contains("#[derive(Debug)]"));
            assert_eq!(model.methods.len(), 1);
            assert!(model.methods[0].contains("fn is_cool"));
        } else {
            panic!("Expected Model");
        }
    }
}

#[cfg(test)]
mod db_tests {
    // use super::*;
    use crate::ast::Model;
    use crate::db::generate_sql;

    #[test]
    fn test_generate_sql() {
        let model = Model {
            name: "User".to_string(),
            fields: vec![
                ("username".to_string(), "String".to_string()),
                ("active".to_string(), "Boolean".to_string()),
            ],
            methods: vec![],
            attributes: vec![],
        };

        let sql = generate_sql(&model);
        assert!(sql.contains("CREATE TABLE user ("));
        assert!(sql.contains("username TEXT"));
        assert!(sql.contains("active BOOLEAN"));
    }
}

#[cfg(test)]
mod parser_tests_v2 {
    use super::*;

    #[test]
    fn test_for_loop() {
        let input = "{% for item in items %} content {% endfor %}";
        let (_rest, node) = parse_for(input).unwrap();

        if let Node::For {
            variable,
            iterable,
            children: _,
        } = node
        {
            assert_eq!(variable, "item");
            assert_eq!(iterable, "items");
        } else {
            panic!("Expected For node");
        }
    }

    #[test]
    fn test_root() {
        let input = "<h1>Hello</h1>";
        let (_rest, _nodes) = parse_root(input).unwrap();
    }

    #[test]
    fn test_nested() {
        let input = "<div>{% for i in list %} <p>Hi</p> {% endfor %}</div>";
        let (rest, nodes) = parse_root(input).unwrap();
        assert!(rest.is_empty());
        assert_eq!(nodes.len(), 1, "Expected 1 Div node");

        // Check inside the Div
        if let Node::Element(el) = &nodes[0] {
            assert_eq!(el.tag_name, "div");
            assert_eq!(
                el.children.len(),
                1,
                "Expected 1 child (For loop) inside Div"
            );

            if let Node::For {
                variable,
                iterable,
                children,
            } = &el.children[0]
            {
                assert_eq!(variable, "i");
                assert_eq!(iterable, "list");
                assert!(!children.is_empty());
            } else {
                panic!("Expected For loop inside Div");
            }
        } else {
            panic!("Expected Element");
        }
    }
}

#[cfg(test)]
mod parser_tests_v3 {
    use super::*;

    #[test]
    fn test_parse_for_with_newlines() {
        let input = "\n        {% for page in docs %}\n            <div />\n        {% endfor %}";
        // Use parse_root (the main entry point) instead of parse_for to test precedence
        let (_rest, nodes) = parse_root(input).unwrap();

        // We expect ONE node (Node::For)
        // If it parses as Text, we get text.
        assert_eq!(nodes.len(), 1, "Expected 1 node, got {:?}", nodes);
        match &nodes[0] {
            Node::For {
                variable,
                iterable,
                children: _,
            } => {
                assert_eq!(variable, "page");
                assert_eq!(iterable, "docs");
            }
            _ => panic!("Expected Node::For, got {:?}", nodes[0]),
        }
    }
}
