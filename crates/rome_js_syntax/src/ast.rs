use crate as rome_js_syntax;
use rome_rowan::{Language, NodeOrToken, SyntaxElementChildren};
use std::fmt::{Debug, Formatter};

#[derive(Clone)]
pub struct DebugSyntaxElementChildren<L: Language>(pub SyntaxElementChildren<L>);

impl<L: Language> Debug for DebugSyntaxElementChildren<L> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.clone().0.map(|element| match element {
                NodeOrToken::Node(node) => {
                    crate::map_syntax_node!(node, node => std::fmt::Debug::fmt(node, f))
                }
                NodeOrToken::Token(token) => Debug::fmt(&token, f),
            }))
            .finish()
    }
}
