//! Token definitions for the lexer

use crate::{LexerReturn, TextRange, TextSize};
use rome_js_syntax::JsSyntaxKind;
use rslint_errors::Diagnostic;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Token {
    kind: JsSyntaxKind,
    range: TextRange,
    after_new_line: bool,
}

impl Token {
    pub fn new(kind: JsSyntaxKind, offset: usize, len: usize) -> Self {
        Self {
            kind,
            range: TextRange::at(TextSize::from(offset as u32), TextSize::from(len as u32)),
            after_new_line: false,
        }
    }

    pub(crate) fn set_after_new_line(&mut self, after_new_line: bool) {
        self.after_new_line = after_new_line
    }

    /// Returns the kind of the token
    pub fn kind(&self) -> JsSyntaxKind {
        self.kind
    }

    /// Returns the start byte offset of the token in the source text
    pub fn start(&self) -> TextSize {
        self.range.start()
    }

    /// Returns the end byte offset of the token in the source text
    pub fn end(&self) -> TextSize {
        self.range.end()
    }

    /// Returns the range in the source text
    pub fn range(&self) -> &TextRange {
        &self.range
    }

    /// Returns the token's length in bytes
    pub fn len(&self) -> TextSize {
        self.range.len()
    }

    /// Returns `true` if this token is preceded by a line break
    pub fn has_preceding_line_break(&self) -> bool {
        self.after_new_line
    }
}

pub(crate) struct LexedToken {
    pub(crate) kind: JsSyntaxKind,
    pub(crate) diagnostic: Option<Box<Diagnostic>>,
}

impl LexedToken {
    pub fn ok(kind: JsSyntaxKind) -> Self {
        Self::new(kind, None)
    }

    pub fn with_diagnostic(kind: JsSyntaxKind, diagnostic: Box<Diagnostic>) -> Self {
        Self::new(kind, Some(diagnostic))
    }

    pub fn new(kind: JsSyntaxKind, diagnostic: Option<Box<Diagnostic>>) -> Self {
        Self { kind, diagnostic }
    }

    pub fn kind(&self) -> JsSyntaxKind {
        self.kind
    }

    pub fn into_lexer_return(self, offset: usize, len: usize) -> LexerReturn {
        let token = Token::new(self.kind, offset, len);

        LexerReturn::new(token, self.diagnostic)
    }
}

macro_rules! lexed_token {
    ($tok:ident) => {
        $crate::token::LexedToken::ok($crate::JsSyntaxKind::$tok)
    };
    ($tok:tt) => {
        $crate::token::LexedToken::ok(T![$tok])
    };
}
