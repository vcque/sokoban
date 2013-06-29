use std::int;
use std::vec;

/// The interface a sokoban game implements.
trait Game {

    /// Game is won when it returns 0. (no target left)
    fn from_win(&self) -> uint;
    /// The size of the game.
    fn size(&self) -> (uint, uint);
    /// Get the tile at position (x,y).
    fn get_tile(&self, x: uint, y: uint) -> Tile;
    /// Get the number of boxes on game.
    fn get_box_number(&self) -> uint;
    /// Get the position of box number i.
    fn get_box_position(&self, box: uint) -> Option<(uint,uint)>;
    /// Get all possible next positions for box number i.
    fn get_box_mask(&self, box: uint) -> Option<~Mask>;
    /// Get the player position.
    fn get_player_position(&self) -> (uint, uint);
    /// Get all possible next positions of the player.
    fn get_position_mask(&self) -> ~Mask;
}

/// Represents each block of a given board.
#[deriving(Eq)]
enum Tile {
    /// Basic passable block.
    Floor,
    /// Putting a box here means loosing, it is passable.
    DeadSpot,
    /// Basic un-passable block.
    Wall,
    /// A box with its id.
    Box(uint),
    /// You need to put a box on it for winning, it is passable.
    Target,
    /// A target with a box on it.
    BoxOnTarget
}

impl Tile {

    fn is_passable(&self) -> bool {
        match *self {
        Floor => { true },
        DeadSpot => { true }
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

impl Position {
    priv fn flow_mask(&self, mask: &mut Mask, (i,j): (int, int)) {
        
    }
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
    
    fn get_box_mask(&self, box: uint) -> Option<~Mask> {
        let mut mask = Mask::new(self.board.data, |til| { *til == Box(box) });
        if (mask.occurences() == 0) { return None; }
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

