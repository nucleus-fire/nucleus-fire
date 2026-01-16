
#[cfg(test)]
mod tests {
    use crate::parser::{parse_node, parse_root};
    use crate::ast::Node;

    #[test]
    fn test_parse_action_simple() {
        let input = "<n:action>code</n:action>";
        let (_, node) = parse_node(input).expect("Failed to parse simple action");
        match node {
            Node::Action(content) => assert_eq!(content, "code"),
            _ => panic!("Parsed as {:?}", node),
        }
    }

    #[test]
    fn test_parse_action_with_spaces() {
        let input = "<n:action  >code</n:action>";
        let (_, node) = parse_node(input).expect("Failed to parse action with spaces");
        match node {
            Node::Action(content) => assert_eq!(content, "code"),
            _ => panic!("Parsed as {:?}", node),
        }
    }

    #[test]
    fn test_parse_action_inside_view() {
        let input = r#"<n:view>
            <n:action>
                let x = 1;
            </n:action>
        </n:view>"#;
        
        // Parsing root usually returns list of nodes.
        // But here we test parse_node on the view.
        // Actually parse_root is better.
        let (_, nodes) = parse_root(input).expect("Failed to parse root");
        let view = &nodes[0];
        
        match view {
            Node::Element(el) => {
                assert_eq!(el.tag_name, "n:view");
                let action_node = el.children.iter().find(|n| matches!(n, Node::Action(_)));
                assert!(action_node.is_some(), "Action node not found in children: {:?}", el.children);
            },
            _ => panic!("Root is not element"),
        }
    }

    #[test]
    fn test_parse_loader_and_action() {
        let input = r#"<n:view title="Test">
            <n:loader>
                let l = 1;
            </n:loader>

            <n:action>
                let a = 1;
            </n:action>
        </n:view>"#;
        
        let (_, nodes) = parse_root(input).expect("Failed to parse root");
        let view = &nodes[0];
        
        match view {
            Node::Element(el) => {
                let loader = el.children.iter().find(|n| matches!(n, Node::Loader(_)));
                let action = el.children.iter().find(|n| matches!(n, Node::Action(_)));
                
                assert!(loader.is_some(), "Loader not found. Children: {:?}", el.children);
                assert!(action.is_some(), "Action not found. Children: {:?}", el.children);
            },
            _ => panic!("Root is not element"),
        }
    }
}
