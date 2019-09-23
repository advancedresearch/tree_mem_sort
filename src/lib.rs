//! # Tree-Memory-Sort
//!
//! An in-memory topological sort algorithm for trees based on Group Theory
//!
//! ### Design
//!
//! This algorithm uses in-memory swapping directly on the array which nodes are stored.
//! Since swap operations satisfy the axioms of Group Theory,
//! the topological sort can be made more efficient by using a group generator.
//!
//! A group generator is an array which stores an index for every node index.
//! When swap operations are performed on the array instead of the data,
//! it is possible to predict where nodes will be stored in the solution
//! without changing the meaning of the current node indices.
//!
//! Once a solution has been found, the group generator can be used to
//! retrace the swapping operations required to order the tree.
//!
//! The order which swaps are retraced might be different than the solving phase:
//!
//! ```text
//! `a, b, c` => `a, (c, b)` => `(c, a), b` => `c, a, b` (solving phase)
//! `c, a, b` => `(b), a, (c)` => `(a, b), c` => `a, b, c` (retrace phase)
//! ```
//!
//!
//! ### Primes example
//!
//! This example shows how the algorithm works using some simple numbers.
//!
//! Assume that you have two equations:
//!
//! ```text
//!    12 = 2 * 6
//!    6 = 3 * 2
//! ```
//!
//! If you arrange these equations as a tree,
//! you will naturally start at the top `12` and list
//! children of each node in the same order as in the equations.
//!
//! When using an automated theorem prover that re-writes
//! a tree like this, it can get messy.
//! Some algorithms relies on a well-ordered tree to perform efficiently.
//!
//! By performing topological sort on the tree,
//! it can be restored to a well-ordered form:
//!
//! ```text
//!     Tree            i       i'
//!     --------------------------
//!     12              3   =>  0
//!     |- 2            2   =>  1
//!     |- 6            1   =>  2
//!        |- 3         4   =>  3
//!        |- 2         0   =>  4
//! ```
//!
//! The algorithm does not change the relative connections inside the tree,
//! just how nodes are stored in memory.
//! However, it needs to change the indices such they point to the correct nodes.
//!
//! Source code:
//!
//! ```rust
//! extern crate tree_mem_sort;
//!
//! use tree_mem_sort::sort;
//!
//! #[derive(Debug)]
//! pub struct Number {
//!     /// The value of the number.
//!     pub value: u32,
//!     /// Which number this was factored from.
//!     pub parent: Option<usize>,
//!     /// Prime factors.
//!     pub children: Vec<usize>,
//! }
//!
//! fn main() {
//!     let mut nodes = vec![
//!         Number {value: 2, parent: Some(1), children: vec![]},       // 0
//!         Number {value: 6, parent: Some(0), children: vec![4, 0]},   // 1
//!         Number {value: 2, parent: Some(2), children: vec![]},       // 2
//!         Number {value: 12, parent: None, children: vec![2, 1]},     // 3
//!         Number {value: 3, parent: Some(2), children: vec![]},       // 4
//!     ];
//!     for i in 0..nodes.len() {
//!         println!("{}: {:?}", i, nodes[i]);
//!     }
//!     // Prints `[2, 6, 2, 12, 3]`
//!     println!("{:?}", nodes.iter().map(|n| n.value).collect::<Vec<u32>>());
//!
//!     sort(&mut nodes, |n| &mut n.parent, |n| &mut n.children);
//!     println!("");
//!     for i in 0..nodes.len() {
//!         println!("{}: {:?}", i, nodes[i]);
//!     }
//!     // Prints `[12, 2, 6, 3, 2]`
//!     println!("{:?}", nodes.iter().map(|n| n.value).collect::<Vec<u32>>());
//! }
//! ```
//!
//! ### Limitations
//!
//! The algorithm assumes that each node is referenced by maximum one parent.
//! If you share nodes between parent nodes, the algorithm might enter an infinite loop.

#![deny(missing_docs)]

/// Performs in-memory topological sort on a tree where
/// order is determined by every child being greater than their parent,
/// and every sibling being greater than previous siblings.
pub fn sort<T, P, C>(nodes: &mut [T], parent: P, children: C)
where
    P: Fn(&mut T) -> &mut Option<usize>,
    C: Fn(&mut T) -> &mut [usize],
{
    // This problem can be solving efficiently using Group Theory.
    // This avoid the need for cloning nodes into a new array,
    // while performing the minimum work to get a normalized tree.
    //
    // Create a group generator that is modified by swapping to find a solution.
    // The group generator keeps track of indices, such that child-parent relations
    // do not have to change until later.
    //
    // Use the order in the generator to detect whether a swap has been performed.
    // The condition for swapping `a, b` is `gen[a] > gen[b]`.
    let mut gen: Vec<usize> = (0..nodes.len()).collect();
    loop {
        let mut changed = false;
        for i in 0..nodes.len() {
            let children = children(&mut nodes[i]);
            for j in 0..children.len() {
                let a = children[j];
                // Store child after its parent.
                if gen[i] > gen[a] {
                    gen.swap(i, a);
                    changed = true;
                }
                // Check all pairs of children.
                for k in j + 1..children.len() {
                    let b = children[k];

                    // Store children in sorted order.
                    if gen[a] > gen[b] {
                        gen.swap(a, b);
                        changed = true;
                    }
                }
            }
        }
        if !changed {
            break;
        }
    }

    // Update the tree data with the new indices from the generator.
    // Do this before performing the actual swapping,
    // since the generator maps from old indices to new indices.
    for i in 0..nodes.len() {
        let p = parent(&mut nodes[i]);
        *p = p.map(|p| gen[p]);
        for ch in children(&mut nodes[i]) {
            *ch = gen[*ch]
        }
    }

    // Swap nodes using the group generator as guide.
    // When swapping has been performed, update the generator to keep track of state.
    // This is because multiple swaps sharing elements might require multiple steps.
    //
    // The order which swaps are retraced might be different than the solving phase:
    //
    // `a, b, c` => `a, (c, b)` => `(c, a), b` => `c, a, b` (solving phase)
    // `c, a, b` => `(b), a, (c)` => `(a, b), c` => `a, b, c` (retrace phase)
    //
    // However, since the generator solution is produced by swapping operations,
    // it is guaranteed to be restorable to the identity generator when retracing.
    //
    // There is no need to loop more than once because each index is stored uniquely by lookup,
    // such that if `g[i] = i` then there exists no `j != i` such that `g[j] = i`.
    for i in 0..nodes.len() {
        while gen[i] != i {
            let j = gen[i];
            nodes.swap(i, j);
            gen.swap(i, j);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(PartialEq, Debug)]
    struct Node {
        val: u32,
        parent: Option<usize>,
        children: Vec<usize>,
    }

    #[test]
    fn empty() {
        let mut nodes: Vec<Node> = vec![];
        sort(&mut nodes, |n| &mut n.parent, |n| &mut n.children);
        assert_eq!(nodes.len(), 0);
    }

    #[test]
    fn one() {
        let mut nodes: Vec<Node> = vec![Node {
            val: 0,
            parent: None,
            children: vec![],
        }];
        sort(&mut nodes, |n| &mut n.parent, |n| &mut n.children);
        assert_eq!(
            nodes,
            vec![Node {
                val: 0,
                parent: None,
                children: vec![]
            }]
        );
    }

    #[test]
    fn two() {
        let mut nodes: Vec<Node> = vec![
            Node {
                val: 0,
                parent: None,
                children: vec![],
            },
            Node {
                val: 1,
                parent: None,
                children: vec![],
            },
        ];
        sort(&mut nodes, |n| &mut n.parent, |n| &mut n.children);
        assert_eq!(
            nodes,
            vec![
                Node {
                    val: 0,
                    parent: None,
                    children: vec![]
                },
                Node {
                    val: 1,
                    parent: None,
                    children: vec![]
                },
            ]
        );

        let mut nodes: Vec<Node> = vec![
            Node {
                val: 1,
                parent: Some(1),
                children: vec![],
            },
            Node {
                val: 0,
                parent: None,
                children: vec![0],
            },
        ];
        sort(&mut nodes, |n| &mut n.parent, |n| &mut n.children);
        assert_eq!(
            nodes,
            vec![
                Node {
                    val: 0,
                    parent: None,
                    children: vec![1]
                },
                Node {
                    val: 1,
                    parent: Some(0),
                    children: vec![]
                },
            ]
        );
    }

    #[test]
    fn three() {
        let mut nodes: Vec<Node> = vec![
            Node {
                val: 2,
                parent: Some(1),
                children: vec![],
            },
            Node {
                val: 1,
                parent: Some(2),
                children: vec![0],
            },
            Node {
                val: 0,
                parent: None,
                children: vec![1],
            },
        ];
        sort(&mut nodes, |n| &mut n.parent, |n| &mut n.children);
        assert_eq!(
            nodes,
            vec![
                Node {
                    val: 0,
                    parent: None,
                    children: vec![1]
                },
                Node {
                    val: 1,
                    parent: Some(0),
                    children: vec![2]
                },
                Node {
                    val: 2,
                    parent: Some(1),
                    children: vec![]
                }
            ]
        );

        let mut nodes: Vec<Node> = vec![
            Node {
                val: 1,
                parent: Some(2),
                children: vec![1],
            },
            Node {
                val: 2,
                parent: Some(0),
                children: vec![],
            },
            Node {
                val: 0,
                parent: None,
                children: vec![0],
            },
        ];
        sort(&mut nodes, |n| &mut n.parent, |n| &mut n.children);
        assert_eq!(
            nodes,
            vec![
                Node {
                    val: 0,
                    parent: None,
                    children: vec![1]
                },
                Node {
                    val: 1,
                    parent: Some(0),
                    children: vec![2]
                },
                Node {
                    val: 2,
                    parent: Some(1),
                    children: vec![]
                }
            ]
        );

        let mut nodes: Vec<Node> = vec![
            Node {
                val: 2,
                parent: Some(2),
                children: vec![],
            },
            Node {
                val: 1,
                parent: Some(2),
                children: vec![],
            },
            Node {
                val: 0,
                parent: None,
                children: vec![1, 0],
            },
        ];
        sort(&mut nodes, |n| &mut n.parent, |n| &mut n.children);
        assert_eq!(
            nodes,
            vec![
                Node {
                    val: 0,
                    parent: None,
                    children: vec![1, 2]
                },
                Node {
                    val: 1,
                    parent: Some(0),
                    children: vec![]
                },
                Node {
                    val: 2,
                    parent: Some(0),
                    children: vec![]
                }
            ]
        );

        let mut nodes: Vec<Node> = vec![
            Node {
                val: 1,
                parent: Some(2),
                children: vec![],
            },
            Node {
                val: 2,
                parent: Some(2),
                children: vec![],
            },
            Node {
                val: 0,
                parent: None,
                children: vec![0, 1],
            },
        ];
        sort(&mut nodes, |n| &mut n.parent, |n| &mut n.children);
        assert_eq!(
            nodes,
            vec![
                Node {
                    val: 0,
                    parent: None,
                    children: vec![1, 2]
                },
                Node {
                    val: 1,
                    parent: Some(0),
                    children: vec![]
                },
                Node {
                    val: 2,
                    parent: Some(0),
                    children: vec![]
                }
            ]
        );

        let mut nodes: Vec<Node> = vec![
            Node {
                val: 1,
                parent: Some(1),
                children: vec![],
            },
            Node {
                val: 0,
                parent: None,
                children: vec![0, 2],
            },
            Node {
                val: 2,
                parent: Some(1),
                children: vec![],
            },
        ];
        sort(&mut nodes, |n| &mut n.parent, |n| &mut n.children);
        assert_eq!(
            nodes,
            vec![
                Node {
                    val: 0,
                    parent: None,
                    children: vec![1, 2]
                },
                Node {
                    val: 1,
                    parent: Some(0),
                    children: vec![]
                },
                Node {
                    val: 2,
                    parent: Some(0),
                    children: vec![]
                }
            ]
        );

        let mut nodes: Vec<Node> = vec![
            Node {
                val: 2,
                parent: Some(1),
                children: vec![],
            },
            Node {
                val: 0,
                parent: None,
                children: vec![2, 0],
            },
            Node {
                val: 1,
                parent: Some(1),
                children: vec![],
            },
        ];
        sort(&mut nodes, |n| &mut n.parent, |n| &mut n.children);
        assert_eq!(
            nodes,
            vec![
                Node {
                    val: 0,
                    parent: None,
                    children: vec![1, 2]
                },
                Node {
                    val: 1,
                    parent: Some(0),
                    children: vec![]
                },
                Node {
                    val: 2,
                    parent: Some(0),
                    children: vec![]
                }
            ]
        );
    }

    #[test]
    fn four() {
        let mut nodes: Vec<Node> = vec![
            Node {
                val: 3,
                parent: Some(1),
                children: vec![],
            },
            Node {
                val: 2,
                parent: Some(2),
                children: vec![0],
            },
            Node {
                val: 1,
                parent: Some(3),
                children: vec![1],
            },
            Node {
                val: 0,
                parent: None,
                children: vec![2],
            },
        ];
        sort(&mut nodes, |n| &mut n.parent, |n| &mut n.children);
        assert_eq!(
            nodes,
            vec![
                Node {
                    val: 0,
                    parent: None,
                    children: vec![1]
                },
                Node {
                    val: 1,
                    parent: Some(0),
                    children: vec![2]
                },
                Node {
                    val: 2,
                    parent: Some(1),
                    children: vec![3]
                },
                Node {
                    val: 3,
                    parent: Some(2),
                    children: vec![]
                }
            ]
        );

        let mut nodes: Vec<Node> = vec![
            Node {
                val: 2,
                parent: Some(2),
                children: vec![1],
            },
            Node {
                val: 3,
                parent: Some(0),
                children: vec![],
            },
            Node {
                val: 1,
                parent: Some(3),
                children: vec![0],
            },
            Node {
                val: 0,
                parent: None,
                children: vec![2],
            },
        ];
        sort(&mut nodes, |n| &mut n.parent, |n| &mut n.children);
        assert_eq!(
            nodes,
            vec![
                Node {
                    val: 0,
                    parent: None,
                    children: vec![1]
                },
                Node {
                    val: 1,
                    parent: Some(0),
                    children: vec![2]
                },
                Node {
                    val: 2,
                    parent: Some(1),
                    children: vec![3]
                },
                Node {
                    val: 3,
                    parent: Some(2),
                    children: vec![]
                }
            ]
        );

        let mut nodes: Vec<Node> = vec![
            Node {
                val: 2,
                parent: Some(3),
                children: vec![1],
            },
            Node {
                val: 3,
                parent: Some(0),
                children: vec![],
            },
            Node {
                val: 0,
                parent: None,
                children: vec![3],
            },
            Node {
                val: 1,
                parent: Some(2),
                children: vec![0],
            },
        ];
        sort(&mut nodes, |n| &mut n.parent, |n| &mut n.children);
        assert_eq!(
            nodes,
            vec![
                Node {
                    val: 0,
                    parent: None,
                    children: vec![1]
                },
                Node {
                    val: 1,
                    parent: Some(0),
                    children: vec![2]
                },
                Node {
                    val: 2,
                    parent: Some(1),
                    children: vec![3]
                },
                Node {
                    val: 3,
                    parent: Some(2),
                    children: vec![]
                }
            ]
        );

        let mut nodes: Vec<Node> = vec![
            Node {
                val: 2,
                parent: Some(2),
                children: vec![],
            },
            Node {
                val: 3,
                parent: Some(3),
                children: vec![],
            },
            Node {
                val: 1,
                parent: Some(3),
                children: vec![0],
            },
            Node {
                val: 0,
                parent: None,
                children: vec![2, 1],
            },
        ];
        sort(&mut nodes, |n| &mut n.parent, |n| &mut n.children);
        assert_eq!(
            nodes,
            vec![
                Node {
                    val: 0,
                    parent: None,
                    children: vec![1, 3]
                },
                Node {
                    val: 1,
                    parent: Some(0),
                    children: vec![2]
                },
                Node {
                    val: 2,
                    parent: Some(1),
                    children: vec![]
                },
                Node {
                    val: 3,
                    parent: Some(0),
                    children: vec![]
                }
            ]
        );

        let mut nodes: Vec<Node> = vec![
            Node {
                val: 3,
                parent: Some(3),
                children: vec![],
            },
            Node {
                val: 1,
                parent: None,
                children: vec![],
            },
            Node {
                val: 2,
                parent: Some(3),
                children: vec![],
            },
            Node {
                val: 0,
                parent: None,
                children: vec![2, 0],
            },
        ];
        sort(&mut nodes, |n| &mut n.parent, |n| &mut n.children);
        assert_eq!(
            nodes,
            vec![
                Node {
                    val: 0,
                    parent: None,
                    children: vec![2, 3]
                },
                Node {
                    val: 1,
                    parent: None,
                    children: vec![]
                },
                Node {
                    val: 2,
                    parent: Some(0),
                    children: vec![]
                },
                Node {
                    val: 3,
                    parent: Some(0),
                    children: vec![]
                }
            ]
        );

        let mut nodes: Vec<Node> = vec![
            Node {
                val: 3,
                parent: Some(2),
                children: vec![],
            },
            Node {
                val: 2,
                parent: Some(2),
                children: vec![],
            },
            Node {
                val: 1,
                parent: Some(3),
                children: vec![1, 0],
            },
            Node {
                val: 0,
                parent: None,
                children: vec![2],
            },
        ];
        sort(&mut nodes, |n| &mut n.parent, |n| &mut n.children);
        assert_eq!(
            nodes,
            vec![
                Node {
                    val: 0,
                    parent: None,
                    children: vec![1]
                },
                Node {
                    val: 1,
                    parent: Some(0),
                    children: vec![2, 3]
                },
                Node {
                    val: 2,
                    parent: Some(1),
                    children: vec![]
                },
                Node {
                    val: 3,
                    parent: Some(1),
                    children: vec![]
                }
            ]
        );

        let mut nodes: Vec<Node> = vec![
            Node {
                val: 0,
                parent: None,
                children: vec![3, 2],
            },
            Node {
                val: 2,
                parent: Some(3),
                children: vec![],
            },
            Node {
                val: 3,
                parent: Some(0),
                children: vec![],
            },
            Node {
                val: 1,
                parent: Some(0),
                children: vec![1],
            },
        ];
        sort(&mut nodes, |n| &mut n.parent, |n| &mut n.children);
        assert_eq!(
            nodes,
            vec![
                Node {
                    val: 0,
                    parent: None,
                    children: vec![1, 3]
                },
                Node {
                    val: 1,
                    parent: Some(0),
                    children: vec![2]
                },
                Node {
                    val: 2,
                    parent: Some(1),
                    children: vec![]
                },
                Node {
                    val: 3,
                    parent: Some(0),
                    children: vec![]
                },
            ]
        );
    }
}
