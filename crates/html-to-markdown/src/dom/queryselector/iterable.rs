use crate::dom::{InnerNodeHandle, Node, NodeHandle, Parser, VDom};

mod private {
    pub trait Sealed {}
}

/// Trait for types that a query selector can iterate over
pub trait QueryIterable<'a>: private::Sealed {
    /// Gets a node at a specific index
    fn get<'b>(&'b self, parser: &'b Parser<'a>, index: usize) -> Option<(&'b Node<'a>, NodeHandle)>;
    /// Gets or computes the length (number of nodes)
    fn len(&self, parser: &Parser) -> usize;
    /// Gets the starting index
    fn start(&self) -> Option<InnerNodeHandle>;
}

impl<'a> private::Sealed for VDom<'a> {}
impl<'a> QueryIterable<'a> for VDom<'a> {
    #[inline]
    fn get<'b>(&'b self, parser: &'b Parser<'a>, index: usize) -> Option<(&'b Node<'a>, NodeHandle)> {
        // In a VDom, the index is equal to the node's id
        // and as such, we can recreate a `NodeHandle` from that ID
        parser.tags.get(index).map(|node| (node, NodeHandle::new(index as u32)))
    }

    #[inline]
    fn len(&self, _parser: &Parser) -> usize {
        self.parser().tags.len()
    }

    #[inline]
    fn start(&self) -> Option<InnerNodeHandle> {
        // The starting ID is always 0 in a VDom
        Some(0)
    }
}
