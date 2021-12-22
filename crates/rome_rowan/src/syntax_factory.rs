mod parsed_children;
mod raw_syntax;

use crate::SyntaxKind;
use std::fmt;
use std::iter::FusedIterator;

pub use self::parsed_children::{
	ParsedChildren, ParsedChildrenIntoIterator, ParsedChildrenIterator,
};
pub use self::raw_syntax::{
	RawSyntaxElement, RawSyntaxElementRef, RawSyntaxNode, RawSyntaxNodeRef, RawSyntaxToken,
	RawSyntaxTokenRef,
};

/// Factory for creating syntax nodes of a particular kind.
pub trait SyntaxFactory: fmt::Debug {
	/// The syntax kind used by the nodes constructed by this syntax factory.
	type Kind: SyntaxKind;

	/// Creates a new syntax node of the passed `kind` with the given children.
	///
	/// The `children` contains the parsed direct children of the node. There may be fewer children
	/// in case there's a syntax error and a required child or an optional child isn't present in the source code.
	/// The `make_syntax` implementation must then fill in empty slots to match the slots as they're defined in the grammar.
	///
	/// The implementation is free to change the `kind` of the node but that has the consequence that
	/// such a node will not be cached. The reason for not caching these nodes is that the cache lookup is performed
	/// before calling `make_syntax`, thus querying the cache with the old kind.
	///
	/// It's important that the factory function is idempotent, meaning, calling the function
	/// multiple times with the same `kind` and `children` returns syntax nodes with the same structure.
	/// This is important because the returned nodes may be cached by `kind` and what `children` are present.
	fn make_syntax(
		kind: Self::Kind,
		children: ParsedChildren<Self::Kind>,
	) -> RawSyntaxNode<Self::Kind>;

	/// Crates a *node list* syntax node. Validates if all elements are valid and changes the node's kind to
	/// [SyntaxKind::to_unknown] if that's not the case.
	fn make_node_list_syntax<F>(
		kind: Self::Kind,
		children: ParsedChildren<Self::Kind>,
		can_cast: F,
	) -> RawSyntaxNode<Self::Kind>
	where
		F: Fn(Self::Kind) -> bool,
	{
		let valid = (&children)
			.into_iter()
			.all(|element| can_cast(element.kind()));

		let kind = if valid { kind } else { kind.to_unknown() };

		RawSyntaxNode::new(kind, children.into_iter().map(Some))
	}

	/// Creates a *separated list* syntax node. Validates if the elements are valid, are correctly
	/// separated by the specified separator token.
	///
	/// It changes the kind of the node to [SyntaxKind::to_unknown] if an element isn't a valid list-node
	/// nor separator.
	///
	/// It inserts empty slots for missing elements or missing markers
	fn make_separated_list_syntax<F>(
		kind: Self::Kind,
		children: ParsedChildren<Self::Kind>,
		can_cast: F,
		separator: Self::Kind,
		allow_trailing: bool,
	) -> RawSyntaxNode<Self::Kind>
	where
		F: Fn(Self::Kind) -> bool,
	{
		let mut next_node = true;

		for child in &children {
			let kind = child.kind();

			if next_node && can_cast(kind) {
				next_node = false;
			} else if !next_node && kind == separator {
				next_node = true;
			} else {
				return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
			}
		}

		if allow_trailing || !next_node || children.is_empty() {
			RawSyntaxNode::new(kind, children.into_iter().map(Some))
		} else {
			// Unallowed trailing comma
			RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some))
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SlotContent {
	Present,
	Absent,
}

/// Description of the slots of a node in combination with [ParsedChildren].
/// It stores for each slot if the node is present in [ParsedChildren] or not, allowing
/// to generate a node with the right number of empty slots.
#[derive(Debug)]
pub struct RawNodeSlots<const COUNT: usize> {
	slots: [SlotContent; COUNT],
	current_slot: usize,
}

impl<const COUNT: usize> Default for RawNodeSlots<COUNT> {
	fn default() -> Self {
		Self {
			slots: [SlotContent::Absent; COUNT],
			current_slot: 0,
		}
	}
}

impl<const COUNT: usize> RawNodeSlots<COUNT> {
	/// Marks that the node for the current slot is *absent* in the source, meaning, that the slot is empty.
	/// Progresses to the next slot
	pub fn mark_absent(&mut self) {
		debug_assert!(self.current_slot < COUNT);

		self.slots[self.current_slot] = SlotContent::Absent;
		self.current_slot += 1;
	}

	/// Marks that the node for the current slot is *present* in the source code and progresses to the next slot.
	pub fn mark_present(&mut self) {
		debug_assert!(self.current_slot < COUNT);

		self.slots[self.current_slot] = SlotContent::Present;
		self.current_slot += 1;
	}

	/// Creates a node with the kind `kind`, filling in the nodes from the `children`.
	pub fn into_node<K: SyntaxKind>(
		self,
		kind: K,
		children: ParsedChildren<K>,
	) -> RawSyntaxNode<K> {
		debug_assert!(self.current_slot == COUNT, "Missing slots");

		RawSyntaxNode::new(
			kind,
			RawNodeSlotIterator {
				children: children.into_iter(),
				slots: self.slots.as_slice().iter(),
			},
		)
	}
}

struct RawNodeSlotIterator<'a, K: SyntaxKind> {
	children: ParsedChildrenIntoIterator<'a, K>,
	slots: std::slice::Iter<'a, SlotContent>,
}

impl<'a, K: SyntaxKind> Iterator for RawNodeSlotIterator<'a, K> {
	type Item = Option<RawSyntaxElement<K>>;

	fn next(&mut self) -> Option<Self::Item> {
		let slot = self.slots.next()?;

		match slot {
			SlotContent::Present => {
				Some(Some(self.children.next().expect(
					"Expected a present node according to the slot description",
				)))
			}
			SlotContent::Absent => Some(None),
		}
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		(self.slots.len(), Some(self.slots.len()))
	}
}

impl<'a, K: SyntaxKind> FusedIterator for RawNodeSlotIterator<'a, K> {}
impl<'a, K: SyntaxKind> ExactSizeIterator for RawNodeSlotIterator<'a, K> {}
