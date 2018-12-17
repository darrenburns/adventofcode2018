use std::fs::read_to_string;

const TREE_FILE: &'static str = "src/files/eight/tree.txt";


#[derive(Default, Debug)]
struct Node {
    metadata: Vec<u32>,
    children: Vec<Node>,
}

fn sum_metadata(node: &Node) -> u32 {
    node.metadata.iter().sum::<u32>() +
        node.children.iter()
            .map(|child| sum_metadata(child))
            .sum::<u32>()
}

fn sum_metadata_part_two(node: &Node) -> u32 {
    if node.children.is_empty() {
        return node.metadata.iter().sum::<u32>();
    }

    node.metadata.iter()
        .fold(0, |sum, node|
            sum + node.children.get(meta as usize - 1)
                .map(|node| sum_metadata_part_two(node))
                .unwrap_or_default(),
        )
}

pub fn calculate_sum() -> u32 {
    let tree = load_tree_data();
    let node = Node::get_node(&mut tree.iter().cloned()).expect("No input data.");

    sum_metadata(&node)
}

pub fn calculate_value_of_root() -> u32 {
    let tree = load_tree_data();
    let node = Node::get_node(&mut tree.iter().cloned()).expect("No input data.");

    sum_metadata_part_two(&node)
}

impl Node {
    fn get_node(iter: &mut impl Iterator<Item=u32>) -> Option<Node> {
        let num_children = match iter.next() {
            Some(num) => num,
            None => return None,
        };

        let num_metadata = iter.next().expect("Invalid input");
        let mut node = Node::default();

        for _ in 0..num_children {
            node.children.extend(Self::get_node(iter));
        }

        for _ in 0..num_metadata {
            node.metadata.push(iter.next().expect("Invalid metadata list."));
        }

        Some(node)
    }
}

fn load_tree_data() -> Vec<u32> {
    read_to_string(TREE_FILE).unwrap()
        .split_whitespace()
        .map(|elem| elem.parse::<u32>().unwrap())
        .collect()
}
