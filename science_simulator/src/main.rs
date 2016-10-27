extern crate git2;
extern crate rustc_serialize;

mod event;
mod evaluator;

use std::io::Read;
use rustc_serialize::json;
use event::Event;
use evaluator::Evaluator;

fn main() {
    let mut json_str = String::new();
    std::fs::File::open("script.json").unwrap().read_to_string(&mut json_str);
    let events: Vec<Event> = json::decode(&json_str).unwrap();

    let mut evaluator = Evaluator::new("/home/mike/Documents/Innopolis/Philosophy/report/science_repo".to_owned());
    evaluator.evaluate(events.into_iter());
}
