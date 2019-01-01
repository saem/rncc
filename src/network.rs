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

pub fn create_empty_network() -> Network {
    return Network {
        nodes: HashSet::new(),
        links: HashSet::new(),
        turns: HashSet::new(),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_network_is_empty() {
        let network = create_empty_network();

        assert!(network.nodes.is_empty());
        assert!(network.links.is_empty());
        assert!(network.turns.is_empty());
    }
}