use bit_vec::BitVec;

/// Will do for now
pub fn expand(to_expand: &BitVec, line_size: usize) -> BitVec {
    BitVec::from_fn(to_expand.len(), |i| {
        to_expand.get(i) == Some(true)
        || (i >= 1 && to_expand.get(i - 1) == Some(true))
        || to_expand.get(i + 1) == Some(true)
        || (i >= line_size && to_expand.get(i - line_size) == Some(true))
        || to_expand.get(i + line_size) == Some(true)
    })
}

pub fn print(to_print: &BitVec, line_size: usize) {
    let mut i = 0;
    for bit in to_print.iter() {
        if i % line_size == 0 { print!("\n"); }
        if bit { print!("."); } else { print!("#"); };
        i += 1;
    }
}
