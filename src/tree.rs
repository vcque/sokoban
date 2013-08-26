use std::str::push_str;
use std::cast::transmute;

/// Simple multi-children tree.
pub struct Tree<N> {
    data: N,
    priv parent: Option<*Tree<N>>,
    childs: ~[Tree<N>]
}

impl <N> Tree<N> {

    pub fn new(dat: N) -> Tree<N> {
        Tree{data: dat, parent: None, childs: ~[]}
    }
    
    pub fn add(&mut self, mut child: Tree<N>) {
        child.set_parent(self);
        self.childs.push(child);
    }

    pub fn remove(&mut self, index: uint) {
        let mut child = self.childs.remove(index);
        child.parent = None;
    }
    
    pub fn get_parent<'a>(&'a self) -> Option<&Tree<N>> {
        unsafe {
            match self.parent {
            Some(p) => { Some(transmute(p)) },
            None    => { None }
            }
        }
    }
    
    fn set_parent(&mut self, parent: &Tree<N>) {
        unsafe {
            self.parent = Some(transmute(parent));
        } 
    }
}

impl <N:ToStr> ToStr for Tree<N> {
    fn to_str(&self) -> ~str {
        let mut result = ~"\n";
        push_str(&mut result, self.data.to_str());
        for self.childs.iter().advance |child| {
            push_str(&mut result, child.to_str());
        }
        result
    }
}

/// Trait for the visitor design pattern.
pub trait Visitor<N> {
    fn visit(&mut self, tree: &mut Tree<N>);
}

/// Work-around for a trait borrowing issue with closure.
pub struct VisitorWrap<N> {
    visitor: ~Visitor<N>
}

impl <N> Visitor<N> for VisitorWrap<N> {
    fn visit(&mut self, tree: &mut Tree<N>) { self.visitor.visit(tree) }
}

pub fn wrap_visitor<N>(v: ~Visitor<N>) -> VisitorWrap<N> {
    VisitorWrap{visitor: v}
}

#[test]
fn test_parent() {
    let mut tree = Tree::new(0);
    let mut tree1 = Tree::new(1);
    let tree2 = Tree::new(2);
    tree1.add(tree2);
    tree.add(tree1);
    
    assert_eq!(tree.childs[0].get_parent().unwrap().data, tree.data);
}
