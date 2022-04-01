
use std::fmt::Debug;

use requests;
use requests::ToJson;
use json;
use str;
use crate::init::{Move, WHITE, BLACK};

static URL: &str = "http://tablebase.lichess.ovh/standard/mainline";


fn json_to_move(json_game: &json::JsonValue) -> Option<Move> {
    Some(Move::from_uci(&json_game["uci"].to_string()))
}
#[derive(Debug)]
pub struct QueryResult {
    pub dtz50: u32,
    pub mainline: Vec<Move>,
    pub winner: Option<bool>
}
impl QueryResult {
    fn get_decisive_result(dtz: u32, json_games: json::iterators::Members, winner: Option<bool>) -> Self {
        let mut moves: Vec<Move> = Vec::new();
        for json_game in json_games {
            moves.push(json_to_move(json_game).unwrap());
        }
        QueryResult{
            dtz50: dtz,
            mainline: moves,
            winner: winner
        }
    }
}
pub fn make_api_call(fen: &str) -> Option<QueryResult>{
    let response = match requests::get(
        format!("{}?fen={}", URL, str::replace(fen, " ", "_"))
    ) {
        Ok(r) => {r},
        Err(e) => {println!("bad request: {:?}", e); return None;}
    };
    let json = match response.json() {
        Ok(r) => {r},
        Err(e) => {println!("response: ({})", 
            std::str::from_utf8(response.content())
                .expect("error while converting Vec<u8> to string")); return None;}
    };
    match json["winner"].as_str() {
        Some(win) => {
            match win {
                "w" => Some(QueryResult::get_decisive_result(json["dtz"].as_u32().unwrap(),
                 json["mainline"].members(), Some(WHITE))),
                "b" => Some(QueryResult::get_decisive_result(json["dtz"].as_u32().unwrap(),
                 json["mainline"].members(), Some(BLACK))),
                _ => None,
            }
        },
        None => Some(QueryResult::get_decisive_result(json["dtz"].as_u32().unwrap(),
        json["mainline"].members(), None)),
    }
}