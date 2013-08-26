use std::ops::{BitAnd, BitOr, Index, Not, Shl, Shr};
use std::to_bytes::{Cb, IterBytes};
use std::uint;
use std::vec;

#[deriving(Clone, Eq)]
struct Bitv {
    length: uint,
    store: ~[uint]
}

impl Bitv {

    /// Default constructor.
    pub fn new(len: uint) -> Bitv {
        let bits = uint::bits;
        let nelem = len / bits + if len % bits > 0 {1} else {0};
        return Bitv{length: len, store: vec::from_elem(nelem, 0) };
    }
    
    pub fn set(&mut self, pos: uint, value: bool) {
        let index = pos / uint::bits;
        let bit = pos % uint::bits;
        let mask = 1 << bit;
        
        self.store[index] = 
            if (value) { self.store[index] | mask }
            else {self.store[index] & !mask };
    }
    
    pub fn get(&self, pos: uint) -> bool {
        let index = pos / uint::bits;
        let bit = pos % uint::bits;
        let mask = 1 << bit;
        
        self.store[index] & mask != 0
    }
    
    pub fn union(&self, other: &Bitv) -> Bitv {
        assert_eq!(self.length, other.length);
        let mut result = Bitv::new(self.length);
        for uint::range(0, self.store.len()) |i| {
            result.store[i] = self.store[i] | other.store[i];
        }
        result.mask();
        return result;
    }
    
    pub fn intersect(&self, other: &Bitv) -> Bitv {
        assert_eq!(self.length, other.length);
        let mut result = Bitv::new(self.length);
        for uint::range(0, self.store.len()) |i| {
            result.store[i] = self.store[i] & other.store[i];
        }
        result.mask();
        return result;
    }
    
    pub fn invert(&self) -> Bitv {
        let mut result = Bitv::new(self.length);
        for uint::range(0, self.store.len()) |i| {
            result.store[i] = !self.store[i];
        }
        result.mask();
        return result;
    }
    
    pub fn shift(&self, shift: uint) -> Bitv {
        let mut result = Bitv::new(self.length);
        let index = shift / uint::bits;
        let bits = shift % uint::bits;
        let length = self.store.len();
        for uint::range(0, length) |i| {
            let first = 
                if (i >= index) { self.store[i-index] }
                else {0};
            let second = 
                if (i >= index + 1) { self.store[i-index-1] }
                else {0};
            
            result.store[i] = first << bits | second >> uint::bits - bits;
        }
        
        result.mask();
        return result;
    }
    
    pub fn shift_back(&self, shift: uint) -> Bitv {
        let mut result = Bitv::new(self.length);
        let index = shift / uint::bits;
        let bits = shift % uint::bits;
        let length = self.store.len();
        for uint::range(0, length) |i| {
            let first = 
                if (i + index < length) { self.store[i + index] }
                else {0};
            let second = 
                if (i + index + 1 < length) { self.store[i + index + 1] }
                else {0};
            
            result.store[i] = first >> bits | second << uint::bits - bits;
        }
        
        result.mask();
        return result;
    }
    
    pub fn assign(&mut self, other: &Bitv) {
        assert_eq!(self.length, other.length);
        self.store = other.store.clone();
    }
    
    pub fn assign_invert(&mut self, other: &Bitv) {
        assert_eq!(self.length, other.length);
        for uint::range(0, self.store.len()) |i| {
            self.store[i] = !other.store[i];
        }
        self.mask();
    }
    
    pub fn assign_union(&mut self, other: &Bitv) {
        assert_eq!(self.length, other.length);
        for uint::range(0, self.store.len()) |i| {
            self.store[i] = self.store[i] | other.store[i];
        }
        self.mask();
    }
    
    pub fn assign_intersect(&mut self, other: &Bitv) {
        assert_eq!(self.length, other.length);
        for uint::range(0, self.store.len()) |i| {
            self.store[i] = self.store[i] & other.store[i];
        }
        self.mask();
    }
    
    pub fn assign_shift(&mut self, shift: uint) {
        let index = shift / uint::bits;
        let bits = shift % uint::bits;
        let length = self.store.len();
        for uint::range_rev(length, 0) |i| {
            let first = 
                if (i >= index + 1) { self.store[i-1-index] }
                else {0};
            let second = 
                if (i >= index + 2) { self.store[i-2-index] }
                else {0};
            
            self.store[i-1] = first << bits | second >> uint::bits - bits;
        }
        self.mask();
    }
    
    pub fn assign_shift_back(&mut self, shift: uint) {
        let index = shift / uint::bits;
        let bits = shift % uint::bits;
        let length = self.store.len();
        for uint::range(0, length) |i| {
            let first = 
                if (i + index < length) { self.store[i + index] }
                else {0};
            let second = 
                if (i + index + 1 < length) { self.store[i + 1 + index] }
                else {0};
            
            self.store[i] = first >> bits | second << uint::bits - bits;
        }
        self.mask();
    }
    
    /// Used for keeping bitv consistent with eq
    fn mask(&mut self) {
        let s = uint::bits;
        let mask = !0 >> (s - self.length % s);
        let index = self.store.len() - 1;
        self.store[index] = self.store[index] & mask;
    }
}

impl IterBytes for Bitv {
    fn iter_bytes(&self, lsb0: bool, f: Cb) -> bool {
        self.store.iter_bytes(lsb0, f)
    }
}

impl Not<Bitv> for Bitv {
    fn not(&self) -> Bitv {
        return self.invert();
    }
}

impl BitAnd<Bitv, Bitv> for Bitv {
    fn bitand(&self, other: &Bitv) -> Bitv {
        return self.intersect(other);
    }
}

impl BitOr<Bitv, Bitv> for Bitv {
    fn bitor(&self, other: &Bitv) -> Bitv {
        return self.union(other);
    }
}

impl Shl<uint, Bitv> for Bitv {
    fn shl(&self, shift: &uint) -> Bitv {
        return self.shift(*shift);
    }
}

impl Shr<uint, Bitv> for Bitv {
    fn shr(&self, shift: &uint) -> Bitv {
        return self.shift_back(*shift);
    }
}

impl Index<uint, bool> for Bitv {
    fn index(&self, index: &uint) -> bool {
        return self.get(*index);
    }
}

#[test]
fn test_intersect() {
    let mut a = Bitv::new(64);
    let mut b = Bitv::new(64);

    for uint::range(0, 64) |i| {
        a.set(i, i % 2 == 0);
        b.set(i, i % 2 != 0);
    }

    assert_eq!(a.intersect(&b), Bitv::new(64));
    assert_eq!(a.union(&b), Bitv::new(64).invert());
}

#[test]
fn test_invert() {
    let mut a = Bitv::new(63);

    for uint::range(16, 56) |i| {
        a.set(i, i % 2 == 0);
    }
    
    assert_eq!(&a, &a.invert().invert());
    assert_eq!(Bitv::new(64), Bitv::new(64).invert().invert());
}

#[test]
fn test_shift() {
    let mut a = Bitv::new(1000);
    let mut b = Bitv::new(1000);
    let mut c = Bitv::new(1000);
    
    a.set(128, true);
    b.set(257, true);
    c.set(130, true);
    
    assert_eq!(&a.shift(129), &b);
    assert_eq!(&a, &b.shift_back(129));
    assert_eq!(&a, &a.shift(129).shift_back(129));
    &a.assign_shift(129);
    assert_eq!(&a, &b);
    &b.assign_shift_back(127);
    assert_eq!(&c, &b);
}
