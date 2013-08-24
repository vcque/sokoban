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
            Ok(file) => { resolve(file); },
            Err(e) => { fail!(e); }
        }},
    _ => { fail!("No file path found."); }
    }
}

fn resolve(reader: @Reader) {
    let mut x = 0;
    let mut y = 0;
    let mut data = ~"";  
    do reader.each_line |line| {
        match (x) {
        0 => {x = line.len();},
        _ => { if x != line.len() { fail!("Wrong file format !") } }
        }
        data = data.append(line);
        y += 1;
        
        true
    };
    
    let b = @Board::new((x, y), data);
    let p = Position::new(b);

    let mut t = Tree::new(p);
    let sv = ~SokobanVisitor{found: false} as ~Visitor<Position>;
    t.accept_consume(&mut tree::wrap_visitor(sv));    
}

#[test]
fn test() {
    let mut data = ~"";
    data = data.append("000000000");
    data = data.append("000  0000");
    data = data.append("0     X 0");
    data = data.append("0 0  0X 0");
    data = data.append("0 T T0@ 0");
    data = data.append("000000000");
    
    let b = @Board::new((9, 6), data);
    let g = Position::new(b);

    let mut t = Tree::new(g);
    let sv = ~SokobanVisitor{found: false} as ~Visitor<Position>;
    t.accept_consume(&mut tree::wrap_visitor(sv));    
}

struct SokobanVisitor {
    found: bool
}

impl Visitor<Position> for SokobanVisitor {
    fn visit(&mut self, tree: &mut Tree<Position>) {
        println(depth(tree).to_str());
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
