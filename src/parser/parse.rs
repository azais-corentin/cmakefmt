use anyhow::{bail, Result};
use logos::Logos;

use super::ast::{Argument, CommandInvocation, File, FileElement, Span};
use super::token::Token;

struct Parser<'a> {
    source: &'a str,
    tokens: Vec<(Token, std::ops::Range<usize>)>,
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(source: &'a str) -> Self {
        let mut tokens = Vec::new();
        let mut lex = Token::lexer(source);
        while let Some(result) = lex.next() {
            let span = lex.span();
            match result {
                Ok(tok) => tokens.push((tok, span)),
                Err(()) => {
                    // Unknown byte — treat as unquoted text so we don't lose content
                    tokens.push((Token::UnquotedText, span));
                }
            }
        }
        Parser {
            source,
            tokens,
            pos: 0,
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos).map(|(t, _)| t)
    }

    fn peek_span(&self) -> Option<Span> {
        self.tokens
            .get(self.pos)
            .map(|(_, r)| Span::new(r.start, r.end))
    }

    fn advance(&mut self) -> Option<(Token, Span)> {
        if self.pos < self.tokens.len() {
            let (tok, range) = &self.tokens[self.pos];
            let result = (tok.clone(), Span::new(range.start, range.end));
            self.pos += 1;
            Some(result)
        } else {
            None
        }
    }

    fn at_end(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    fn skip_spaces(&mut self) {
        while let Some(Token::Space) = self.peek() {
            self.pos += 1;
        }
    }
}

/// Parse CMake source text into an AST.
pub fn parse(source: &str) -> Result<File> {
    let mut parser = Parser::new(source);
    let elements = parse_file_elements(&mut parser)?;
    Ok(File { elements })
}

fn parse_file_elements(p: &mut Parser) -> Result<Vec<FileElement>> {
    let mut elements = Vec::new();

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
            }
            Some(Token::LineComment) => {
                let (_, span) = p.advance().unwrap();
                elements.push(FileElement::LineComment(span));
            }
            Some(Token::UnquotedText) => {
                // This should be a command invocation: identifier '(' args ')'
                let cmd = parse_command_invocation(p)?;
                elements.push(FileElement::Command(cmd));
            }
            Some(other) => {
                bail!(
                    "unexpected token {:?} at byte offset {}",
                    other,
                    p.peek_span().map(|s| s.start).unwrap_or(0)
                );
            }
            None => break,
        }
    }

    Ok(elements)
}

fn parse_command_invocation(p: &mut Parser) -> Result<CommandInvocation> {
    // Consume command name (UnquotedText)
    let (_, name_span) = p.advance().unwrap();

    // Skip spaces between name and '('
    p.skip_spaces();

    // Optional space_before_paren handling — just consume '('
    match p.peek() {
        Some(Token::LParen) => {
            p.advance();
        }
        other => {
            bail!(
                "expected '(' after command name '{}' at byte {}, got {:?}",
                name_span.text(p.source),
                name_span.end,
                other
            );
        }
    }

    // Parse arguments
    let arguments = parse_arguments(p)?;

    // Expect ')'
    match p.peek() {
        Some(Token::RParen) => {
            p.advance();
        }
        other => {
            bail!(
                "expected ')' to close command '{}' at byte {}, got {:?}",
                name_span.text(p.source),
                name_span.start,
                other
            );
        }
    }

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
        arguments,
        trailing_comment,
    })
}

fn parse_arguments(p: &mut Parser) -> Result<Vec<Argument>> {
    let mut args = Vec::new();

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
                        bail!("expected ')' to close paren group, got {:?}", other);
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
            // Bracket comment inside argument list — treat like line comment
            let (_, span) = p.advance().unwrap();
            Ok(Argument::LineComment(span))
        }
        other => {
            bail!("expected argument, got {:?}", other);
        }
    }
}
