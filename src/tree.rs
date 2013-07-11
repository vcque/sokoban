use std::str::push_str;

/// Simple multi-children tree.
pub struct Tree<N> {
    data: N,
    depth: uint,
    childs: ~[Tree<N>]
}

impl <N> Tree<N> {

    pub fn new(dat: N) -> Tree<N> {
        Tree{data: dat, depth: 0, childs: ~[]}
    }
    
    pub fn accept(&mut self, visitor: &mut VisitorWrap<N>) {
        visitor.visit(self);
        for self.childs.mut_iter().advance |tree| {
            tree.accept(visitor);
        }
    }
    
    pub fn accept_consume(&mut self, visitor: &mut VisitorWrap<N>) {
        self.accept(visitor);
        self.childs.clear();
    }
    
    pub fn add_child(&mut self, child: Tree<N>) {
        self.childs.push(child);
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
    let tree1 = Tree::new(1);
    let tree2 = Tree::new(2);
    tree.add_child(tree1);
    tree.add_child(tree2);
    
    print(tree.to_str());
}
