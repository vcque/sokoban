use std::iter::repeat;
use bit_vec::BitVec;

use board::Board;
use bitv;

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct Position {
    /// The boxes' positions
    pub boxes: BitVec,
    /// The player's possible movements
    pub player: BitVec,
}

impl Position {

    /// Build a new position from a move. Needs to be expanded.
    pub fn move_to(&self, mov: &Move) -> Position {
        let mut boxes: BitVec = self.boxes.clone();
        boxes.set(mov.from as usize, false);
        boxes.set(mov.to as usize, true);

        let mut player =
            if self.player.get(mov.to as usize) != Some(true) {
                self.player.clone()
            } else {
                BitVec::from_elem(self.player.len(), false)
            };

        player.set(mov.player as usize, true);

        Position {
            boxes: boxes,
            player: player
        }
    }

    /// Expand the player possibilities according to the board and boxes
    pub fn expand(&mut self, board: &Board) {
        let x = board.size.0;

        let mut mask: BitVec = board.floor.clone();
        mask.difference(&self.boxes);

        let mut back_buf: BitVec = self.player.clone();
        let mut front_buf: BitVec = self.player.clone();

        bitv::expand_from(&back_buf, &mut front_buf, x);
        front_buf.intersect(&mask);

        while back_buf != front_buf {
            let swap = back_buf;
            back_buf = front_buf;
            front_buf = swap;

            bitv::expand_from(&back_buf, &mut front_buf, x);
            front_buf.intersect(&mask);
        }

        self.player.clone_from(&front_buf);
    }

    /// Number of targets left.
    pub fn win(&self, board: &Board) -> bool {
        self.boxes.storage() == board.targets.storage()
    }

    /// Gives the possible next moves according to board. The position must be expanded.
    pub fn moves(&self, board: &Board) -> Vec<Move> {

        // print_position(board, self);
        let (line_size, _) = board.size;
        let dirs: [i16; 4] = [-1, 1, -(line_size as i16), line_size as i16];

        return self.boxes.iter()
            .zip((0 .. ))
            .filter_map(|(bit, pos)| if bit { Some(pos as i16) } else { None })
            .flat_map(|pos| repeat(pos).zip(dirs.iter()))
            // .inspect(|&(from, to)| println!("{}, dir {} can be pushed ?", from, to))
            .filter(|&(pos, dir)| self.player.get((pos - dir) as usize) == Some(true))  // player can reach
            .flat_map(|(pos, dir)| {
                repeat(pos).zip(repeat(dir).zip(1 as i16..))
                .map(|(a, (b, c))| (a, a + b * (c - 1),  a + b * c))
                // .inspect(|&(from, to)| println!("{} to {} has room ?", from, to))
                .take_while(|&(_, _, to)|
                    board.places.get(to as usize) == Some(true)
                    && self.boxes.get(to as usize) != Some(true))
            })
            .map(|(from, player, to)| Move{from: from, player: player, to: to})
            .collect()
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Move {
    pub from: i16,
    pub to: i16,
    pub player: i16,
}
