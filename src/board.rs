use std::int;
use std::vec;
use extra::bitv;

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

#[deriving(Eq)]
enum Tile {
    Floor,
    Wall,
    Box(uint),
    Target,
    BoxOnTarget
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

pub struct Position {
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
        // let mut mask = Mask::new(self.board.data, |til| { *til == Box(box) };
        // if (mask.occurences() == 0) { return None;}
        None
    }
    
    fn get_player_position(&self) -> (uint, uint) {
        self.player
    }
    
    fn get_position_mask(&self) -> ~Mask {
        Mask::new(self.board.data, |_| { false })
    }
}

#[test]
fn test_tile() {
    assert!(Box(1) == Box(1));
    assert!(Box(1) != Box(2));
    assert!(Box(1) != Wall);
    assert!(Box(0).is_box());
}

