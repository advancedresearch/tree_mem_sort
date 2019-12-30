/*
Duplicate of DAG based on IPLD_DAG Construction
*/

extern crate tree_mem_sort;

use tree_mem_sort::sort_dag;

use std::collections::BTreeMap;
use std::fmt;

use serde::de;
use serde::ser;
use serde::{Deserialize, Serialize};
use serde_bytes;
use serde_cbor::tags::{current_cbor_tag, Tagged};

const CBOR_TAG_CID: u64 = 42;

#[derive(Debug, PartialEq)]
struct Cid(Vec<u8>);

impl ser::Serialize for Cid {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let value = serde_bytes::Bytes::new(&self.0);
        Tagged::new(Some(CBOR_TAG_CID), &value).serialize(s)
    }
}

impl<'de> de::Deserialize<'de> for Cid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let tagged = Tagged::<serde_bytes::ByteBuf>::deserialize(deserializer)?;
        match tagged.tag {
            Some(CBOR_TAG_CID) | None => Ok(Cid(tagged.value.to_vec())),
            Some(_) => Err(de::Error::custom("unexpected tag")),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Ipld {
    Null,
    Bool(bool),
    Integer(i128),
    Float(f64),
    String(String),
    Bytes(Vec<u8>),
    List(Vec<Ipld>),
    Map(BTreeMap<String, Ipld>),
    Link(Vec<u8>),
}

#[derive(PartialEq, Debug)]
struct Node {
    val: u32,
    parents: Cid,
    children: Cid,
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
