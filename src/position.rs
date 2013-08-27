use std::uint;

use bitv::Bitv;
use board::Board;
use board::{Move, UP, LEFT, RIGHT, DOWN};

fn build_possibles(board: &Board, boxes: &Bitv, pos: uint) -> Bitv {
    let (x, y) = board.size;
    let n = x * y;

    let noBox = !boxes;
    let mut old = Bitv::new(n);
    let mut result = Bitv::new(n);
    let buf = &mut Bitv::new(n);
    
    result.set(pos, true);

    while (old != result) {
        old.assign(&result);
        buf.assign(&old);
        result.assign_union(buf);
                
        buf.assign_shift(1);
        result.assign_union(buf);
        
        buf.assign_shift_back(2);
        result.assign_union(buf);

        buf.assign_shift_back(x-1);
        result.assign_union(buf);

        buf.assign_shift(2*x);
        result.assign_union(buf);

        result.assign_intersect(&board.floor);
        result.assign_intersect(&noBox);
    }
    
    return result;
}

pub struct Position {
    // Data that don't change from one position to another.
    board: @Board,
    /// The boxes' positions
    boxes: Bitv,
    /// The player's possible movements
    player: Bitv,
    /// The player's position in linear coordinate (used mainly for the ToStr impl)
    playerPosition: uint
}

impl Eq for Position {
    fn eq(&self, other: &Position) -> bool {
        self.boxes == other.boxes && self.player == other.player
    }

    fn ne(&self, other: &Position) -> bool {
        !self.eq(other)
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
    pub fn from_move(game: &Position, mov: Move) -> Position {
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
        
        let mut pp =
            if ( game.player & boxes == Bitv::new(boxes.length) ) {
                game.player.clone()            
            } else {
                Bitv::new(game.player.length)
            };
            
        pp.set(n, true);
        
        let mut result = Position {
            board: game.board,
            boxes: boxes,
            playerPosition: mov.position,
            player: pp
        };
        
        result.expand_player();
        return result;
    }

    /// Number of targets left.
    pub fn is_win(&self) -> bool {
        self.boxes == self.board.target
    }
    
    pub fn moves(&self) -> ~[Move] {
        let mut result = ~[];
        let (x,_) = self.board.size;

        let room = self.board.places & !self.boxes;
        let upRoom = room >> x;
        let downRoom = room << x;
        let leftRoom = room >> 1;
        let rightRoom = room << 1;
        
        let upPlayer = self.player >> x;
        let downPlayer = self.player << x;
        let leftPlayer = self.player >> 1;
        let rightPlayer = self.player << 1;
        
        let up = self.boxes & upPlayer & downRoom;
        let down = self.boxes & downPlayer & upRoom;
        let left = self.boxes & leftPlayer & rightRoom;
        let right = self.boxes & rightPlayer & leftRoom;
        
        for uint::range(0, up.length) |i| {
            if (up[i]) {
                result.push(Move{position: i, mov: UP}); 
            }
            if (down[i]) {
                result.push(Move{position: i, mov: DOWN}); 
            }
            if (left[i]) {
                result.push(Move{position: i, mov: LEFT}); 
            }
            if (right[i]) {
                result.push(Move{position: i, mov: RIGHT}); 
            }
        }

        return result;
    }
    
    fn expand_player(&mut self) {
        let (x, _) = self.board.size;
        let length = self.player.length;

        let noBox = !self.boxes;
        let mut old = Bitv::new(length);
        let buf = &mut Bitv::new(length);
        let result = &mut self.player;
        
        while (&old != result) {
            old.assign(result);
            buf.assign(&old);
            result.assign_union(buf);
                    
            buf.assign_shift(1);
            result.assign_union(buf);
            
            buf.assign_shift_back(2);
            result.assign_union(buf);

            buf.assign_shift_back(x-1);
            result.assign_union(buf);

            buf.assign_shift(2*x);
            result.assign_union(buf);

            result.assign_intersect(&self.board.floor);
            result.assign_intersect(&noBox);
        }
    }

}
