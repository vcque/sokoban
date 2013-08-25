use std::uint;

use bitv::Bitv;
use board::Board;
use board::{Move, UP, LEFT, RIGHT, DOWN};

pub struct Position {
    // Data that don't change from one position to another.
    board: @Board,
        
    /// The boxes' positions
    boxes: Bitv,
    /// The player's position in linear coordinate (used mainly for the ToStr impl)
    playerPosition: uint,
    /// The player's possible movements
    player: Bitv
}

impl Eq for Position {
    fn eq(&self, other: &Position) -> bool {
        self.boxes == other.boxes && self.player == other.player
    }

    fn ne(&self, other: &Position) -> bool {
        !self.eq(other)
    }
}

impl Position {

    /// Constructor from a board.
    pub fn new(board: @Board) -> Position {
        let pp = build_possibles(board, &board.initial, board.player);

        return Position { 
            board: board,
            boxes: board.initial.clone(),
            playerPosition: board.player,
            player: pp
        }
    }

    /// Constructor from another game.
    pub fn from_game(game: &Position, mov: Move) -> Position {
        let mut boxes = game.boxes.clone();
        let (x, _) = game.board.size;
        let n = mov.position;
        
        boxes.set(n, false);
        boxes.set(
            match (mov.mov) {
            UP      => {n - x},
            LEFT    => {n - 1},
            RIGHT   => {n + 1},
            DOWN    => {n + x}
            }
            , true);
        
        let pp = build_possibles(game.board, &boxes, mov.position);
        return Position {
            board: game.board,
            boxes: boxes,
            playerPosition: mov.position,
            player: pp
        }
    }

    /// Number of targets left.
    pub fn is_win(&self) -> bool {
        self.boxes == self.board.target
    }
    
    pub fn moves(&self) -> ~[Move] {
        let mut result = ~[];
        let (x,_) = self.board.size;

        let up = self.boxes >> x;
        let down = self.boxes << x;
        let left = self.boxes >> 1;
        let right = self.boxes << 1;
        
        let room = self.board.places & !self.boxes;
        
        { // Up case
            let box = up & room;
            let player = down & self.player;
            let mov = box & player >> 2*x;
            for uint::range(0, mov.length) |i| {
                if (mov[i]) {
                    result.push(Move{position: i + x, mov: UP}); 
                }
            }
        }
        { // Down case
            let box = down & room;
            let player = up & self.player;
            let mov = box & player << 2*x;
            for uint::range(0, mov.length) |i| {
                if (mov[i]) {
                    result.push(Move{position: i - x, mov: DOWN}); 
                }
            }
        }
        { // Left case
            let box = left & room;
            let player = right & self.player;
            let mov = box & player >> 2;
            for uint::range(0, mov.length) |i| {
                if (mov[i]) {
                    result.push(Move{position: i + 1, mov: LEFT}); 
                }
            }
        }
        { // Right case
            let box = right & room;
            let player = left & self.player;
            let mov = box & player << 2;
            for uint::range(0, mov.length) |i| {
                if (mov[i]) {
                    result.push(Move{position: i - 1, mov: RIGHT}); 
                }
            }
        }

        return result;
    }
}

impl ToStr for Position {

    fn to_str(&self) -> ~str {
        let (x, y) = self.board.size;
        let mut result = ~"";
        
        for uint::range(0, y) |j| {
            for uint::range(0, x) |i| {
                let n = self.board.to_linear_coord((i, j));
                if n == self.playerPosition {
                    result = result.append("@");
                } else if !self.board.floor[n] {
                    result = result.append("#");
                } else if self.boxes[n] {
                    result = result.append("$");
                } else if self.board.target[n] {
                    result = result.append(".");
                } else {
                    result = result.append(" ");
                }
            }
            result = result.append("\n");
        }
        
        return result;
    }
}

fn build_possibles(board: &Board, boxes: &Bitv, pos: uint) -> Bitv {
    let (x, y) = board.size;
    
    let mut old = Bitv::new(x * y);
    let mut result = Bitv::new(x * y);
    result.set(pos, true);
    let noBox = !boxes;
    
    while (old != result) {
        old = result;
        
        let up = old >> x;
        let down = old << x;
        let left = old >> 1;
        let right = old << 1;
        
        result = old | left | right | up | down;
        result = result & board.floor & noBox;
    }
    
    return result;
}

