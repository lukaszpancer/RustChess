#![allow(non_snake_case)]
#![feature(const_for)]
#![feature(core_intrinsics)]
#![feature(generators)]
#![feature(generator_trait)]
#![feature(type_alias_impl_trait)]
#![feature(test)]
#![allow(dead_code)]
#![macro_use]
use core::panic;
use std::{fs::read_to_string, thread, cell::RefCell, rc::Rc};
extern crate lazy_static;
extern crate auto_ops;
mod init;
mod pgn;
mod gen_iter;
mod engine;
mod syzygy;
use ahash::AHashMap;
use engine::Engine;
use lazy_static::lazy_static;
use pgn::GameBuilder;

use crate::init::{WHITE, Board, BB_ALL, Move, STARTING_BOARD_FEN, STARTING_FEN};
use rand::{thread_rng, prelude::SliceRandom};

pub static mut I: u64 = 0;

lazy_static! {
    pub static ref FILE: String = {
        let filename = "short_lichess.pgn";
        match read_to_string(filename) {
            Ok(f) => match f.parse() {
                Ok(f2) => f2,
                Err(e) => {panic!("Error: {:?}", e)}
            },
            Err(e) => { panic!("Erorr: {:?}", e);}
        }

    };
    pub static ref GAMES: Vec<&'static str> = {

        let split_on = "\r\n\r\n[";
        // let split_on = "\n\n[";
        let mut rng = thread_rng();
        let mut file = FILE.split(split_on).collect::<Vec<&str>>();
        file.shuffle(&mut rng);

        file
    };
}
fn main() {
        let no_threads = 1;
        let no_games = GAMES.len();
        let games_per_thread = no_games / no_threads;
        println!("n: {}, n/t: {}", no_games, games_per_thread);
        let mut threads = Vec::new();
        for i in 0..no_threads as usize {
            let slice = &GAMES[(i * games_per_thread)..((i + 1) * games_per_thread)];
            let handle = thread::spawn(move || {
                let a  = pgn::read_game_async(slice);
                let mut b: AHashMap<usize, f64> = AHashMap::new();
                for k in a.keys() {
                    if a.get(k).unwrap().iter().count() != 0 {
                        b.insert(*k, a.get(k).unwrap().iter().sum::<u64>() as f64 / a.get(k).unwrap().iter().count() as f64);
                    }
                    else {
                        // b.insert(*k,0f64);
                    }
                }
                for i in 0..201 {
                    println!("{}", b.get(&(i as usize)).unwrap());
                }
                println!("average: {:?}", b.values().sum::<f64>() / b.values().count() as f64);
            });
            threads.push(handle);
        }
        for thread in threads {
            let _r = thread.join();
        }

        // let mut game = Board::new(Some(STARTING_FEN));
        // game.push(game.parse_san("e4"));
        // game.push(game.parse_san("e5"));
        // game.push(game.parse_san("Qh5"));
        // game.push(game.parse_san("Nc6"));
        // game.push(game.parse_san("Bc4"));
        // game.push(game.parse_san("Nf6"));
        // game.push(game.parse_san("Qxf7"));
        // println!("is it checkmate?: {}", game.is_checkmate());
        // println!("{}", game.baseboard.unicode(!game.turn, false, "."));
}

extern crate test;