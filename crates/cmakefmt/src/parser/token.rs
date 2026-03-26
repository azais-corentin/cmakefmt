use logos::{Lexer, Logos};

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    #[regex(r"[ \t]+")]
    Space,

    #[regex(r"\r?\n")]
    Newline,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[regex(r"\[=*\[", bracket_arg_callback)]
    BracketArgument,

    #[regex(r"#\[=*\[", bracket_comment_callback, priority = 10)]
    BracketComment,

    #[token("#", line_comment_callback)]
    LineComment,

    #[token("\"", quoted_arg_callback)]
    QuotedArgument,

    #[regex(r#"([^ \t\r\n()#"\\\[]|\\.)([^ \t\r\n()#"\\]|\\.)*"#)]
    UnquotedText,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Space => f.write_str("<space>"),
            Token::Newline => f.write_str("<newline>"),
            Token::LParen => f.write_str("'('"),
            Token::RParen => f.write_str("')'"),
            Token::BracketArgument => f.write_str("bracket argument"),
            Token::BracketComment => f.write_str("bracket comment"),
            Token::LineComment => f.write_str("line comment"),
            Token::QuotedArgument => f.write_str("quoted string"),
            Token::UnquotedText => f.write_str("unquoted text"),
        }
    }
}

fn bracket_arg_callback(lex: &mut Lexer<Token>) -> bool {
    let opening = lex.slice();
    let eq_count = opening.len() - 2;
    let mut closer = String::with_capacity(eq_count + 2);
    closer.push(']');
    for _ in 0..eq_count {
        closer.push('=');
    }
    closer.push(']');

    let remainder = lex.remainder();
    if let Some(end) = memchr::memmem::find(remainder.as_bytes(), closer.as_bytes()) {
        lex.bump(end + closer.len());
        true
    } else {
        false
    }
}

fn bracket_comment_callback(lex: &mut Lexer<Token>) -> bool {
    let opening = lex.slice();
    let eq_count = opening.len() - 3;
    let mut closer = String::with_capacity(eq_count + 2);
    closer.push(']');
    for _ in 0..eq_count {
        closer.push('=');
    }
    closer.push(']');

    let remainder = lex.remainder();
    if let Some(end) = memchr::memmem::find(remainder.as_bytes(), closer.as_bytes()) {
        lex.bump(end + closer.len());
        true
    } else {
        false
    }
}

fn line_comment_callback(lex: &mut Lexer<Token>) -> bool {
    let remainder = lex.remainder();
    // If `#` is followed by `[=*[`, this is a bracket comment — reject so
    // the higher-priority BracketComment token can match instead.
    let bytes = remainder.as_bytes();
    if bytes.first() == Some(&b'[') {
        // Check for `[=*[` pattern
        let mut j = 1;
        while j < bytes.len() && bytes[j] == b'=' {
            j += 1;
        }
        if j < bytes.len() && bytes[j] == b'[' {
            return false;
        }
    }
    // Consume to end of line
    if let Some(nl) = memchr::memchr(b'\n', remainder.as_bytes()) {
        lex.bump(nl);
    } else {
        lex.bump(remainder.len());
    }
    true
}

fn quoted_arg_callback(lex: &mut Lexer<Token>) -> bool {
    let bytes = lex.remainder().as_bytes();
    let mut i = 0;

    while let Some(offset) = memchr::memchr2(b'"', b'\\', &bytes[i..]) {
        let pos = i + offset;
        match bytes[pos] {
            b'"' => {
                lex.bump(pos + 1);
                return true;
            }
            b'\\' if pos + 1 < bytes.len() => {
                i = pos + 2;
            }
            _ => {
                // Backslash at end of input — no char to escape
                break;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tokenize_with_slices(input: &str) -> Vec<(Token, &str)> {
        let mut lex = Token::lexer(input);
        let mut result = Vec::new();
        while let Some(tok_result) = lex.next() {
            let tok = tok_result.unwrap_or(Token::UnquotedText);
            result.push((tok, lex.slice()));
        }
        result
    }

    fn token_types(input: &str) -> Vec<Token> {
        tokenize_with_slices(input)
            .into_iter()
            .map(|(t, _)| t)
            .collect()
    }

    #[test]
    fn empty_input_produces_no_tokens() {
        assert!(tokenize_with_slices("").is_empty());
    }

    #[test]
    fn simple_command_token_sequence() {
        let tokens = token_types("if(A AND B)");
        assert_eq!(
            tokens,
            vec![
                Token::UnquotedText, // if
                Token::LParen,
                Token::UnquotedText, // A
                Token::Space,
                Token::UnquotedText, // AND
                Token::Space,
                Token::UnquotedText, // B
                Token::RParen,
            ]
        );
    }

    #[test]
    fn spaces_and_tabs_combined_into_single_space_token() {
        let tokens = tokenize_with_slices("a  \t  b");
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].0, Token::UnquotedText);
        assert_eq!(tokens[1].0, Token::Space);
        assert_eq!(tokens[1].1, "  \t  ");
        assert_eq!(tokens[2].0, Token::UnquotedText);
    }

    #[test]
    fn newline_lf() {
        let tokens = token_types("a\nb");
        assert_eq!(
            tokens,
            vec![Token::UnquotedText, Token::Newline, Token::UnquotedText]
        );
    }

    #[test]
    fn newline_crlf() {
        let tokens = token_types("a\r\nb");
        assert_eq!(
            tokens,
            vec![Token::UnquotedText, Token::Newline, Token::UnquotedText]
        );
    }

    #[test]
    fn bracket_argument_depth_0() {
        let tokens = tokenize_with_slices("[[content]]");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].0, Token::BracketArgument);
        assert_eq!(tokens[0].1, "[[content]]");
    }

    #[test]
    fn bracket_argument_depth_1() {
        let tokens = tokenize_with_slices("[=[content]=]");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].0, Token::BracketArgument);
        assert_eq!(tokens[0].1, "[=[content]=]");
    }

    #[test]
    fn bracket_argument_depth_3() {
        let tokens = tokenize_with_slices("[===[nested ]] content]===]");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].0, Token::BracketArgument);
        assert_eq!(tokens[0].1, "[===[nested ]] content]===]");
    }

    #[test]
    fn unclosed_bracket_argument_produces_error() {
        // Unclosed bracket arg: the callback returns false, so logos produces an Error
        let mut lex = Token::lexer("[=[unclosed");
        let result = lex.next();
        // Should not produce a valid BracketArgument
        assert!(result.is_some());
        let tok = result.unwrap();
        assert!(tok.is_err() || tok.unwrap() != Token::BracketArgument);
    }

    #[test]
    fn bracket_comment_takes_priority_over_line_comment() {
        let tokens = tokenize_with_slices("#[=[comment]=]");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].0, Token::BracketComment);
        assert_eq!(tokens[0].1, "#[=[comment]=]");
    }

    #[test]
    fn bracket_comment_depth_0() {
        let tokens = tokenize_with_slices("#[[comment]]");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].0, Token::BracketComment);
    }

    #[test]
    fn line_comment_rejects_bracket_pattern() {
        // `#[=[` should NOT produce LineComment — it should be BracketComment
        let tokens = tokenize_with_slices("#[=[text]=]");
        assert_eq!(tokens[0].0, Token::BracketComment);
    }

    #[test]
    fn line_comment_basic() {
        let tokens = tokenize_with_slices("# this is a comment\n");
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].0, Token::LineComment);
        assert_eq!(tokens[0].1, "# this is a comment");
        assert_eq!(tokens[1].0, Token::Newline);
    }

    #[test]
    fn line_comment_at_eof() {
        let tokens = tokenize_with_slices("# comment at eof");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].0, Token::LineComment);
        assert_eq!(tokens[0].1, "# comment at eof");
    }

    #[test]
    fn quoted_string_basic() {
        let tokens = tokenize_with_slices("\"hello world\"");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].0, Token::QuotedArgument);
        assert_eq!(tokens[0].1, "\"hello world\"");
    }

    #[test]
    fn quoted_string_with_escapes() {
        let tokens = tokenize_with_slices(r#""hello \"world\"""#);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].0, Token::QuotedArgument);
        assert_eq!(tokens[0].1, r#""hello \"world\"""#);
    }

    #[test]
    fn unclosed_quoted_string_produces_error() {
        let mut lex = Token::lexer("\"unclosed");
        let result = lex.next();
        assert!(result.is_some());
        let tok = result.unwrap();
        // Should fail — callback returns false for unclosed quote
        assert!(tok.is_err());
    }

    #[test]
    fn backslash_at_eof_in_quoted_string() {
        let mut lex = Token::lexer("\"hello\\");
        let result = lex.next();
        assert!(result.is_some());
        let tok = result.unwrap();
        assert!(tok.is_err());
    }

    #[test]
    fn unquoted_text_with_escapes() {
        let tokens = tokenize_with_slices("a\\ b");
        // `a\ ` is a single unquoted token because `\\ ` (backslash + space) is an escape
        assert_eq!(tokens[0].0, Token::UnquotedText);
    }

    #[test]
    fn parentheses_are_separate_tokens() {
        let tokens = token_types("()");
        assert_eq!(tokens, vec![Token::LParen, Token::RParen]);
    }

    #[test]
    fn multiple_token_types_in_sequence() {
        let tokens = token_types("set(VAR \"value\")");
        assert_eq!(
            tokens,
            vec![
                Token::UnquotedText, // set
                Token::LParen,
                Token::UnquotedText, // VAR
                Token::Space,
                Token::QuotedArgument, // "value"
                Token::RParen,
            ]
        );
    }
}
