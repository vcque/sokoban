extern mod extra (vers = "0.7");

use std::io;
use std::os;

use board::Board;
use position::Position;
use tree::{Tree, Visitor};

mod bitv;
mod board;
mod position;
mod tree;

fn main() {
    match os::args() {
    [_, b] => { 
        match io::file_reader(&Path(b)) {
            Ok(file) => { parse(file); },
            Err(e) => { fail!(e); }
        }},
    _ => { fail!("No file path found."); }
    }
}

fn parse(reader: @Reader) {
    let mut x: uint = 0;
    let mut y: uint = 0;
    let mut data = ~[];

    do reader.each_line |line| {
        x = x.max(&line.len());
        y += 1;
        data.push(line.to_owned());
        
        true
    };

    let b = @Board::new((x, y), data);
    let p = Position::new(b);

    let mut t = Tree::new(p);
    let sv = ~SokobanVisitor{found: false} as ~Visitor<Position>;
    t.accept_consume(&mut tree::wrap_visitor(sv));    
}

struct SokobanVisitor {
    found: bool
}

impl Visitor<Position> for SokobanVisitor {
    fn visit(&mut self, tree: &mut Tree<Position>) {
        if (self.found) {return;}
        let mov = &tree.data.moves();
        for mov.iter().advance |m| {
            let new = Position::from_game(&tree.data, m.clone());
            if new.is_win() {
                self.found = true;
                print_moves(tree);
            }
            if !has_equal_parent(&new, tree) {
                let t = Tree::new(new);
                tree.add(t);
            }
        }
    }
    
}

fn has_equal_parent(p: &Position, tree: &Tree<Position>) -> bool {
    if (p == &tree.data) {
        return true;
    }
    match (tree.get_parent()) {
        Some(t) => { has_equal_parent(p, t) }
        None => { false }
    }
}

fn print_moves(tree: &Tree<Position>) {
    match tree.get_parent() {
        Some(p) => { print_moves(p); },
        _ => {}
    }
    
    println(tree.data.to_str());
} 

fn depth(tree: &Tree<Position>) -> uint {
    match tree.get_parent() {
        Some(p) => { depth(p) + 1 },
        _ => {0}
    }
}

#[test]
fn test() {
    let mut data = ~[];
    data.push(~"#########");
    data.push(~"###  ####");
    data.push(~"#     $ #");
    data.push(~"# #  #$ #");
    data.push(~"# . .#@ #");
    data.push(~"#########");
    
    let b = @Board::new((9, 6), data);
    let g = Position::new(b);

    let mut t = Tree::new(g);
    let sv = ~SokobanVisitor{found: false} as ~Visitor<Position>;
    t.accept_consume(&mut tree::wrap_visitor(sv));    
}

