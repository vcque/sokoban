extern crate bit_vec;
extern crate time;
extern crate fnv;

use std::collections::{HashMap, LinkedList};
use std::collections::hash_map::Entry::*;
use std::env;
use std::fs::File;
use std::io::Read;
use std::hash::BuildHasherDefault;

use fnv::FnvHasher;

use bitv::Bitv;
use bitv::BitvImpl;
use board::{Board, print_position};
use position::Position;


mod board;
mod position;
mod bitv;

type FnvHash = BuildHasherDefault<FnvHasher>;

pub fn main() {
    for argument in env::args() {
        println!("arg: {}", argument);
    }
    let board = match env::args().skip(1).next() {
        Some(arg) => parse(&arg),
        _ => { panic!("Must have at least one arg") }
    };

    let start = time::now();
    let result = resolve(&board);
    let end = time::now();

    match result {
        Some(positions) => {
            for position in positions.iter().rev() {
                println!("");
                print_position(&board, position);
            }
            println!("A solution was found.");
            println!("Made in {} steps.", positions.len());
        },
        None => println!("Could not find any solution.")
    }
    println!("done in {}", (end - start).num_milliseconds());
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

    return Board::new((lines[0].len() as u8, lines.len() as u8), lines);
}

fn resolve(board: &Board) -> Option<Vec<Position>> {
    let mut position_index = 1;
    let mut visited = HashMap::<BitvImpl, BitvImpl, FnvHash>::default();
    let mut position_by_id = HashMap::<u32, (u32, Position), FnvHash>::default();
    visited.reserve(2500000);
    position_by_id.reserve(2500000);

    let mut queue = LinkedList::<(u32, Position)>::new();

    let mut initial_position = board.initial_position();
    initial_position.expand(board);
    queue.push_front((0, initial_position));

    let mut time_at_step = time::now();

    while !queue.is_empty() {
        let (parent_index, next_position) = queue.pop_back().unwrap();
        position_index += 1;

        if position_index % 100000 == 0 {
            let time = time::now();
            println!("{} steps done in {}", 100000, (time_at_step - time).num_milliseconds());
            time_at_step = time;
        }

        for mov in next_position.moves(board) {
            let mut new_pos = next_position.move_to(&mov);

            if board.targets.get_bit(mov.to as usize) && new_pos.win(board) {
                println!("size of HashMap: {}, {}", visited.capacity(), position_by_id.capacity());
                println!("we won !");
                println!("number of visited positions : {}", position_index);
                let mut result = vec![new_pos, next_position];
                result.append(&mut recover_parents(&mut position_by_id, parent_index));
                return Some(result);
            }

            match visited.entry(new_pos.boxes) {
                Occupied(mut entry) => {
                    let player = entry.get_mut();
                    if !player.intersect(&new_pos.player) {
                        new_pos.expand(&board);
                        player.or(&new_pos.player);
                        queue.push_front((position_index, new_pos));
                    }
                },
                Vacant(entry) => {
                    new_pos.expand(&board);
                    entry.insert(new_pos.player);
                    queue.push_front((position_index, new_pos));
                }
            }
        }

        position_by_id.insert(position_index, (parent_index, next_position));
    }
    None
}

fn recover_parents(map: &mut HashMap<u32, (u32, Position), FnvHash>, first_parent_id: u32) -> Vec<Position> {
    let mut current_index = first_parent_id;
    let mut result: Vec<Position> = vec![];
    while let Some((p_i, parent_pos)) = map.remove(&current_index) {
        current_index = p_i;
        result.push(parent_pos);
    }
    result
}

#[test]
fn lvl1() {
    use board::print_position;

    let board = parse("lvls/lvl1");
    println!("Initial position:");
    print_position(&board, &board.initial_position());

    let start = time::now();
    let result = resolve(&board);
    let end = time::now();

    println!("done in {}", (end - start).num_milliseconds());
    match result {
        Some(positions) => {
            println!("A solution was found.");
            println!("Made in {} steps.", positions.len());
            for position in positions.iter().rev() {
                println!("");
                print_position(&board, position);
            }
            println!("Made in {} steps.", positions.len());
        },
        None => println!("Could not find any solution.")
    }
}

#[test]
fn simple_test() {
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
