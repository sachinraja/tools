use crate::{LexMode, Lexer, LexerReturn, TextSize, Token};
use rome_js_syntax::{JsSyntaxKind, JsSyntaxKind::EOF};
use std::collections::VecDeque;
use std::iter::FusedIterator;

/// The source of tokens for the parser
#[derive(Debug)]
pub struct BufferedLexer<'t> {
    /// Stores the information about the tokens between the `lexer`s current token and
    /// current position of the buffered token.
    lookbehind: VecDeque<LexerReturn>,

    /// Underlying lexer. May be ahead if iterated with lookahead
    inner: Lexer<'t>,
}

impl<'t> BufferedLexer<'t> {
    pub fn new(lexer: Lexer<'t>) -> BufferedLexer<'t> {
        BufferedLexer {
            inner: lexer,
            lookbehind: VecDeque::new(),
        }
    }

    #[inline(always)]
    pub fn next(&mut self) -> LexerReturn {
        if let Some(next) = self.lookbehind.pop_front() {
            next
        } else {
            self.inner.next()
        }
    }

    /// Returns the source text
    #[inline]
    pub fn source(&self) -> &'t str {
        self.inner.source()
    }

    pub fn checkpoint(&self) -> BufferedLexerCheckpoint {
        if let Some(current) = self.lookbehind.front() {
            BufferedLexerCheckpoint {
                position: current.token.start(),
                after_new_line: current.token.has_preceding_line_break(),
            }
        } else {
            BufferedLexerCheckpoint {
                position: self.inner.position(),
                after_new_line: self.inner.state.after_newline,
            }
        }
    }

    pub fn rewind(&mut self, checkpoint: BufferedLexerCheckpoint) {
        self.inner.rewind(checkpoint.position);
        self.inner.state.after_newline = checkpoint.after_new_line;
        self.lookbehind.clear();
    }

    pub fn re_lex(&mut self, token: Token, mode: LexMode) -> LexerReturn {
        self.lookbehind.clear();
        let result = self.inner.re_lex(token, mode);

        result
    }

    #[inline(always)]
    pub fn lookahead<'s>(&'s mut self) -> LookaheadIterator<'s, 't> {
        LookaheadIterator::new(self)
    }
}

#[derive(Debug)]
pub struct LookaheadIterator<'l, 't> {
    buffered: &'l mut BufferedLexer<'t>,
    nth: usize,
}

impl<'l, 't> LookaheadIterator<'l, 't> {
    fn new(lexer: &'l mut BufferedLexer<'t>) -> Self {
        Self {
            buffered: lexer,
            nth: 1,
        }
    }
}

impl<'l, 't> Iterator for LookaheadIterator<'l, 't> {
    type Item = Token;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.nth == 0 {
            return None;
        }

        if let Some(lexer_return) = self.buffered.lookbehind.get((self.nth - 1) as usize) {
            self.nth += 1;
            return Some(*lexer_return.token());
        }

        let lex_return = self.buffered.inner.next();
        let token = lex_return.token;
        self.buffered.lookbehind.push_back(lex_return);

        if token.kind() == EOF {
            // Marks that the iterator is at an end
            self.nth = 0;
        } else {
            self.nth += 1;
        }

        Some(token)
    }
}

impl<'l, 't> FusedIterator for LookaheadIterator<'l, 't> {}

impl<'l, 't> IntoIterator for &'l mut BufferedLexer<'t> {
    type Item = LexerReturn;
    type IntoIter = BufferedLexerIterator<'l, 't>;

    fn into_iter(self) -> Self::IntoIter {
        BufferedLexerIterator {
            inner: self,
            returned_eof: false,
        }
    }
}

pub struct BufferedLexerIterator<'l, 't> {
    inner: &'l mut BufferedLexer<'t>,
    returned_eof: bool,
}

impl Iterator for BufferedLexerIterator<'_, '_> {
    type Item = LexerReturn;

    fn next(&mut self) -> Option<Self::Item> {
        if self.returned_eof {
            return None;
        }

        let result = self.inner.next();

        if result.kind() == EOF {
            self.returned_eof = true;
        }

        Some(result)
    }
}

impl FusedIterator for BufferedLexerIterator<'_, '_> {}

#[derive(Debug)]
pub struct BufferedLexerCheckpoint {
    position: TextSize,
    after_new_line: bool, // TODO remove
                          // state: LexerState
}

#[cfg(test)]
mod tests {
    use crate::buffered_lexer::BufferedLexer;
    use crate::{Lexer, TextRange, TextSize};
    use rome_js_syntax::JsSyntaxKind::{JS_NUMBER_LITERAL, NEWLINE, WHITESPACE};
    use rome_js_syntax::T;

    #[test]
    fn without_lookahead() {
        let lexer = Lexer::from_str("let a\n = 5", 0);
        let mut buffered = BufferedLexer::new(lexer);

        let current = *buffered.next().token();
        assert_eq!(current.kind(), T![let]);
        assert!(!current.has_preceding_line_break());
        assert_eq!(
            current.range(),
            TextRange::at(TextSize::from(0), TextSize::from(3))
        );

        assert_eq!(buffered.next().kind(), WHITESPACE);
        assert_eq!(buffered.next().kind(), T![ident]);
        assert_eq!(buffered.next().kind(), NEWLINE);
        assert_eq!(buffered.next().kind(), WHITESPACE);

        let current = *buffered.next().token();
        assert_eq!(current.kind(), T![=]);
        assert!(current.has_preceding_line_break());

        assert_eq!(buffered.next().kind(), WHITESPACE);
        assert_eq!(buffered.next().kind(), JS_NUMBER_LITERAL);
        assert_eq!(buffered.next().kind(), T![EOF]);
    }

    #[test]
    fn lookahead() {
        let lexer = Lexer::from_str("let a\n = 5", 0);
        let mut buffered = BufferedLexer::new(lexer);

        let current = *buffered.next().token();
        assert_eq!(current.kind(), T![let]);
        assert!(!current.has_preceding_line_break());
        assert_eq!(
            current.range(),
            TextRange::at(TextSize::from(0), TextSize::from(3))
        );

        {
            let mut lookahead = buffered.lookahead();

            let nth1 = lookahead.next().unwrap();
            let nth2 = lookahead.next().unwrap();
            let nth3 = lookahead.next().unwrap();

            assert_eq!(nth1.kind(), WHITESPACE);
            assert_eq!(nth2.kind(), T![ident]);
            assert_eq!(nth3.kind(), NEWLINE);
        }

        assert_eq!(
            buffered
                .lookbehind
                .iter()
                .map(|entry| entry.kind())
                .collect::<Vec<_>>(),
            vec![WHITESPACE, T![ident], NEWLINE]
        );

        assert_eq!(buffered.next().kind(), WHITESPACE);

        {
            let mut lookahead = buffered.lookahead();
            let nth1 = lookahead.next().unwrap();
            let nth2 = lookahead.next().unwrap();
            let nth3 = lookahead.next().unwrap();
            let nth4 = lookahead.next().unwrap();

            assert_eq!(nth1.range().start(), TextSize::from(4));
            assert_eq!(nth1.kind(), T![ident]);
            assert_eq!(nth2.range().start(), TextSize::from(5));
            assert_eq!(nth2.kind(), NEWLINE);
            assert_eq!(nth3.kind(), WHITESPACE);
            assert_eq!(nth4.kind(), T![=]);
            assert!(nth4.has_preceding_line_break());
        }

        assert_eq!(buffered.next().kind(), T![ident]);
        assert_eq!(buffered.next().kind(), NEWLINE);
        assert_eq!(buffered.next().kind(), WHITESPACE);

        let current = *buffered.next().token();
        assert_eq!(current.kind(), T![=]);
        assert!(current.has_preceding_line_break());
        assert_eq!(buffered.next().kind(), WHITESPACE);
        assert_eq!(buffered.next().kind(), JS_NUMBER_LITERAL);
        assert_eq!(buffered.next().kind(), T![EOF]);
    }
}
