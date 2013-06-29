/// Simple multi-children tree.
pub struct Tree<N> {
    data: N,
    childs: ~[Tree<N>]
}

impl <N> Tree<N> {

    pub fn new(dat: N) -> Tree<N> {
        Tree{data: dat, childs: ~[]}
    }
    
    pub fn accept(&mut self, visitor: &mut VisitorWrap<N>) {
        visitor.visit(self);
        for self.childs.each_mut |tree| {
            tree.accept(visitor);
        }
    }
    
    pub fn accept_consume(&mut self, visitor: &mut VisitorWrap<N>) {
        self.accept(visitor);
        self.childs.clear();
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
