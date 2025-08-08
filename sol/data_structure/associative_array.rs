use crate::problem;
use alloc::boxed::Box;
use core::{
    cmp::Ordering,
    fmt::{Result, Write},
};
use toy::Reader;

problem!(data_structure, associative_array);

#[derive(Clone, Default)]
struct Node {
    k: u64,
    v: u64,
    w: usize,
    r: Tree,
    l: Tree,
}

impl Node {
    fn new(k: u64, v: u64, w: usize) -> Self {
        Self {
            k,
            v,
            w,
            r: Tree::default(),
            l: Tree::default(),
        }
    }
}

#[derive(Clone, Default)]
struct NodeBox {
    inner: Box<Node>,
}

impl From<Node> for NodeBox {
    fn from(value: Node) -> Self {
        Self {
            inner: Box::new(value),
        }
    }
}

impl NodeBox {
    fn insert(mut self, k: u64, v: u64, w: usize) -> Self {
        match k.cmp(&self.inner.k) {
            Ordering::Less => {
                let mut l = self.inner.l.insert(k, v, w);
                if self.inner.w <= l.inner.w {
                    self.inner.l = l.into();
                    self
                } else {
                    self.inner.l = l.inner.r;
                    l.inner.r = self.into();
                    l
                }
            }
            Ordering::Equal => {
                self.inner.v = v;
                self
            }
            Ordering::Greater => {
                let mut r = self.inner.r.insert(k, v, w);
                if self.inner.w <= r.inner.w {
                    self.inner.r = r.into();
                    self
                } else {
                    self.inner.r = r.inner.l;
                    r.inner.l = self.into();
                    r
                }
            }
        }
    }
    fn get(&self, k: u64) -> Option<u64> {
        match k.cmp(&self.inner.k) {
            Ordering::Less => self.inner.l.get(k),
            Ordering::Equal => Some(self.inner.v),
            Ordering::Greater => self.inner.r.get(k),
        }
    }
}

#[derive(Clone, Default)]
struct Tree {
    root: Option<NodeBox>,
}

impl From<NodeBox> for Tree {
    fn from(value: NodeBox) -> Self {
        Self { root: Some(value) }
    }
}

impl Tree {
    fn insert(self, k: u64, v: u64, w: usize) -> NodeBox {
        self.root
            .map_or_else(|| Node::new(k, v, w).into(), |node| node.insert(k, v, w))
    }
    fn get(&self, k: u64) -> Option<u64> {
        self.root.as_ref().and_then(|node| node.get(k))
    }
}

#[inline]
pub fn associative_array(mut rd: Reader, wt: &mut impl Write) -> Result {
    let q = rd.u26() as usize;
    let mut tree = Tree::default();
    for i in 0..q {
        match rd.digit() {
            0 => {
                let k = rd.u64();
                let v = rd.u64();
                tree = tree.insert(k, v, i.reverse_bits()).into();
            }
            1 => {
                let k = rd.u64();
                let v = tree.get(k).unwrap_or_default();
                writeln!(wt, "{v}")?;
            }
            _ => unreachable!(),
        }
    }
    Ok(())
}
