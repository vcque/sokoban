/*
TODO : Changer Mask en double tableau de bool.
TODO : Faire le flow mask (sans gérer la position. Puis avec.)


*/

use std::int;
use std::vec;
use tree::{Tree, Visitor, wrap_visitor};

trait Game {
    fn from_win(&self) -> uint;
    fn size(&self) -> (uint, uint);
    fn get_tile(&self, x: uint, y: uint) -> Tile;
    fn get_box_number(&self) -> uint;
    fn get_box_position(&self, box: uint) -> Option<(uint,uint)>;
    fn get_box_mask(&self, box: uint) -> Option<Mask>;
    fn get_player_position(&self) -> (uint, uint);
    fn get_position_mask(&self) -> ~Mask;
}

enum Tile {
    Floor,
    Wall,
    Box(uint),
    Target,
    BoxOnTarget
}

impl Eq for Tile {

    fn eq(&self, other: &Tile) -> bool {
        match *self {
        Box(n) => {*other == Box(n)},
        a => {*other == a}
        }
    }
    
    fn ne(&self, other: &Tile) -> bool { !self.eq(other) }
    
}

impl Tile {

    fn is_passable(&self) -> bool {
        match *self {
        Floor => { true },
        Target => { true },
        _ => { false }
        }
    }
    
    fn is_box(&self) -> bool {
        match *self {
        Box(_) => { true },
        BoxOnTarget => { true },
        _ => { false }
        }
    }
}

/*
The mask data structure.
*/
struct Mask { data: ~[~[bool]] }

impl Mask {

    fn new(board: &[~[Tile]], f: &fn(&Tile) -> bool) -> ~Mask {
        let mut result = vec::with_capacity(board.len());
    
        for board.each |col| {
            let mut column = vec::with_capacity(board.len());
            for col.each |til| {
                column.push(f(til));   
            }
            result.push(column);
        }   
        
        ~Mask{data: result}
    }

    fn contains(&self, other: &Mask) -> bool {
        if (self.size() != other.size()) { fail!("Mask not of the same size !!") }
        
        let mut result = true;
        let (x,y) = self.size();
        for int::range(0, x.to_int()-1) |i| {
            for int::range(0, y.to_int()-1) |j| {
                let notContain = !self.data[i][j] && other.data[i][j];
                result = result && !notContain;
            }
        }
        
        result
    }
    
    fn size(&self) -> (uint, uint) { 
        match self.data.len() {
        0 => { (0, 0) },
        x => { (x, self.data[0].len()) }
        }
    }

    fn occurences(&self) -> uint {
        let mut result = 0;
        for self.data.each |col| {
            for col.each |exists| {
                if (*exists) { result += 1 }
            }
        }
        result
    }
}

struct Board { data: ~[~[Tile]] }

struct Position {
    board: Board,
    player: (uint, uint)
}

impl Game for Position {

    fn from_win(&self) -> uint {
        Mask::new(self.board.data, |til| {*til == Target}).occurences()
    }
    fn size(&self) -> (uint, uint) {
        let b = &(self.board.data);
        match (b.len()) {
        0 => { (0, 0) },
        i => { (i, b[0].len()) }
        }
        
    }
    fn get_tile(&self, x: uint, y: uint) -> Tile {
        self.board.data[x][y]
    }
    fn get_box_number(&self) -> uint {
        Mask::new(self.board.data, |til| {til.is_box()}).occurences()
    }
    fn get_box_position(&self, box: uint) -> Option<(uint, uint)> {
        let mut x = None;
        for self.board.data.eachi |i, col| {
            for col.eachi |j, cell| {
                if (*cell == Box(box)) { x = Some((i,j)) }
            }
        }
        
        return x;
    }
    
    fn get_box_mask(&self, box: uint) -> Option<Mask> {
        let mut mask = Mask::new(self.board.data, |til| { *til == Box(box) };
        if (mask.occurences() == 0) { return None;}
        
    }
    fn get_player_position(&self) -> (uint, uint) {
        self.player
    }
    fn get_position_mask(&self) -> ~Mask {
        Mask::new(self.board.data, |_| { false })
    }
    
}

fn resolve_game(game: &mut Position) {
    
    let mut tree = Tree{data: game, parent: None, childs: ~[]};
    let visitor = ~Visitor{f: |tree| {
        // 1. Check if win or nearer from victory. (box on target)
                
        // 2. Calculate the masks.
        // 3. Check if this position is contained into one of the above.
        // 4. Creates as many new positions as there are possibilities.
    }};
    let wrap = wrap_visitor(visitor);
    tree.accept_consume(&mut wrap);
}
// */