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
