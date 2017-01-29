use data_structures::LazyCell;
use {NodeType, Range};

#[derive(Debug)]
pub struct RawNodes {
    data: Vec<RawNode>
}

impl RawNodes {
    pub fn new() -> RawNodes {
        RawNodes { data: Vec::new() }
    }

    pub fn push(&mut self, node: RawNode) -> NodeId {
        let result = NodeId(self.data.len() as u32);
        self.data.push(node);
        result
    }
}

impl ::std::ops::Index<NodeId> for RawNodes {
    type Output = RawNode;
    fn index(&self, index: NodeId) -> &RawNode {
        &self.data[index.0 as usize]
    }
}

impl ::std::ops::IndexMut<NodeId> for RawNodes {
    fn index_mut(&mut self, index: NodeId) -> &mut RawNode {
        &mut self.data[index.0 as usize]
    }
}

#[derive(Clone, Copy, Debug)]
pub struct NodeId(u32);

pub const ZERO_NODE_ID: NodeId = NodeId(0);

#[derive(Debug)]
pub struct RawNode {
    pub ty: NodeType,
    pub parent: Option<NodeId>,
    pub next_sibling: Option<NodeId>,
    pub data: RawNodeData,
}

#[derive(Debug)]
pub enum RawNodeData {
    Leaf {
        range: Range
    },
    Composite {
        first_child: Option<NodeId>,
        range: LazyCell<Range>,
    }
}

impl RawNode {
    pub fn first_child(&self) -> Option<NodeId> {
        match self.data {
            RawNodeData::Leaf { .. } => None,
            RawNodeData::Composite { first_child, .. } => first_child,
        }
    }

    pub fn set_first_child(&mut self, id: NodeId) {
        match self.data {
            RawNodeData::Leaf { .. } => panic!("Leaf node can't have children"),
            RawNodeData::Composite { ref mut first_child, .. } => *first_child = Some(id),
        }
    }
}
