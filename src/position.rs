use bit_vec::BitVec;

use bitv;

use board::Board;

#[derive(Hash, Eq, PartialEq)]
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
        boxes.set(mov.player_position, false);
        boxes.set(mov.box_position, true);
        let mut player =
            if self.player.get(mov.box_position) != Some(true) {
                self.player.clone()
            } else {
                BitVec::from_elem(self.player.len(), false)
            };

        player.set(mov.player_position, true);

        Position {
            boxes: boxes,
            player: player
        }
    }

    /// Expand the player possibilities according to the board and boxes
    pub fn expand(&mut self, board: &Board) {
        let (x, _) = board.size;

        let mut mask = board.floor.clone();
        mask.difference(&self.boxes);
        let mask = mask; // remove mutability

        let mut old = self.player.clone();
        let mut result = bitv::expand(&old, x);
        result.intersect(&mask);

        while old != result {
            loop {
                old = result;
                result = bitv::expand(&old, x);
                if result.intersect(&mask) { break; } // forego check if the result was not cropped
            }
        }
        self.player = result;
    }

    /// Number of targets left.
    pub fn win(&self, board: &Board) -> bool {
        self.boxes == board.targets
    }

    /// Gives the possible next moves according to board. The position must be expanded.
    pub fn moves(&self, board: &Board) -> Vec<Move> {
        let (line_size, _) = board.size;

        return self.boxes.iter()
            .zip((0 .. ))
            .filter_map(|(bit, pos)| if bit { Some(pos) } else { None })
            .flat_map(|pos| vec![
                (pos - 1, pos, pos + 1),
                (pos + 1, pos, pos - 1),
                (pos - line_size, pos, pos + line_size),
                (pos + line_size, pos, pos - line_size),
                ].into_iter()
            )
            .filter(|&(prev, _, _)| self.player.get(prev) == Some(true))  // player can reach
            .filter(|&(_, _, next)| board.places.get(next) == Some(true))  // empty space for box
            .filter(|&(_, _, next)| self.boxes.get(next) != Some(true))  // no other box there
            .map(|(_, curr, next)| Move{player_position: curr, box_position: next})
            .collect()
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Move {
    pub player_position: usize,
    pub box_position: usize,
}
