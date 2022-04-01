use std::{io::{Write, Read}, process::{Child, ChildStdin, ChildStdout, Command, Stdio}, cell::RefCell};

use requests::Result;

use crate::init::Move;

pub struct Engine {
    process: RefCell<Child>,
}
impl Engine {
    pub fn new(engine_path: &str) -> Engine{
        let process = Command::new(engine_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to connect to the engine");
        Engine {
            process: RefCell::new(process)
        }
    }
    pub fn write(&self, cmd: &str) {
        self.process.borrow_mut().stdin.as_mut().unwrap().write_all(format!("{}\n" , cmd).as_bytes()).expect("failed to write");
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    pub fn read_line(&mut self) -> Option<String>{
        let mut retval = String::new();
        let mut buffer: [u8;1] = [0];

        while buffer[0] != '\n' as u8 {
            let res = self.process.borrow_mut().stdout.as_mut().unwrap().read(&mut buffer);
            if res.is_err() {
                return None;
            }
            retval.push(buffer[0] as char);
        }
        Some(retval)

    }
    pub fn start(&mut self) {
        self.write("uci");
        self.read_line_starts_with("uciok");
        self.write("isready");
        self.read_line_starts_with("readyok");
    }
    pub fn read_line_starts_with(&mut self, phrase: &str) -> String {
        loop {
            match self.read_line() {
                Some(s) => {
                    if s.starts_with(phrase) {
                        return s;
                    }
                },
                _ => {panic!("Couldn't get the best move")}
            }
        }
    }
    pub fn set_position(&mut self, fen: &str) {
        if fen == crate::init::STARTING_FEN {
            self.write("position startpos");
        }
        self.write(&format!("position fen {}", fen));
    }
    pub fn set_option(&self, name: &str, value: &str) {
        self.write(&format!("setoption name {} value {}", name, value));
    }
    pub fn get_best_move(&mut self, time_ms: usize) -> Move {
        self.write(&format!("go movetime {}", time_ms));
        let line =  self.read_line_starts_with("bestmove");
        let best_move = line.split(" ").nth(1).unwrap().to_owned();
        return crate::init::Move::from_uci(&best_move);
    } 
}