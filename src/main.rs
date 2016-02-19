extern crate bit_vec;
extern crate time;

use std::collections::{HashSet, LinkedList};
use std::fs::File;
use std::env;

use std::io::Read;

use board::Board;
use position::Position;

mod board;
mod position;
mod bitv;

pub fn main() {
    for argument in env::args() {
        println!("arg: {}", argument);
    }
    let board = match env::args().take(1).next() {
        Some(arg) => match File::open(arg) {
            Ok(file) => { parse(file) },
            _ => { panic!("Could not open file"); }

        },
        _ => { panic!("Must have at least one arg") }
    };

    resolve(&board);
}

fn parse(mut file: File) -> Board {
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut data = vec![];


    let mut lines = String::new();
    let _ = file.read_to_string(&mut lines);
    for line in lines.split('\n') {
        x = std::cmp::max(x, line.len());
        y += 1;
        data.push(line.clone().to_owned());
    };

    return Board::new((x, y), data);
}

fn resolve(board: &Board) {
    let mut visited = HashSet::<Position>::new();
    let mut queue = LinkedList::<Position>::new();
    queue.push_front(board.initial_position());

    let limit = 10000;
    let mut counter = 0;

    while !queue.is_empty() {
        counter += 1;
        if counter > limit {
            println!("wtf, over the LIMIT");
            return;
        }
        let mut next_position: Position = queue.pop_back().unwrap();
        next_position.expand(board);
        if visited.contains(&next_position) {
            continue;
        } else {
            for mov in next_position.moves(board) {
                let new_pos = next_position.move_to(&mov);
                if new_pos.win(board) {
                    println!("we won !");
                    println!("number of iterations : {}", counter);
                    println!("winning boxes position :");
                    bitv::print(&new_pos.boxes, board.size.0);
                    return;
                } else {
                    queue.push_front(new_pos);
                }
            }
            visited.insert(next_position);
        }
    }

}

#[test]
fn test() {
    let mut data = vec![];
    data.push("#########".to_owned());
    data.push("###  ####".to_owned());
    data.push("#     $ #".to_owned());
    data.push("# #  #$ #".to_owned());
    data.push("# . .#@ #".to_owned());
    data.push("#########".to_owned());

    let board = Board::new((9, 6), data);
    let start = time::now();
    resolve(&board);
    let end = time::now();
    println!("done in {}", (end - start).num_milliseconds());
}
