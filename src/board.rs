use std::str;
use std::uint;

use bitv::Bitv;

pub struct Board {
    /// Size of the board.
    size: (uint,uint),
    /// Passable tiles.
    floor: Bitv,
    /// Possible box positions.
    places: Bitv,
    /// Target tiles.
    target: Bitv,
    /// Initial boxes position.
    initial: Bitv,
    /// Initial player position.
    player: uint
}

impl Board {

    pub fn new(size: (uint, uint), data: ~[~str]) -> Board {
        let (x, y) = size;
        let mut result = Board {
            size: size,
            floor: Bitv::new(x * y),
            places: Bitv::new(x * y),
            target: Bitv::new(x * y),
            initial: Bitv::new(x * y),
            player: 0
        };
        
        let mut nbrBox = 0;
        let mut nbrTarget = 0;
        
        for uint::range(0, y) |j| {
            let toParse = &data[j];
            let mut i = 0;
            for toParse.iter().advance() |ch| {
                let n = i + j * x;
                match (ch) {
                    ' ' => { // Floor
                        result.floor.set(n, true);
                    },
                    '#' => {}, // Wall
                    '$' => { // Box
                        result.initial.set(n, true);
                        result.floor.set(n, true);
                        nbrBox += 1;
                    },
                    '.' => { // Target
                        result.target.set(n, true);
                        result.floor.set(n, true);
                        nbrTarget += 1;
                    },
                    '@' => { // Player
                        result.floor.set(n, true);
                        result.player = n;
                    },
                    '*' => { // Box on target
                        result.floor.set(n, true);
                        result.initial.set(n, true);
                        result.target.set(n, true);
                        nbrTarget += 1;
                        nbrBox += 1;
                    },
                    '+' => { // Player on target
                        result.floor.set(n, true);
                        result.target.set(n, true);
                        result.player = n;
                        nbrTarget += 1;
                    },
                    c => {
                        fail!("Wrong character : " + str::from_char(c));
                    }
                }
                i += 1;
            }
        }
            
        assert!(nbrBox >= nbrTarget, "Impossible game, there's more targets than boxes !");
            
        let mut places;
        // TODO refine deadspot places.
        let up = result.floor >> x;
        let down = result.floor << x;
        let left = result.floor >> 1;
        let right = result.floor << 1;

        places = (up | left) & (left | down) & (down | right) & (right | up);
        places = places | result.target;
        places = places & result.floor;
        
        result.places = places;
        return result;
    }
    
    pub fn to_square_coord(&self, pos: uint) -> (uint, uint) {
        let (x,_) = self.size;
        return (pos % x, pos / x);
    }
    
    pub fn to_linear_coord(&self, (i, j) : (uint, uint)) -> uint {
        let (x,_) = self.size;
        return i + x * j;
    }
}

#[deriving(Eq, Clone)]
pub struct Move {
    position: uint,
    mov: Movement
}

#[deriving(Eq, Clone)]
pub enum Movement {
    UP,
    RIGHT,
    LEFT,
    DOWN
}
