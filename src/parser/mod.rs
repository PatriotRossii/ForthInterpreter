use pest::Parser;

#[derive(Parser)]
#[grammar = "parser/forth.pest"]
pub struct ForthParser;

#[cfg(test)]
mod tests {
    use super::ForthParser;

    #[test]
    fn test_parse_comment() {
        use super::*;        
        
        ForthParser::parse(Rule::COMMENT, "(hi)").unwrap();
        ForthParser::parse(Rule::COMMENT, "(h\ni)").unwrap_err();
        ForthParser::parse(Rule::COMMENT, "(hi").unwrap_err();
    }

    #[test]
    fn test_parse_integer() {
        use super::*;

        ForthParser::parse(Rule::integer, "123").unwrap();
        ForthParser::parse(Rule::integer, "-123").unwrap();

        ForthParser::parse(Rule::integer, "--123").unwrap_err();
        ForthParser::parse(Rule::integer, "hello").unwrap_err();
    }

    #[test]
    fn test_parse_string() {
        use super::*;

        ForthParser::parse(Rule::string, r#""hello""#).unwrap();
        ForthParser::parse(Rule::string, r#""""#).unwrap();

        ForthParser::parse(Rule::string, r#"123"#).unwrap_err();
        ForthParser::parse(Rule::string, r#"'hello'"#).unwrap_err();
    }

    #[test]
    fn test_parse_word() {
        use super::*;

        ForthParser::parse(Rule::word, "hello").unwrap();
        ForthParser::parse(Rule::word, "hello_world").unwrap();
        ForthParser::parse(Rule::word, "second_stack_2").unwrap();

        ForthParser::parse(Rule::word, "8hello").unwrap_err();
    }

}