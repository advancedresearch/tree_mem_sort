# Tree-Memory-Sort

An in-memory topological sort algorithm for trees based on Group Theory

### Design

This algorithm uses in-memory swapping directly on the array which nodes are stored.
Since swap operations satisfy the axioms of Group Theory,
the topological sort can be made more efficient by using a group generator.

A group generator is an array which stores an index for every node index.
When swap operations are performed on the array instead of the data,
it is possible to predict where nodes will be stored in the solution
without changing the meaning of the current node indices.

Once a solution has been found, the group generator can be used to
retrace the swapping operations required to order the tree.

The order which swaps are retraced might be different than the solving phase:

```text
`a, b, c` => `a, (c, b)` => `(c, a), b` => `c, a, b` (solving phase)
`c, a, b` => `(b), a, (c)` => `(a, b), c` => `a, b, c` (retrace phase)
```


### Primes example

This example shows how the algorithm works using some simple numbers.

Assume that you have two equations:

```text
   12 = 2 * 6
   6 = 3 * 2
```

If you arrange these equations as a tree,
you will naturally start at the top `12` and list
children of each node in the same order as in the equations.

When using an automated theorem prover that re-writes
a tree like this, it can get messy.
Some algorithms relies on a well-ordered tree to perform efficiently.

By performing topological sort on the tree,
it can be restored to a well-ordered form:

```text
    Tree            i       i'
    --------------------------
    12              3   =>  0
    |- 2            2   =>  1
    |- 6            1   =>  2
       |- 3         4   =>  3
       |- 2         0   =>  4
```

The algorithm does not change the relative connections inside the tree,
just how nodes are stored in memory.
However, it needs to change the indices such they point to the correct nodes.

Source code:

```rust
extern crate tree_mem_sort;

use tree_mem_sort::sort;

#[derive(Debug)]
pub struct Number {
    /// The value of the number.
    pub value: u32,
    /// Which number this was factored from.
    pub parent: Option<usize>,
    /// Prime factors.
    pub children: Vec<usize>,
}

fn main() {
    let mut nodes = vec![
        Number {value: 2, parent: Some(1), children: vec![]},       // 0
        Number {value: 6, parent: Some(0), children: vec![4, 0]},   // 1
        Number {value: 2, parent: Some(2), children: vec![]},       // 2
        Number {value: 12, parent: None, children: vec![2, 1]},     // 3
        Number {value: 3, parent: Some(2), children: vec![]},       // 4
    ];
    for i in 0..nodes.len() {
        println!("{}: {:?}", i, nodes[i]);
    }
    // Prints `[2, 6, 2, 12, 3]`
    println!("{:?}", nodes.iter().map(|n| n.value).collect::<Vec<u32>>());

    sort(&mut nodes, |n| &mut n.parent, |n| &mut n.children);
    println!("");
    for i in 0..nodes.len() {
        println!("{}: {:?}", i, nodes[i]);
    }
    // Prints `[12, 2, 6, 3, 2]`
    println!("{:?}", nodes.iter().map(|n| n.value).collect::<Vec<u32>>());
}
```

### Limitations

The algorithm assumes that each node is referenced by maximum one parent.
If you share nodes between parent nodes, the algorithm might enter an infinite loop.
