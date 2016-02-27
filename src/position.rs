use std::iter::repeat;

use board::Board;
use bitv::BitvImpl;
use bitv::Bitv;

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct Position {
    /// The boxes' positions
    pub boxes: BitvImpl,
    /// The player's possible movements
    pub player: BitvImpl,
}

impl Position {

    /// Build a new position from a move. Needs to be expanded.
    pub fn move_to(&self, mov: &Move) -> Position {
        let mut boxes: BitvImpl = self.boxes.clone();
        boxes.unset_bit(mov.from as usize);
        boxes.set_bit(mov.to as usize);

        let mut player =
            if self.player.get_bit(mov.to as usize) {
                self.player.clone()
            } else {
                [0; 4]
            };

        player.set_bit(mov.player as usize);

        Position {
            boxes: boxes,
            player: player
        }
    }

    /// Expand the player possibilities according to the board and boxes
    pub fn expand(&mut self, board: &Board) {
        let x = board.size.0 as usize;

        let mut mask: BitvImpl = board.floor.clone();

        let mut box_mask = self.boxes.clone();
        box_mask.neg();

        mask.and(&box_mask);
        let mut space = self.player.clone();

        space.expand_from(&self.player, x);
        space.and(&mask);

        while space != self.player {
            self.player.expand_from(&space, x);
            self.player.and(&mask);

            space.expand_from(&self.player, x);
            space.and(&mask);
        }

        self.player = space;
    }

    /// Number of targets left.
    pub fn win(&self, board: &Board) -> bool {
        self.boxes == board.targets
    }

    /// Gives the possible next moves according to board. The position must be expanded.
    pub fn moves(&self, board: &Board) -> Vec<Move> {

        // print_position(board, self);
        let (line_size, _) = board.size;
        let dirs: [i16; 4] = [-1, 1, -(line_size as i16), line_size as i16];

        let mut places = self.boxes.clone();
        places.neg();
        places.and(&board.places);

        (board.size.0 .. board.size.0 * (board.size.1 - 1))
            .filter(|i| self.boxes.get_bit(*i as usize))
            .flat_map(|pos| repeat(pos as i16).zip(dirs.iter()))
            .filter(|&(pos, dir)| self.player.get_bit((pos - dir) as usize))  // player can reach
            .flat_map(|(pos, dir)| {
                repeat(pos).zip(repeat(dir).zip(1 as i16..))
                .map(|(a, (b, c))| (a, a + b * (c - 1),  a + b * c))
                .take_while(|&(_, _, to)| {
                    places.get_bit(to as usize)
                })
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
