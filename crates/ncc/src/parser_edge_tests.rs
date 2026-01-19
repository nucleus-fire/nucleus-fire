use crate::parser::parse_root;

#[test]
fn test_unclosed_tag() {
    let input = "<n:view>Hello";
    let result = parse_root(input);

    // many0 always returns Ok. Robustness check is that it didn't consume everything.
    if let Ok((rem, _nodes)) = result {
        assert!(
            !rem.trim().is_empty(),
            "Unclosed tag should leave remainder (failed to parse)"
        );
    } else {
        panic!("Parser crashed");
    }
}

#[test]
fn test_mismatched_tag() {
    let input = "<n:view></n:script>";
    let result = parse_root(input);
    if let Ok((rem, _)) = result {
        assert!(
            !rem.trim().is_empty(),
            "Mismatched tag should stop parsing and leave remainder"
        );
    }
}

#[test]
fn test_empty_input() {
    let input = "";
    let result = parse_root(input);
    match result {
        Ok((rem, nodes)) => {
            assert!(nodes.is_empty());
            assert!(rem.is_empty());
        }
        Err(_) => panic!("Empty input should be valid empty list"),
    }
}

#[test]
fn test_garbage_input() {
    let input = "sdafasdf < >>> invalid";
    let result = parse_root(input);
    // Should parse text "sdafasdf " then stop at <
    if let Ok((rem, _)) = result {
        assert!(!rem.trim().is_empty(), "Garbage should leave remainder");
    }
}
