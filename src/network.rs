use std::collections::HashSet;

#[derive(Debug)]
pub struct Network {
    nodes: HashSet<Node>,
    links: HashSet<Link>,
    turns: HashSet<Turn>,
}

#[derive(Eq, PartialEq, Hash, Debug)]
struct Node(u32);

#[derive(Eq, PartialEq, Hash, Debug)]
struct Link {
    i: Node,
    j: Node,
}

#[derive(Eq, PartialEq, Hash, Debug)]
struct Turn {
    i: Node,
    j: Node,
    k: Node,
}

pub fn new() -> Network {
    return Network {
        nodes: HashSet::new(),
        links: HashSet::new(),
        turns: HashSet::new(),
    };
}

enum Operation {
    AddNode(u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_network_is_empty() {
        let network = new();

        assert!(network.nodes.is_empty());
        assert!(network.links.is_empty());
        assert!(network.turns.is_empty());
    }
}