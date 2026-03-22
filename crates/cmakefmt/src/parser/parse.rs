use anyhow::{Result, bail};
use logos::Logos;
use tracing::info_span;

use super::ast::{Argument, CommandInvocation, File, FileElement, Span};
use super::token::Token;
use crate::instrumentation::{EVENT_PARSER_COMMAND, EVENT_PARSER_FILE};

/// Convert a byte offset into a 1-based (line, column) pair.
fn byte_offset_to_line_col(source: &str, offset: usize) -> (usize, usize) {
    let offset = offset.min(source.len());
    let before = &source[..offset];
    let line = before.bytes().filter(|&b| b == b'\n').count() + 1;
    let col = before.len() - before.rfind('\n').map_or(0, |n| n + 1) + 1;
    (line, col)
}

fn describe_token(tok: Option<&Token>) -> String {
    match tok {
        Some(t) => t.to_string(),
        None => "end of input".to_string(),
    }
}

/// Streaming parser that pulls tokens from the logos lexer on demand.
///
/// Uses a single-element lookahead (`peeked`) instead of collecting all tokens
/// into a Vec. This eliminates a ~50K-element intermediate allocation for large
/// files and improves cache locality.
struct Parser<'a> {
    source: &'a str,
    lexer: logos::Lexer<'a, Token>,
    /// One-element lookahead: `Some((token, span))` if peeked, `None` if exhausted.
    peeked: Option<(Token, std::ops::Range<usize>)>,
}

impl<'a> Parser<'a> {
    fn new(source: &'a str) -> Self {
        let mut lexer = Token::lexer(source);
        let peeked = Self::next_token(&mut lexer);
        Parser {
            source,
            lexer,
            peeked,
        }
    }

    /// Pull the next token from the lexer, converting errors to UnquotedText.
    fn next_token(lexer: &mut logos::Lexer<'a, Token>) -> Option<(Token, std::ops::Range<usize>)> {
        lexer.next().map(|result| {
            let span = lexer.span();
            match result {
                Ok(tok) => (tok, span),
                // Unknown byte — treat as unquoted text so we don't lose content
                Err(()) => (Token::UnquotedText, span),
            }
        })
    }

    fn peek(&self) -> Option<&Token> {
        self.peeked.as_ref().map(|(t, _)| t)
    }

    fn peek_span(&self) -> Option<Span> {
        self.peeked.as_ref().map(|(_, r)| Span::new(r.start, r.end))
    }

    fn advance(&mut self) -> Option<(Token, Span)> {
        let current = self.peeked.take()?;
        self.peeked = Self::next_token(&mut self.lexer);
        Some((current.0, Span::new(current.1.start, current.1.end)))
    }

    fn at_end(&self) -> bool {
        self.peeked.is_none()
    }

    fn skip_spaces(&mut self) {
        while let Some(Token::Space) = self.peek() {
            self.advance();
        }
    }
}

/// Parse CMake source text into an AST.
pub fn parse(source: &str) -> Result<File> {
    let _stage = info_span!(EVENT_PARSER_FILE, input_bytes = source.len()).entered();
    let mut parser = Parser::new(source);
    let elements = parse_file_elements(&mut parser)?;
    Ok(File { elements })
}

fn parse_file_elements(p: &mut Parser) -> Result<Vec<FileElement>> {
    let estimated_elements = p.source.len() / 70;
    let mut elements = Vec::with_capacity(estimated_elements);

    while !p.at_end() {
        match p.peek() {
            Some(Token::Space) => {
                p.skip_spaces();
                // After skipping spaces, check what follows
                continue;
            }
            Some(Token::Newline) => {
                p.advance();
                elements.push(FileElement::BlankLine);
            }
            Some(Token::BracketComment) => {
                let (_, span) = p.advance().unwrap();
                elements.push(FileElement::BracketComment(span));
                // Consume trailing newline — it's the end of the comment, not a blank line
                if matches!(p.peek(), Some(Token::Newline)) {
                    p.advance();
                }
            }
            Some(Token::LineComment) => {
                let (_, span) = p.advance().unwrap();
                elements.push(FileElement::LineComment(span));
                // Consume trailing newline — it's the end of the comment, not a blank line
                if matches!(p.peek(), Some(Token::Newline)) {
                    p.advance();
                }
            }
            Some(Token::UnquotedText) => {
                // This should be a command invocation: identifier '(' args ')'
                let cmd = parse_command_invocation(p)?;
                elements.push(FileElement::Command(cmd));
            }
            Some(other) => {
                let offset = p.peek_span().map(|s| s.start).unwrap_or(0);
                let (line, col) = byte_offset_to_line_col(p.source, offset);
                bail!("unexpected token {other} at {line}:{col}");
            }
            None => break,
        }
    }

    Ok(elements)
}

fn parse_command_invocation(p: &mut Parser) -> Result<CommandInvocation> {
    let _stage = info_span!(EVENT_PARSER_COMMAND).entered();

    // Consume command name (UnquotedText)
    let (_, name_span) = p.advance().unwrap();

    // Skip spaces between name and '('
    p.skip_spaces();

    // Optional space_before_paren handling — just consume '('
    let open_paren_span = match p.peek() {
        Some(Token::LParen) => {
            let (_, open_span) = p.advance().unwrap();
            open_span
        }
        other => {
            let (line, col) = byte_offset_to_line_col(p.source, name_span.end);
            bail!(
                "expected '(' after command name '{}' at {line}:{col}, got {}",
                name_span.text(p.source),
                describe_token(other)
            );
        }
    };

    // Parse arguments
    let arguments = parse_arguments(p)?;

    // Expect ')'
    let close_paren_span = match p.peek() {
        Some(Token::RParen) => {
            let (_, close_span) = p.advance().unwrap();
            close_span
        }
        other => {
            let (line, col) = byte_offset_to_line_col(p.source, name_span.start);
            bail!(
                "expected ')' to close command '{}' at {line}:{col}, got {}",
                name_span.text(p.source),
                describe_token(other)
            );
        }
    };

    // Consume optional trailing whitespace + line comment before newline
    p.skip_spaces();

    let trailing_comment = if matches!(p.peek(), Some(Token::LineComment)) {
        let (_, span) = p.advance().unwrap();
        Some(span)
    } else {
        None
    };

    // Consume the trailing newline if present
    if matches!(p.peek(), Some(Token::Newline)) {
        p.advance();
    }

    Ok(CommandInvocation {
        name: name_span,
        open_paren: open_paren_span,
        close_paren: close_paren_span,
        arguments,
        trailing_comment,
    })
}

fn parse_arguments(p: &mut Parser) -> Result<Vec<Argument>> {
    let mut args = Vec::with_capacity(8);

    loop {
        // Skip separators (spaces, newlines)
        // But line comments inside arg lists become Argument::LineComment
        match p.peek() {
            Some(Token::Space) => {
                p.advance();
                continue;
            }
            Some(Token::Newline) => {
                p.advance();
                continue;
            }
            Some(Token::RParen) | None => break,
            Some(Token::LineComment) => {
                let (_, span) = p.advance().unwrap();
                args.push(Argument::LineComment(span));
                continue;
            }
            Some(Token::LParen) => {
                // Nested paren group
                p.advance(); // consume '('
                let inner = parse_arguments(p)?;
                match p.peek() {
                    Some(Token::RParen) => {
                        p.advance();
                    }
                    other => {
                        let offset = p.peek_span().map(|s| s.start).unwrap_or(0);
                        let (line, col) = byte_offset_to_line_col(p.source, offset);
                        bail!(
                            "expected ')' to close paren group at {line}:{col}, got {}",
                            describe_token(other)
                        );
                    }
                }
                args.push(Argument::ParenGroup { arguments: inner });
                continue;
            }
            _ => {}
        }

        // Parse a single argument
        let arg = parse_single_argument(p)?;
        args.push(arg);
    }

    Ok(args)
}

fn parse_single_argument(p: &mut Parser) -> Result<Argument> {
    match p.peek() {
        Some(Token::BracketArgument) => {
            let (_, span) = p.advance().unwrap();
            Ok(Argument::Bracket(span))
        }
        Some(Token::QuotedArgument) => {
            let (_, span) = p.advance().unwrap();
            Ok(Argument::Quoted(span))
        }
        Some(Token::UnquotedText) => {
            let (_, span) = p.advance().unwrap();

            // Handle unquoted_legacy: if immediately followed by QuotedArgument
            // with no separation, merge into a single unquoted span.
            // e.g., -Da="b c" is UnquotedText + QuotedArgument with no space.
            let mut end = span.end;
            loop {
                match p.peek() {
                    Some(Token::QuotedArgument) => {
                        let next_span = p.peek_span().unwrap();
                        if next_span.start == end {
                            // Adjacent — merge
                            p.advance();
                            end = next_span.end;
                        } else {
                            break;
                        }
                    }
                    Some(Token::UnquotedText) => {
                        let next_span = p.peek_span().unwrap();
                        if next_span.start == end {
                            // Adjacent unquoted text (shouldn't happen with logos,
                            // but defensive)
                            p.advance();
                            end = next_span.end;
                        } else {
                            break;
                        }
                    }
                    _ => break,
                }
            }

            Ok(Argument::Unquoted(Span::new(span.start, end)))
        }
        Some(Token::BracketComment) => {
            // Bracket comment inside argument list — distinct from line comment
            let (_, span) = p.advance().unwrap();
            Ok(Argument::BracketComment(span))
        }
        other => {
            let offset = p.peek_span().map(|s| s.start).unwrap_or(0);
            let (line, col) = byte_offset_to_line_col(p.source, offset);
            bail!(
                "expected argument at {line}:{col}, got {}",
                describe_token(other)
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // byte_offset_to_line_col
    // -----------------------------------------------------------------------

    #[test]
    fn line_col_empty_input() {
        assert_eq!(byte_offset_to_line_col("", 0), (1, 1));
    }

    #[test]
    fn line_col_single_line() {
        assert_eq!(byte_offset_to_line_col("hello", 0), (1, 1));
        assert_eq!(byte_offset_to_line_col("hello", 3), (1, 4));
        assert_eq!(byte_offset_to_line_col("hello", 5), (1, 6));
    }

    #[test]
    fn line_col_multi_line() {
        let src = "ab\ncd\nef";
        assert_eq!(byte_offset_to_line_col(src, 0), (1, 1)); // 'a'
        assert_eq!(byte_offset_to_line_col(src, 2), (1, 3)); // '\n'
        assert_eq!(byte_offset_to_line_col(src, 3), (2, 1)); // 'c'
        assert_eq!(byte_offset_to_line_col(src, 6), (3, 1)); // 'e'
    }

    #[test]
    fn line_col_offset_past_end() {
        assert_eq!(byte_offset_to_line_col("abc", 100), (1, 4));
    }

    #[test]
    fn line_col_at_newline_boundary() {
        let src = "a\nb";
        assert_eq!(byte_offset_to_line_col(src, 1), (1, 2)); // at '\n'
        assert_eq!(byte_offset_to_line_col(src, 2), (2, 1)); // at 'b'
    }

    // -----------------------------------------------------------------------
    // parse — empty / minimal inputs
    // -----------------------------------------------------------------------

    #[test]
    fn parse_empty_file() {
        let file = parse("").unwrap();
        assert!(file.elements.is_empty());
    }

    #[test]
    fn parse_blank_lines() {
        let file = parse("\n\n\n").unwrap();
        assert_eq!(file.elements.len(), 3);
        for elem in &file.elements {
            assert!(matches!(elem, FileElement::BlankLine));
        }
    }

    #[test]
    fn parse_whitespace_only() {
        let file = parse("   \t  ").unwrap();
        assert!(file.elements.is_empty());
    }

    // -----------------------------------------------------------------------
    // parse — commands
    // -----------------------------------------------------------------------

    #[test]
    fn parse_simple_command() {
        let src = "message(STATUS \"hello\")\n";
        let file = parse(src).unwrap();
        assert_eq!(file.elements.len(), 1);

        let cmd = match &file.elements[0] {
            FileElement::Command(cmd) => cmd,
            other => panic!("expected Command, got {other:?}"),
        };

        assert_eq!(cmd.name.text(src), "message");
        assert_eq!(cmd.arguments.len(), 2);

        assert!(matches!(&cmd.arguments[0], Argument::Unquoted(s) if s.text(src) == "STATUS"));
        assert!(matches!(&cmd.arguments[1], Argument::Quoted(s) if s.text(src) == "\"hello\""));
        assert!(cmd.trailing_comment.is_none());
    }

    #[test]
    fn parse_command_no_args() {
        let src = "cmake_minimum_required()\n";
        let file = parse(src).unwrap();
        let cmd = match &file.elements[0] {
            FileElement::Command(cmd) => cmd,
            other => panic!("expected Command, got {other:?}"),
        };
        assert!(cmd.arguments.is_empty());
    }

    #[test]
    fn parse_multiple_commands() {
        let src = "set(A 1)\nset(B 2)\n";
        let file = parse(src).unwrap();
        assert_eq!(file.elements.len(), 2);
        assert!(matches!(&file.elements[0], FileElement::Command(_)));
        assert!(matches!(&file.elements[1], FileElement::Command(_)));
    }

    #[test]
    fn parse_command_with_trailing_comment() {
        let src = "set(VAR val) # a comment\n";
        let file = parse(src).unwrap();
        let cmd = match &file.elements[0] {
            FileElement::Command(cmd) => cmd,
            other => panic!("expected Command, got {other:?}"),
        };
        assert!(cmd.trailing_comment.is_some());
        let comment = cmd.trailing_comment.unwrap();
        assert_eq!(comment.text(src), "# a comment");
    }

    // -----------------------------------------------------------------------
    // parse — comments
    // -----------------------------------------------------------------------

    #[test]
    fn parse_line_comment() {
        let src = "# top-level comment\n";
        let file = parse(src).unwrap();
        assert_eq!(file.elements.len(), 1);
        assert!(
            matches!(&file.elements[0], FileElement::LineComment(s) if s.text(src) == "# top-level comment")
        );
    }

    #[test]
    fn parse_bracket_comment() {
        let src = "#[[bracket comment]]\n";
        let file = parse(src).unwrap();
        assert_eq!(file.elements.len(), 1);
        assert!(matches!(&file.elements[0], FileElement::BracketComment(_)));
    }

    // -----------------------------------------------------------------------
    // parse — arguments
    // -----------------------------------------------------------------------

    #[test]
    fn parse_bracket_argument() {
        let src = "cmd([=[bracket arg]=])\n";
        let file = parse(src).unwrap();
        let cmd = match &file.elements[0] {
            FileElement::Command(cmd) => cmd,
            other => panic!("expected Command, got {other:?}"),
        };
        assert_eq!(cmd.arguments.len(), 1);
        assert!(
            matches!(&cmd.arguments[0], Argument::Bracket(s) if s.text(src) == "[=[bracket arg]=]")
        );
    }

    #[test]
    fn parse_argument_merging() {
        // Adjacent unquoted+quoted should merge into single Unquoted argument
        let src = "cmd(-Da=\"b c\")\n";
        let file = parse(src).unwrap();
        let cmd = match &file.elements[0] {
            FileElement::Command(cmd) => cmd,
            other => panic!("expected Command, got {other:?}"),
        };
        assert_eq!(cmd.arguments.len(), 1);
        assert!(matches!(&cmd.arguments[0], Argument::Unquoted(s) if s.text(src) == "-Da=\"b c\""));
    }

    #[test]
    fn parse_parenthesized_group() {
        let src = "cmd((a b c))\n";
        let file = parse(src).unwrap();
        let cmd = match &file.elements[0] {
            FileElement::Command(cmd) => cmd,
            other => panic!("expected Command, got {other:?}"),
        };
        assert_eq!(cmd.arguments.len(), 1);
        match &cmd.arguments[0] {
            Argument::ParenGroup { arguments } => {
                assert_eq!(arguments.len(), 3);
            }
            other => panic!("expected ParenGroup, got {other:?}"),
        }
    }

    #[test]
    fn parse_comment_in_args() {
        let src = "cmd(arg1 # inline comment\n  arg2)\n";
        let file = parse(src).unwrap();
        let cmd = match &file.elements[0] {
            FileElement::Command(cmd) => cmd,
            other => panic!("expected Command, got {other:?}"),
        };
        assert_eq!(cmd.arguments.len(), 3);
        assert!(matches!(&cmd.arguments[0], Argument::Unquoted(_)));
        assert!(matches!(&cmd.arguments[1], Argument::LineComment(_)));
        assert!(matches!(&cmd.arguments[2], Argument::Unquoted(_)));
    }

    #[test]
    fn parse_bracket_comment_in_args() {
        let src = "cmd(arg1 #[[bc]] arg2)\n";
        let file = parse(src).unwrap();
        let cmd = match &file.elements[0] {
            FileElement::Command(cmd) => cmd,
            other => panic!("expected Command, got {other:?}"),
        };
        assert!(
            cmd.arguments
                .iter()
                .any(|a| matches!(a, Argument::BracketComment(_)))
        );
    }

    // -----------------------------------------------------------------------
    // parse — error cases
    // -----------------------------------------------------------------------

    #[test]
    fn parse_missing_open_paren_returns_error() {
        let result = parse("command\n");
        assert!(result.is_err());
    }

    #[test]
    fn parse_missing_close_paren_returns_error() {
        let result = parse("command(arg\n");
        assert!(result.is_err());
    }

    // -----------------------------------------------------------------------
    // parse — mixed content
    // -----------------------------------------------------------------------

    #[test]
    fn parse_mixed_content() {
        let src = "# comment\n\nset(A 1)\n# another\nmessage(\"hi\")\n";
        let file = parse(src).unwrap();
        assert_eq!(file.elements.len(), 5);
        assert!(matches!(&file.elements[0], FileElement::LineComment(_)));
        assert!(matches!(&file.elements[1], FileElement::BlankLine));
        assert!(matches!(&file.elements[2], FileElement::Command(_)));
        assert!(matches!(&file.elements[3], FileElement::LineComment(_)));
        assert!(matches!(&file.elements[4], FileElement::Command(_)));
    }

    #[test]
    fn parse_command_with_space_before_paren() {
        let src = "set (VAR val)\n";
        let file = parse(src).unwrap();
        let cmd = match &file.elements[0] {
            FileElement::Command(cmd) => cmd,
            other => panic!("expected Command, got {other:?}"),
        };
        assert_eq!(cmd.name.text(src), "set");
        assert_eq!(cmd.arguments.len(), 2);
    }
}
