extern mod extra (vers = "0.7");

use std::uint;
use std::vec;
use board::{Position, Game, Mask};
use tree::{Tree, Visitor, wrap_visitor};

mod board;
mod tree;


fn resolve_game(game: Position) {
    let visitor = SokobanVisitor::new();
    let mut tree = Tree::new(game);
    tree.accept_consume(&mut wrap_visitor(visitor as ~Visitor<Position>));
}
    


struct SokobanVisitor {
    positions: ~[Position]
}

impl SokobanVisitor {
    fn new() -> ~SokobanVisitor { ~SokobanVisitor{ positions: ~[] } }
}

impl Visitor<Position> for SokobanVisitor {

    fn visit(&mut self, tree: &mut Tree<Position>) {
    
        let current = tree.data.clone();
        self.positions.truncate(tree.depth);
        
        // 1. Check if win or nearer from victory than parent. (box on target)
        let better = 
            match (self.positions.last_opt()) {
            Some(parent) => { parent.from_win() < current.from_win() },
            None => { true }
            };
        
        // 2. Calculate the masks.
        let nbrBox = current.get_box_number();
        for uint::range(0,nbrBox) |i| {
            // current.get_box_mask(i);
            match (current.get_box_mask(i)) {
            Some(mask) => { self.create_positions(tree, i, &current, mask) },
            _ => {}
            }
        }
        // 3. Check if this position is contained into one of the above.
        // 4. Creates as many new positions as there are possibilities.
        self.positions.push(current);
        
    } 
}

impl SokobanVisitor {

    fn create_positions(self, tree: &mut Tree<Position>, box: uint, parent: &Position, possibles: &Mask) {
        let (x,y) = parent.get_box_position(box).get();
        let p = possibles.as_positions();
        for p.iter.advance |(i,j)| {
            if ( x != i && y != j ) {
                let mut newOne = parent.clone();
                newOne.board.data[i][j] = Tile::Floor; 
            }
        }
    }

}
