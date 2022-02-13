use crate::node::Node;
use std::collections::VecDeque;

pub struct MerkleTree {
    nodes: VecDeque<Node>,
    num_blocks: usize,
    block_size: usize,
    // height?
}
