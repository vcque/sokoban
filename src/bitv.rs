use bit_vec::BitVec;

pub fn print(to_print: &BitVec, line_size: usize) {
    let mut i = 0;
    for bit in to_print.iter() {
        if i % line_size == 0 { print!("\n"); }
        if bit { print!("."); } else { print!("#"); };
        i += 1;
    }
}

/// Will do for now
pub fn expand_from(from: &BitVec, to_expand: &mut BitVec, x: usize) {

    let block = from.storage()[0];
    unsafe {
        to_expand.storage_mut()[0] =
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
            to_expand.storage_mut()[i] =
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
        to_expand.storage_mut()[last] =
            block
            | (block >> 1)
            | (block << 1) | (from.storage()[last - 1] >> 31)
            | (block >> x)
            | (block << x) | (from.storage()[last - 1] >> (32 - x))
            ;
    }
}

/// Will do for now
pub fn intersect(first: &BitVec, second: &BitVec) -> bool {
    first.storage().iter()
        .zip(second.storage())
        .any(|(a, b)| a & b != 0)
}

#[test]
fn test_expand() {
    let from = BitVec::from_fn(100, |i| { i == 30 || i == 56 });
    let mut to: BitVec = BitVec::from_elem(100, true);

    expand_from(&from, &mut to, 10);
    print(&from, 10);
    println!("");
    print(&to, 10);

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
