use rome_js_syntax::JsSyntaxKind;
use rome_js_syntax::JsSyntaxKind::EOF;
use rome_rowan::TextSize;
use rslint_errors::file::FileId;
use rslint_errors::Diagnostic;
use rslint_lexer::buffered_lexer::{BufferedLexer, BufferedLexerCheckpoint};
use rslint_lexer::{LexMode, Lexer, LexerReturn, TextRange, Token};
use std::collections::VecDeque;

/// The source of tokens for the parser
pub struct TokenSource<'l> {
    lexer: BufferedLexer<'l>,

    /// A list of the tokens including whitespace.
    tokens: Vec<Token>,

    lookahead: VecDeque<Lookahead>,

    lookahead_offset: usize,
}

#[derive(Debug, Copy, Clone)]
struct Lookahead {
    kind: JsSyntaxKind,
    after_newline: bool,
}

impl<'l> TokenSource<'l> {
    /// Creates a new token source.
    pub fn new(lexer: BufferedLexer<'l>) -> TokenSource<'l> {
        TokenSource {
            lexer,
            tokens: vec![],
            lookahead_offset: 0,
            lookahead: VecDeque::new(),
        }
    }

    /// Creates a new token source for the given string
    pub fn from_str(source: &'l str, file_id: FileId) -> TokenSource<'l> {
        let lexer = Lexer::from_str(source, file_id);
        let buffered = BufferedLexer::new(lexer);
        TokenSource::new(buffered)
    }

    /// Moves the token source to the first non-trivia token. Returns the lexing error
    /// for the skipped trivia and the now current token
    pub fn initialize(&mut self) -> Vec<Diagnostic> {
        self.next_non_trivia_token()
    }

    fn next_non_trivia_token(&mut self) -> Vec<Diagnostic> {
        self.lookahead.pop_front();

        let mut diagnostics = vec![];
        let mut skipped = 0;

        for mut result in &mut self.lexer {
            skipped += 1;

            self.tokens.push(*result.token());

            if let Some(diagnostic) = result.take_diagnostic() {
                diagnostics.push(diagnostic.as_ref().clone());
            }

            if !result.kind().is_trivia() {
                break;
            }
        }

        if self.lookahead_offset != 0 {
            debug_assert!(self.lookahead_offset >= skipped);
            self.lookahead_offset -= skipped;
        }

        diagnostics
    }

    #[inline(always)]
    pub fn current(&self) -> &Token {
        self.tokens.last().unwrap()
    }

    /// Gets the kind of the nth non-trivia token
    #[inline(always)]
    pub fn nth(&mut self, n: usize) -> JsSyntaxKind {
        if n == 0 {
            self.current().kind()
        } else {
            self.lookahead(n).map_or(EOF, |lookahead| lookahead.kind)
        }
    }

    /// Returns true if the nth non-trivia token is preceded by a line break
    #[inline(always)]
    pub fn has_nth_preceding_line_break(&mut self, n: usize) -> bool {
        if n == 0 {
            self.current().has_preceding_line_break()
        } else {
            self.lookahead(n)
                .map_or(false, |lookahead| lookahead.after_newline)
        }
    }

    #[inline(always)]
    fn lookahead(&mut self, n: usize) -> Option<Lookahead> {
        assert_ne!(n, 0);

        if let Some(lookahead) = self.lookahead.get(n - 1) {
            return Some(*lookahead);
        }

        let mut iter = self.lexer.lookahead().skip(self.lookahead_offset);
        let mut remaining = n - self.lookahead.len();
        for item in iter {
            self.lookahead_offset += 1;

            if !item.kind().is_trivia() {
                remaining -= 1;

                let lookahead = Lookahead {
                    after_newline: item.has_preceding_line_break(),
                    kind: item.kind(),
                };

                self.lookahead.push_back(lookahead);

                if remaining == 0 {
                    return Some(lookahead);
                }
            }
        }

        None
    }

    pub fn rewind(&mut self, checkpoint: TokenSourceCheckpoint) {
        assert!(self.tokens.len() >= checkpoint.tokens_len as usize);
        self.tokens.truncate(checkpoint.tokens_len as usize);
        self.lexer.rewind(checkpoint.lexer);
        self.lookahead.clear();
        self.lookahead_offset = 0;
    }

    pub fn checkpoint(&self) -> TokenSourceCheckpoint {
        TokenSourceCheckpoint {
            tokens_len: self.tokens.len() as u32,
            lexer: self.lexer.checkpoint(),
        }
    }

    /// Returns the last token
    #[inline(always)]
    pub fn last_token(&self) -> Option<&Token> {
        self.tokens
            .iter()
            .rev()
            // Skip the current token
            .skip(1)
            .find(|t| !t.kind().is_trivia())
    }

    /// Returns the source text
    #[inline(always)]
    pub fn source(&self) -> &'l str {
        self.lexer.source()
    }

    /// Bumps the current token and moves the parser to the next non-trivia token
    #[inline(always)]
    pub fn bump(&mut self) -> Vec<Diagnostic> {
        if self.current().kind() == EOF {
            vec![]
        } else {
            self.next_non_trivia_token()
        }
    }

    pub fn re_lex(&mut self, mode: LexMode) -> LexerReturn {
        let current = *self.current();
        let result = self.lexer.re_lex(current, mode);

        if result.kind() != current.kind() {
            self.tokens.pop();
            self.tokens.push(*result.token());
        }

        result
    }

    /// Returns the byte offset of the current token from the start of the source document
    #[inline(always)]
    pub fn position(&self) -> TextSize {
        self.current().range().start()
    }

    #[inline(always)]
    pub fn cur_token_idx(&self) -> usize {
        self.tokens.len() - 1
    }

    pub fn finish(self) -> Vec<Token> {
        self.tokens
    }
}

#[derive(Debug)]
pub struct TokenSourceCheckpoint {
    lexer: BufferedLexerCheckpoint,
    /// A `u32` should be enough because `TextSize` is also limited to `u32`.
    /// The worst case is a document where every character is its own token. This would
    /// result in `u32::MAX` tokens
    tokens_len: u32,
}
