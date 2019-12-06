use common::aoc::{load_input, run_many, print_time, print_result};

fn main() {
    let input = load_input("day06");

    let (tree, dur_parse) = run_many(100, || OrbitTree::parse(&input));
    let (res_part1, dur_part1) = run_many(1000, || tree.checksum(tree.com_index, 0));
    let (res_part2, dur_part2) = run_many(1000, || tree.num_transfers("YOU", "SAN"));

    print_result("P1", res_part1);
    print_result("P2", res_part2);

    print_time("Parse", dur_parse);
    print_time("P1", dur_part1);
    print_time("P2", dur_part2);
}

struct OrbitTree {
    nodes: Vec<OrbitNode>,
    com_index: usize,
}

impl OrbitTree {
    fn node(&self, name: &str) -> Option<usize> {
        for (index, node) in self.nodes.iter().enumerate() {
            if &node.name == name {
                return Some(index);
            }
        }

        None
    }

    fn ensure_node(&mut self, name: &str) -> usize {
        for (index, node) in self.nodes.iter().enumerate() {
            if &node.name == name {
                return index
            }
        }

        self.nodes.push(OrbitNode{
            name: name.to_string(),
            children: Vec::with_capacity(32),
            parent: self.nodes.len(),
        });

        self.nodes.len() - 1
    }

    fn checksum(&self, index: usize, level: u32) -> u32 {
        let mut total = level;

        for child_index in self.nodes[index].children.iter() {
            total += self.checksum(*child_index, level + 1);
        }

        total
    }

    fn num_transfers(&self, from: &str, to: &str) -> u32 {
        let from_index = self.node(from).unwrap();
        let to_index = self.node(to).unwrap();

        let mut from_parents: Vec<usize> = Vec::with_capacity(64);

        let mut current_index = from_index;
        while current_index != self.com_index {
            current_index = self.nodes[current_index].parent;

            from_parents.push(current_index);

            if current_index == to_index {
                return from_parents.len() as u32;
            }
        }

        let mut distance = 0;
        current_index = to_index;
        while current_index != self.com_index {
            current_index = self.nodes[current_index].parent;

            for (index, from_parent) in from_parents.iter().enumerate() {
                if *from_parent == current_index {
                    return distance + index as u32;
                }
            }

            distance += 1;
        }

        panic!("Not found");
    }

    fn new() -> OrbitTree {
        OrbitTree{
            nodes: Vec::with_capacity(256),
            com_index: 0,
        }
    }

    fn parse(str: &str) -> OrbitTree {
        let mut tree = Self::new();

        for line in str.lines() {
            if line.len() < 2 {
                continue;
            }

            let mut tokens = line.split(')');
            let left: &str = tokens.next().unwrap();
            let right: &str = tokens.next().unwrap();

            let left_index = tree.ensure_node(left);
            let right_index = tree.ensure_node(right);

            if left == "COM" {
                tree.com_index = left_index;
            }

            tree.nodes[left_index].children.push(right_index);
            tree.nodes[right_index].parent = left_index;
        }

        tree
    }
}

struct OrbitNode {
    name: String,
    parent: usize,
    children: Vec<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_checksum() {
        let tree_str = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\n";
        let tree = OrbitTree::parse(tree_str);

        assert_eq!(tree.checksum(tree.com_index, 0), 42);
    }

    #[test]
    fn test_tree_transfers() {
        let tree_str = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN\n";
        let tree = OrbitTree::parse(tree_str);

        assert_eq!(tree.num_transfers("YOU", "SAN"), 4);
    }
}