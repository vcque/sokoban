use bitv;
use bit_vec::BitVec;
use position::Position;

pub struct Board {
    /// Size of the board
    pub size: (usize, usize),
    /// Passable tiles
    pub floor: BitVec,
    /// Possible box positions
    pub places: BitVec,
    /// Target tiles
    pub targets: BitVec,
    /// Initial boxes position
    pub boxes: BitVec,
    /// Initial player position
    pub player: usize
}

impl Board {

    pub fn new(size: (usize, usize), data: Vec<String>) -> Board {
        let (x, y) = size;

        let mut player = 0;
        let mut nbr_box = 0;
        let mut nbr_target = 0;


        let mut floor = BitVec::from_elem(x * y, false);
        let mut targets = BitVec::from_elem(x * y, false);
        let mut boxes = BitVec::from_elem(x * y, false);

        for j in 0 .. y {
            let mut i = 0;
            for ch in data[j].chars() {
                let n = i + j * x;
                match ch {
                    ' ' => { // Floor
                        floor.set(n, true);
                    },
                    '#' => {}, // Wall
                    '$' => { // Box
                        boxes.set(n, true);
                        floor.set(n, true);
                        nbr_box += 1;
                    },
                    '.' => { // Target
                        targets.set(n, true);
                        floor.set(n, true);
                        nbr_target += 1;
                    },
                    '@' => { // Player
                        floor.set(n, true);
                        player = n;
                    },
                    '*' => { // Box on target
                        floor.set(n, true);
                        boxes.set(n, true);
                        targets.set(n, true);
                        nbr_target += 1;
                        nbr_box += 1;
                    },
                    '+' => { // Player on target
                        floor.set(n, true);
                        targets.set(n, true);
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

        let places = BitVec::from_fn(floor.len(), |i| {
            targets.get(i) == Some(true) || (
                floor.get(i) == Some(true)
                && (floor.get(i - 1) == Some(true) || floor.get(i - x) == Some(true))
                && (floor.get(i + 1) == Some(true) || floor.get(i - x) == Some(true))
                && (floor.get(i - 1) == Some(true) || floor.get(i + x) == Some(true))
                && (floor.get(i + 1) == Some(true) || floor.get(i + x) == Some(true))
            )
        });

        bitv::print(&places, x);

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
        let mut player_positions: BitVec = BitVec::from_elem(self.boxes.len(), false);
        player_positions.set(self.player, true);

        Position {
            boxes: self.boxes.clone(),
            player: player_positions
        }
    }

    pub fn to_square_coord(&self, pos: usize) -> (usize, usize) {
        let (x,_) = self.size;
        return (pos % x, pos / x);
    }

    pub fn to_linear_coord(&self, (i, j) : (usize, usize)) -> usize {
        let (x,_) = self.size;
        return i + x * j;
    }
}
