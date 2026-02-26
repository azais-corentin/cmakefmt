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
    if let Some(end) = remainder.find(&closer) {
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
    if let Some(end) = remainder.find(&closer) {
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
    if let Some(nl) = remainder.find('\n') {
        lex.bump(nl);
    } else {
        lex.bump(remainder.len());
    }
    true
}

fn quoted_arg_callback(lex: &mut Lexer<Token>) -> bool {
    let bytes = lex.remainder().as_bytes();
    let len = bytes.len();
    let mut i = 0;

    while i < len {
        match bytes[i] {
            b'"' => {
                lex.bump(i + 1);
                return true;
            }
            b'\\' if i + 1 < len => {
                i += 2;
            }
            _ => {
                i += 1;
            }
        }
    }

    false
}
