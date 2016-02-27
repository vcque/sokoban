use bit_vec::BitVec;

use position::Position;
use bitv::BitvImpl;
use bitv::Bitv;

pub struct Board {
    /// Passable tiles
    pub floor: BitvImpl,
    /// Possible box positions
    pub places: BitvImpl,
    /// Target tiles
    pub targets: BitvImpl,
    /// Initial boxes position
    pub boxes: BitvImpl,
    /// Size of the board
    pub size: (u8, u8),
    /// Initial player position
    pub player: usize
}

impl Board {

    pub fn new(size: (u8, u8), data: Vec<String>) -> Board {
        let x = size.0 as usize;
        let y = size.1 as usize;

        let mut player = 0;
        let mut nbr_box = 0;
        let mut nbr_target = 0;


        let mut floor: [u64; 4] = [0; 4];
        let mut targets: [u64; 4] = [0; 4];
        let mut boxes: [u64; 4] = [0; 4];

        for j in 0 .. y {
            let j = j as usize;
            let mut i = 0;
            for ch in data[j].chars() {
                let n = i + j * x as usize;
                match ch {
                    ' ' => { // Floor
                        floor.set_bit(n);
                    },
                    '#' => {}, // Wall
                    '$' => { // Box
                        boxes.set_bit(n);
                        floor.set_bit(n);
                        nbr_box += 1;
                    },
                    '.' => { // Target
                        targets.set_bit(n);
                        floor.set_bit(n);
                        nbr_target += 1;
                    },
                    '@' => { // Player
                        floor.set_bit(n);
                        player = n;
                    },
                    '*' => { // Box on target
                        floor.set_bit(n);
                        boxes.set_bit(n);
                        targets.set_bit(n);
                        nbr_target += 1;
                        nbr_box += 1;
                    },
                    '+' => { // Player on target
                        floor.set_bit(n);
                        targets.set_bit(n);
                        player = n;
                        nbr_target += 1;
                    },
                    c => {
                        panic!(format!("Wrong character : {}", c));
                    }
                }
                i += 1;
            }
        }

        assert!(nbr_box >= nbr_target, "Impossible game, there's more targets than boxes !");

        let places_bitvec = BitVec::from_fn(floor.len() * 64, |i| {
            targets.get_bit(i) || (
                floor.get_bit(i)
                && (floor.get_bit(i - 1) || floor.get_bit(i - x))
                && (floor.get_bit(i + 1) || floor.get_bit(i - x))
                && (floor.get_bit(i - 1) || floor.get_bit(i + x))
                && (floor.get_bit(i + 1) || floor.get_bit(i + x))
            )
        });

        let mut places = [0 as u64; 4];
        for (i, bit) in places_bitvec.iter().enumerate() {
            if bit { places.set_bit(i); }
        }

        Board {
            size: size,
            places: places,
            floor: floor,
            targets: targets,
            boxes: boxes,
            player: player
        }
    }

    pub fn initial_position(&self) -> Position {
        let mut player_positions: BitvImpl = [0; 4];
        player_positions.set_bit(self.player);

        Position {
            boxes: self.boxes.clone(),
            player: player_positions
        }
    }
}

pub fn print_position(board: &Board, pos: &Position) {
    for i in 0 .. board.size.0 * board.size.1 {
        if i % board.size.0 == 0 { print!("\n"); }
        let i = i as usize;
        if !board.floor.get_bit(i) { print!("#"); }
        else if pos.player.get_bit(i) {
            if board.targets.get_bit(i) { print!("+"); }
            else { print!("@"); }
        } else if pos.boxes.get_bit(i) {
            if board.targets.get_bit(i) { print!("*"); }
            else { print!("$"); }
        } else if board.targets.get_bit(i) { print!("."); }
         else if !board.places.get_bit(i) { print!("-"); }
        else { print!(" "); }
    }
    print!("\n");
}
