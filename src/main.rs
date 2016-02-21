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
    let board = match env::args().skip(1).next() {
        Some(arg) => parse(&arg),
        _ => { panic!("Must have at least one arg") }
    };

    resolve(&board);
}

fn parse(filename: &str) -> Board {
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => panic!("no such file"),
    };
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .ok()
        .expect("failed to read!");
    let lines: Vec<String> = file_contents.split("\n")
        .map(|s: &str| s.to_string())
        .collect();

    return Board::new((lines[0].len(), lines.len()), lines);
}

fn resolve(board: &Board) {
    let mut visited = HashSet::<Position>::new();
    let mut queue = LinkedList::<Position>::new();
    queue.push_front(board.initial_position());

    let check = 100000;
    let mut counter = 0;
    let mut already_visited = 0;

    while !queue.is_empty() {
        let mut next_position: Position = queue.pop_back().unwrap();
        next_position.expand(board);
        counter += 1;
        if counter % check == 0 {
            println!("check at {}", counter);
            println!("There was {} collisions out of {}", already_visited, check);
            already_visited = 0;
            board::print_position(board, &next_position);
        }

        if visited.contains(&next_position) {
            already_visited += 1;
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
fn lvl1() {
    let board = parse("lvls/lvl1");
    resolve(&board);
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
