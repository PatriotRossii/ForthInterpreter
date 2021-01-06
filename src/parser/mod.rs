use pest::Parser;

#[derive(Parser)]
#[grammar = "parser/forth.pest"]
pub struct ForthParser;

#[cfg(test)]
mod tests {
    use super::*;        

    #[test]
    fn test_parse_comment() {        
        ForthParser::parse(Rule::COMMENT, "(hi)").unwrap();
        ForthParser::parse(Rule::COMMENT, "(h\ni)").unwrap_err();
        ForthParser::parse(Rule::COMMENT, "(hi").unwrap_err();
    }

    #[test]
    fn test_parse_integer() {
        ForthParser::parse(Rule::integer, "123").unwrap();
        ForthParser::parse(Rule::integer, "-123").unwrap();

        ForthParser::parse(Rule::integer, "--123").unwrap_err();
        ForthParser::parse(Rule::integer, "hello").unwrap_err();
    }

    #[test]
    fn test_parse_string() {
        ForthParser::parse(Rule::string, r#""hello""#).unwrap();
        ForthParser::parse(Rule::string, r#""""#).unwrap();

        ForthParser::parse(Rule::string, r#"123"#).unwrap_err();
        ForthParser::parse(Rule::string, r#"'hello'"#).unwrap_err();
    }

    #[test]
    fn test_parse_word() {
        ForthParser::parse(Rule::ident, "hello").unwrap();
        ForthParser::parse(Rule::ident, "hello_world").unwrap();
        ForthParser::parse(Rule::ident, "second_stack_2").unwrap();

        ForthParser::parse(Rule::user_ident, "8hello").unwrap_err();
        ForthParser::parse(Rule::ident, "*").unwrap();
    }

    #[test]
    fn test_parse_expression() {
        ForthParser::parse(Rule::expression, "1 2 3 * +").unwrap();

        ForthParser::parse(Rule::expression, "2 5 *").unwrap();
        ForthParser::parse(Rule::expression, "foo").unwrap();
        ForthParser::parse(Rule::expression, "1 2 3 dup").unwrap();

    }

}