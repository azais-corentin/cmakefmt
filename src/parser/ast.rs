/// Byte range in source text (start inclusive, end exclusive).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn text<'a>(&self, source: &'a str) -> &'a str {
        &source[self.start..self.end]
    }
}

/// Root node of a CMake file.
#[derive(Debug, Clone)]
pub struct File {
    pub elements: Vec<FileElement>,
}

/// Top-level element in a CMake file.
#[derive(Debug, Clone)]
pub enum FileElement {
    Command(CommandInvocation),
    BracketComment(Span),
    LineComment(Span),
    BlankLine,
}

/// A command invocation: `command_name(args...)`.
#[derive(Debug, Clone)]
pub struct CommandInvocation {
    pub name: Span,
    pub arguments: Vec<Argument>,
    /// Optional line comment after the closing paren on the same line.
    pub trailing_comment: Option<Span>,
}

/// An argument inside a command invocation.
#[derive(Debug, Clone)]
pub enum Argument {
    /// `[=[content]=]` — bracket delimited, verbatim.
    Bracket(Span),
    /// `"content"` — quoted, may contain escapes and variable refs.
    Quoted(Span),
    /// Unquoted text — may contain escapes, variable refs, semicolons.
    Unquoted(Span),
    /// Parenthesized group inside arguments: `(args...)`.
    ParenGroup { arguments: Vec<Argument> },
    /// A line comment inside an argument list (forces line break).
    LineComment(Span),
}
