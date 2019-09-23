/*
This is an example to demonstrate the algorithm on Directed Acyclic Graphs (DAGs)
encoded as trees with ordered children and shared nodes.
*/

extern crate tree_mem_sort;

use tree_mem_sort::sort_dag;

#[derive(PartialEq, Debug)]
struct Node {
    val: u32,
    parents: Vec<usize>,
    children: Vec<usize>,
}

fn main() {
    let mut nodes: Vec<Node> = vec![
        Node {
            val: 0,
            parents: vec![],
            children: vec![2, 3],
        },
        // Shared between `1` and `2`.
        Node {
            val: 3,
            parents: vec![2, 3],
            children: vec![]
        },
        Node {
            val: 1,
            parents: vec![0],
            children: vec![1],
        },
        Node {
            val: 2,
            parents: vec![0],
            children: vec![1],
        },
    ];
    sort_dag(&mut nodes, |n| &mut n.parents, |n| &mut n.children);
    assert_eq!(
        nodes,
        vec![
            Node { val: 0, parents: vec![], children: vec![1, 2] },
            Node { val: 1, parents: vec![0], children: vec![3] },
            Node { val: 2, parents: vec![0], children: vec![3] },
            Node { val: 3, parents: vec![1, 2], children: vec![] }
        ]
    );

    let mut nodes: Vec<Node> = vec![
        Node {
            val: 0,
            parents: vec![],
            children: vec![1, 3],
        },
        Node {
            val: 1,
            parents: vec![0],
            children: vec![2],
        },
        Node {
            val: 3,
            parents: vec![1, 3],
            children: vec![]
        },
        Node {
            val: 2,
            parents: vec![0],
            children: vec![2],
        },
    ];
    sort_dag(&mut nodes, |n| &mut n.parents, |n| &mut n.children);
    assert_eq!(
        nodes,
        vec![
            Node { val: 0, parents: vec![], children: vec![1, 2] },
            Node { val: 1, parents: vec![0], children: vec![3] },
            Node { val: 2, parents: vec![0], children: vec![3] },
            Node { val: 3, parents: vec![1, 2], children: vec![] }
        ]
    );

    let mut nodes: Vec<Node> = vec![
        Node {
            val: 0,
            parents: vec![],
            children: vec![3, 1],
        },
        Node {
            val: 2,
            parents: vec![0],
            children: vec![2],
        },
        Node {
            val: 3,
            parents: vec![3, 1],
            children: vec![]
        },
        Node {
            val: 1,
            parents: vec![0],
            children: vec![2],
        },
    ];
    sort_dag(&mut nodes, |n| &mut n.parents, |n| &mut n.children);
    assert_eq!(
        nodes,
        vec![
            Node { val: 0, parents: vec![], children: vec![1, 2] },
            Node { val: 1, parents: vec![0], children: vec![3] },
            Node { val: 2, parents: vec![0], children: vec![3] },
            Node { val: 3, parents: vec![1, 2], children: vec![] }
        ]
    );

    let mut nodes: Vec<Node> = vec![
        Node {
            val: 0,
            parents: vec![],
            children: vec![3, 1],
        },
        Node {
            val: 2,
            parents: vec![0],
            children: vec![2],
        },
        Node {
            val: 3,
            parents: vec![3, 1],
            children: vec![]
        },
        Node {
            val: 1,
            parents: vec![0],
            children: vec![2],
        },
    ];
    sort_dag(&mut nodes, |n| &mut n.parents, |n| &mut n.children);
    assert_eq!(
        nodes,
        vec![
            Node { val: 0, parents: vec![], children: vec![1, 2] },
            Node { val: 1, parents: vec![0], children: vec![3] },
            Node { val: 2, parents: vec![0], children: vec![3] },
            Node { val: 3, parents: vec![1, 2], children: vec![] }
        ]
    );

    let mut nodes: Vec<Node> = vec![
        Node {
            val: 0,
            parents: vec![],
            children: vec![3, 2],
        },
        Node {
            val: 3,
            parents: vec![3, 2],
            children: vec![]
        },
        Node {
            val: 2,
            parents: vec![0],
            children: vec![1],
        },
        Node {
            val: 1,
            parents: vec![0],
            children: vec![1],
        },
    ];
    sort_dag(&mut nodes, |n| &mut n.parents, |n| &mut n.children);
    assert_eq!(
        nodes,
        vec![
            Node { val: 0, parents: vec![], children: vec![1, 2] },
            Node { val: 1, parents: vec![0], children: vec![3] },
            Node { val: 2, parents: vec![0], children: vec![3] },
            Node { val: 3, parents: vec![1, 2], children: vec![] }
        ]
    );

    let mut nodes: Vec<Node> = vec![
        Node {
            val: 0,
            parents: vec![],
            children: vec![3, 2],
        },
        Node {
            val: 3,
            parents: vec![3, 2],
            children: vec![]
        },
        Node {
            val: 2,
            parents: vec![0],
            children: vec![1],
        },
        Node {
            val: 1,
            parents: vec![0],
            // The sibling-as-child must become before shared children.
            children: vec![2, 1],
        },
    ];
    sort_dag(&mut nodes, |n| &mut n.parents, |n| &mut n.children);
    assert_eq!(
        nodes,
        vec![
            Node { val: 0, parents: vec![], children: vec![1, 2] },
            Node { val: 1, parents: vec![0], children: vec![2, 3] },
            Node { val: 2, parents: vec![0], children: vec![3] },
            Node { val: 3, parents: vec![1, 2], children: vec![] }
        ]
    );
}
