use std::int;
use std::vec;
use std::str::push_str;

/// The interface a sokoban game implements.
pub trait Game {

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
#[deriving(Eq, Clone)]
enum Tile {
    /// Basic passable block.
    Floor,
    /// Putting a box here means loosing, it is passable.
    DeadSpot,
    /// Basic un-passable block.
    Wall,
    /// A box with its id for keeping track of it.
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

impl ToStr for Tile {

    fn to_str(&self) -> ~str {
        match *self {
        Floor => {~" "},
        DeadSpot => {~" "},
        Wall => {~"0"},
        Box(_) => {~"B"},
        Target => {~"X"},
        BoxOnTarget => {~"T"}
        }
    }
}

/*
The mask data structure.
*/
#[deriving(Eq)]
struct Mask { data: ~[~[bool]] }

impl Mask {

    fn new(board: &[~[Tile]], f: &fn(&Tile) -> bool) -> ~Mask {
        let mut result = vec::with_capacity(board.len());
    
        for board.iter().advance |col| {
            let mut column = vec::with_capacity(board.len());
            for col.iter().advance |til| {
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
    
    fn diff(&self, other: &Mask) -> ~Mask {
        if (self.size() != other.size()) { fail!("Mask not of the same size !!") }
        let (x,y) = self.size();
        let mut result = ~*self.clone();
        for int::range(0, x.to_int()-1) |i| {
            for int::range(0, y.to_int()-1) |j| {
                result.data[i][j] = self.data[i][j] && !other.data[i][j];
            }
        }
        
        return result;
    }
    
    fn size(&self) -> (uint, uint) { 
        match self.data.len() {
        0 => { (0, 0) },
        x => { (x, self.data[0].len()) }
        }
    }

    fn occurences(&self) -> uint {
        let mut result = 0;
        for self.data.iter().advance |col| {
            for col.iter().advance |exists| {
                if (*exists) { result += 1 }
            }
        }
        result
    }
    
    fn as_positions(&self) -> ~[(uint,uint)] {
        let mut result = ~[];
        let mut x = 0;
        for self.data.iter().advance |col| {
            let mut y = 0;
            for col.iter().advance |exists| {
                if (*exists) { result.push((x,y)); }
                y = y+1;
            }
            x = x+1;
        }
        
        result
    }
}

impl ToStr for Mask {
    fn to_str(&self) -> ~str {
        let mut result = ~"";
        for self.data.iter().advance |col| {
            for col.iter().advance |b| {
                if *b {push_str(&mut result, "1");}
                else {push_str(&mut result, "0");}
            }
            push_str(&mut result, "\n");
        }
    result
    }
}

#[deriving(Eq, Clone)]
struct Board { data: ~[~[Tile]] }

#[deriving(Eq, Clone)]
pub struct Position {
    board: Board,
    player: (uint, uint)
}

impl Position {

    // TODO : taking the position mask into account.
    priv fn box_flow_mask(&self, mask: &mut Mask, (i,j): (uint, uint)) {
        
        // x axis.
        let xl = self.get_tile(i-1, j);
        let xr = self.get_tile(i+1, j);
        let xlm = mask.data[i-1][j];
        let xrm = mask.data[i+1][j];
        if ((xl.is_passable() || xlm) && (xr.is_passable() || xrm)) {
            if (!xlm) {
                mask.data[i-1][j] = true;
                self.box_flow_mask(mask, (i-1, j));
            }
            if (!xrm) {
                mask.data[i+1][j] = true;
                self.box_flow_mask(mask, (i+1, j));
            }
        }

        // y axis.        
        let yl = self.get_tile(i, j-1);
        let yr = self.get_tile(i, j+1);
        let ylm = mask.data[i][j-1];
        let yrm = mask.data[i][j+1];
        if ((yl.is_passable() || ylm) && (yr.is_passable() || yrm)) {
            if (!ylm) {
                mask.data[i][j-1] = true;
                self.box_flow_mask(mask, (i, j-1));
            }
            if (!yrm) {
                mask.data[i][j+1] = true;
                self.box_flow_mask(mask, (i, j+1));
            }
        }
    }
    

    priv fn flow_mask(&self, mask: &mut Mask, (i,j): (uint, uint)) {
        // x axis.
        let xl = self.get_tile(i-1, j);
        let xr = self.get_tile(i+1, j);
        let xlm = mask.data[i-1][j];
        let xrm = mask.data[i+1][j];
        if (xl.is_passable() && !xlm) {
            mask.data[i-1][j] = true;
            self.flow_mask(mask, (i-1, j));
        }
        if (xr.is_passable() && !xrm) {
            mask.data[i+1][j] = true;
            self.box_flow_mask(mask, (i+1, j));
        }

        // y axis.        
        let yl = self.get_tile(i, j-1);
        let yr = self.get_tile(i, j+1);
        let ylm = mask.data[i][j-1];
        let yrm = mask.data[i][j+1];
        if (yl.is_passable() && !ylm) {
            mask.data[i][j-1] = true;
            self.flow_mask(mask, (i, j-1));
        }
        if (yr.is_passable() && !yrm) {
            mask.data[i][j+1] = true;
            self.flow_mask(mask, (i, j+1));
        }
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
        let (n,m) = self.size();
        if ( x < 0  || y < 0 || x >= n || y >= m ) {
            Wall
        } else {
            self.board.data[x][y]
        }
    }
    
    fn get_box_number(&self) -> uint {
        Mask::new(self.board.data, |til| {til.is_box()}).occurences()
    }
    
    fn get_box_position(&self, box: uint) -> Option<(uint, uint)> {
        let mut result = None;
        let mut i = 0;
        for self.board.data.iter().advance |col| {
        let mut j = 0;
            for col.iter().advance |cell| {
                if (*cell == Box(box)) { result = Some((i,j)); }
                j = j + 1;
            }
        i = i + 1;
        }
        return result;
    }
    
    fn get_box_mask(&self, box: uint) -> Option<~Mask> {
        let mut mask = Mask::new(self.board.data, |til| { *til == Box(box) });
        if (mask.occurences() == 0) { return None; }
        else {
            let pos = self.get_box_position(box);
            self.box_flow_mask(mask, pos.get());
            Some(mask)
        }
    }
    
    
    fn get_player_position(&self) -> (uint, uint) {
        self.player
    }
    
    fn get_position_mask(&self) -> ~Mask {
        let mut result = Mask::new(self.board.data, |_| { false });
        let (x,y) = self.player;
        result.data[x][y] = true;
        self.flow_mask(result, self.player);
        result
    }
}

#[test]
fn test_tile() {
    assert!(Box(1) == Box(1));
    assert!(Box(1) != Box(2));
    assert!(Box(1) != Wall);
    assert!(Box(0).is_box());
}

#[test]
fn test_flow_mask() {
    let mut tiles = vec::with_capacity(10);
    for int::range(0,10) |x| {
        let col = vec::from_fn(10, |i| {
            match (x, i) {
            (5,5) => {Box(0)},
            (0,_) => {Wall},
            (9,_) => {Wall},
            (_,0) => {Wall},
            (_,9) => {Wall},
            // (1,3) => {Floor},
            // (_,3) => {Wall},
            (_,_) => {Floor}
            }
        });
        tiles.push(col);
    }

    let b = Board{data: tiles};
    let pos = Position{board: b, player: (4,4)};
    
    let flowMask = pos.get_box_mask(0);
    let mask = Mask::new(pos.board.data, |til| {
        *til != Wall
    });
    
    assert!(flowMask.get() == mask);
}

