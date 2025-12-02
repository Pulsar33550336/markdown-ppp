#![cfg(test)]
use rstest::rstest;

#[test]
fn text_with_newlines_formats_to_single_line() {
    let input = r#"line1 line1 line1
line2 line2 line2 line2 line2"#;

    let expected = "line1 line1 line1 line2 line2 line2 line2 line2";

    let config = crate::printer::config::Config::default();
    let doc = crate::parser::parse_markdown(crate::parser::MarkdownParserState::default(), input)
        .unwrap();

    let result = crate::printer::render_markdown(&doc, config);
    assert_eq!(expected, result);
}

#[test]
fn text_with_smart_wrapping_disabled() {
    let input = r#"A long line of text that will definitely be wrapped. This line is intentionally made very long to ensure that it exceeds the default width of 80 characters."#;

    let expected_smart = r#"A long line of text that will definitely be wrapped. This line is intentionally
made very long to ensure that it exceeds the default width of 80 characters."#;

    let _expected_no_smart = r#"A long line of text that will definitely be wrapped. This line is intentionally
made very long to ensure that it exceeds the default width of 80 characters."#;

    let doc = crate::parser::parse_markdown(crate::parser::MarkdownParserState::default(), input)
        .unwrap();

    // Test with smart wrapping enabled (default)
    let config_smart = crate::printer::config::Config::default();
    let result_smart = crate::printer::render_markdown(&doc, config_smart);
    assert_eq!(expected_smart, result_smart);

    // Test with smart wrapping disabled
    let config_no_smart = crate::printer::config::Config::default().with_smart_wrapping(false);
    let result_no_smart = crate::printer::render_markdown(&doc, config_no_smart);
    assert_eq!(expected_smart, result_no_smart);
}

#[rstest(
    input,
    expected,
    case("line1\nline2", "line1 line2"),
    case("word1 word2\nword3 word4", "word1 word2 word3 word4"),
    case("first\nsecond\nthird", "first second third")
)]
fn text_newlines_normalize_to_spaces(input: &str, expected: &str) {
    let config = crate::printer::config::Config::default();
    let doc = crate::parser::parse_markdown(crate::parser::MarkdownParserState::default(), input)
        .unwrap();

    let result = crate::printer::render_markdown(&doc, config);
    assert_eq!(expected, result);
}
