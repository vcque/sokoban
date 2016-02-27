use bit_vec::BitVec;

pub type BitvImpl = [u64; 4];
/// Method needed for resolving a board
pub trait Bitv {

    fn set_bit(&mut self, i: usize);

    fn unset_bit(&mut self, i: usize);

    fn get_bit(&self, i: usize) -> bool;

    fn intersect(&self, second: &Self) -> bool;

    fn expand_from(&mut self, second: &Self, x: usize);

    fn or(&mut self, second: &Self);

    fn and(&mut self, second: &Self);

    fn neg(&mut self);

    fn print(&self, x: usize);
}

impl Bitv for BitVec {

    fn set_bit(&mut self, i: usize) {
        self.set(i, true);
    }

    fn unset_bit(&mut self, i: usize) {
        self.set(i, true);
    }

    fn get_bit(&self, i: usize) -> bool{
        self.get(i) == Some(true)
    }

    fn intersect(&self, second: &Self) -> bool {
        self.storage().iter()
            .zip(second.storage())
            .any(|(a, b)| a & b != 0)
    }

    fn expand_from(&mut self, from: &Self, x: usize) {
        let block = from.storage()[0];
        unsafe {
            self.storage_mut()[0] =
                block
                | (block >> 1) | (from.storage()[1] << 31)
                | (block << 1)
                | (block >> x) | (from.storage()[1] << (32 - x))
                | (block << x)
                ;
        }

        for (i, block) in from.storage()
            .iter()
            .enumerate()
            .skip(1)
            .take(from.storage().len() - 2) {
            unsafe {
                self.storage_mut()[i] =
                    block
                    | (block >> 1) | (from.storage()[i + 1] << 31)
                    | (block << 1) | (from.storage()[i - 1] >> 31)
                    | (block >> x) | (from.storage()[i + 1] << (32 - x))
                    | (block << x) | (from.storage()[i - 1] >> (32 - x))
                    ;
            }
        }

        let last = from.storage().len() - 1;
        let block = from.storage()[last];
        unsafe {
            self.storage_mut()[last] =
                block
                | (block >> 1)
                | (block << 1) | (from.storage()[last - 1] >> 31)
                | (block >> x)
                | (block << x) | (from.storage()[last - 1] >> (32 - x))
                ;
        }
    }

    fn neg(&mut self) {

    }

    fn or(&mut self, _: &Self) {

    }

    fn and(&mut self, _: &Self) {}

    fn print(&self, line_size: usize) {
        let mut i = 0;
        for bit in self.iter() {
            if i % line_size == 0 { print!("\n"); }
            if bit { print!("."); } else { print!("#"); };
            i += 1;
        }
    }
}

impl Bitv for [u64; 4] {

    fn set_bit(&mut self, i: usize) {
        let w = i / 64;
        let b = i % 64;
        self[w] = self[w] | (1 << b);
    }

    fn unset_bit(&mut self, i: usize) {
        let w = i / 64;
        let b = i % 64;
        self[w] = self[w] & !(1 << b);
    }

    fn get_bit(&self, i: usize) -> bool {
        let w = i / 64;
        let b = i % 64;
        self[w] & (1 << b) != 0
    }

    fn intersect(&self, second: &Self) -> bool {
        let mut acc = 0;
        for i in 0..self.len() {
            acc += self[i] & second[i];
        }
        acc != 0
    }

    fn expand_from(&mut self, from: &Self, x: usize) {
        let block = from[0];
        self[0] =
            block
            | (block >> 1) | (from[1] << 63)
            | (block << 1)
            | (block >> x) | (from[1] << (64 - x))
            | (block << x)
            ;


        for (i, block) in from
            .iter()
            .enumerate()
            .skip(1)
            .take(from.len() - 2) {
            self[i] =
                block
                | (block >> 1) | (from[i + 1] << 63)
                | (block << 1) | (from[i - 1] >> 63)
                | (block >> x) | (from[i + 1] << (64 - x))
                | (block << x) | (from[i - 1] >> (64 - x))
                ;

        }

        let last = from.len() - 1;
        let block = from[last];
        self[last] =
            block
            | (block >> 1)
            | (block << 1) | (from[last - 1] >> 63)
            | (block >> x)
            | (block << x) | (from[last - 1] >> (64 - x))
            ;
    }

    fn or(&mut self, second: &Self) {
        for i in 0..self.len() {
            self[i] = self[i] | second[i];
        }
    }

    fn and(&mut self, second: &Self) {
        for i in 0..self.len() {
            self[i] = self[i] & second[i];
        }
    }


    fn neg(&mut self) {
        for i in 0..self.len() {
            self[i] = !self[i];
        }
    }

    fn print(&self, x: usize) {
        for i in 0.. self.len() * 64 {
            if i % x == 0 { print!("\n"); }
            if self.get_bit(i) { print!("."); } else { print!("#"); };
        }
    }
}

#[test]
fn test_expand() {
    let from = BitVec::from_fn(100, |i| { i == 30 || i == 56 });
    let mut to: BitVec = BitVec::from_elem(100, true);

    to.expand_from(&from, 10);

    assert_eq!(to[30], true);
    assert_eq!(to[29], true);
    assert_eq!(to[31], true);
    assert_eq!(to[20], true);
    assert_eq!(to[40], true);

    assert_eq!(to[56], true);
    assert_eq!(to[57], true);
    assert_eq!(to[55], true);
    assert_eq!(to[66], true);
    assert_eq!(to[46], true);
}
