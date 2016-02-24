extern crate bit_vec;
extern crate time;

use std::collections::{HashMap, LinkedList};
use std::env;
use std::fs::File;
use std::io::Read;

use bit_vec::BitVec;

use bitv::intersect;
use board::{Board, print_position};
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

fn resolve(board: &Board) -> Option<Vec<Position>> {
    let mut position_index = 1;
    let mut visited = HashMap::<Vec<u32>, BitVec>::with_capacity(1500000);
    let mut position_by_id = HashMap::<u32, (u32, Position)>::with_capacity(1500000);
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

            if board.targets.get(mov.to as usize) == Some(true) && new_pos.win(board) {
                println!("we won !");
                println!("number of visited positions : {}", position_index);
                let mut result = vec![new_pos, next_position];
                result.append(&mut recover_parents(&mut position_by_id, parent_index));
                return Some(result);
            }

            #[derive(PartialEq, Eq, Debug)]
            enum Res { Found, BoxNotFound, PlayerNotFound };

            let need_change = match visited.get_mut(new_pos.boxes.storage()) {
                Some(player) => {

                    if !intersect(&player, &new_pos.player) {
                        new_pos.expand(&board);
                        player.union(&new_pos.player);
                        Res::PlayerNotFound
                    } else {
                        Res::Found
                    }
                }
                None => {
                    new_pos.expand(&board);
                    Res::BoxNotFound
                }
            };

            if need_change != Res::Found {
                if need_change == Res::BoxNotFound {
                    visited.insert(new_pos.boxes.storage().to_vec(), new_pos.player.clone());
                }
                queue.push_front((position_index, new_pos));
            }
        }

        position_by_id.insert(position_index, (parent_index, next_position));
    }
    None
}

fn recover_parents(map: &mut HashMap<u32, (u32, Position)>, first_parent_id: u32) -> Vec<Position> {
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
