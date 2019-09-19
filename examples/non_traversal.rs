/*
This is an example to demonstrate a solution you get
from topological sorting of trees which you do not get
from building indices from traversal.
*/

extern crate tree_mem_sort;

use tree_mem_sort::sort;

#[derive(PartialEq, Debug)]
struct Node {
    val: u32,
    parent: Option<usize>,
    children: Vec<usize>,
}

fn main() {
    // The first example shows that swapping two children of the root
    // can be sufficient to find a topological sort.
    let mut nodes: Vec<Node> = vec![
        Node {val: 0, parent: None, children: vec![2, 1]},
        Node {val: 4, parent: Some(0), children: vec![]},
        Node {val: 1, parent: Some(0), children: vec![3, 4]},
        Node {val: 2, parent: Some(2), children: vec![]},
        Node {val: 3, parent: Some(2), children: vec![]},
    ];
    sort(&mut nodes, |n| &mut n.parent, |n| &mut n.children);
    assert_eq!(nodes, vec![
        Node { val: 0, parent: None, children: vec![1, 2] },
        Node { val: 1, parent: Some(0), children: vec![3, 4] },
        Node { val: 4, parent: Some(0), children: vec![] },
        Node { val: 2, parent: Some(1), children: vec![] },
        Node { val: 3, parent: Some(1), children: vec![] },
    ]);

    // The second example shows what happens when the children
    // of one sub-root is stored earlier, they end up in a different order.
    let mut nodes: Vec<Node> = vec![
        Node {val: 0, parent: None, children: vec![4, 3]},
        Node {val: 2, parent: Some(2), children: vec![]},
        Node {val: 3, parent: Some(2), children: vec![]},
        Node {val: 4, parent: Some(0), children: vec![]},
        Node {val: 1, parent: Some(0), children: vec![1, 2]},
    ];
    sort(&mut nodes, |n| &mut n.parent, |n| &mut n.children);
    assert_eq!(nodes, vec![
        Node { val: 0, parent: None, children: vec![1, 4] },
        Node { val: 1, parent: Some(0), children: vec![2, 3] },
        Node { val: 2, parent: Some(3), children: vec![] },
        Node { val: 3, parent: Some(3), children: vec![] },
        Node { val: 4, parent: Some(0), children: vec![] }
    ]);

    // The topological sort from swapping using a group generator as guide,
    // tends to put children close to their original location.
}
