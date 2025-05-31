use core::fmt;
use std::{fs};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    pub suit: String,
    pub value: String,
    pub is_face: bool,
}

impl fmt::Display for Card {
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
        if ["♦".to_string(), "♥".to_string()].contains(&self.suit) {
            write!(f, "\x1B[41m\x1B[97m{}{}\x1B[49m", self.suit, self.value)
        } else if ["★".to_string(), "".to_string()].contains(&self.suit) {
            write!(f, "\x1B[43m\x1B[97m{}{}\x1B[49m", self.suit, self.value)
        } else {
            write!(f, "\x1B[40m\x1B[97m{}{}\x1B[49m", self.suit, self.value)
        }
    }
}

impl Card {
    pub fn new(suit: &str, value: &str, is_face: bool) -> Self {
        return Self {
            suit: suit.to_string(),
            value: value.to_string(),
            is_face,
        }
    }

    pub fn read(path: &str) -> Self {
        let file = fs::File::open(path).expect("Wrong File");
        let json: serde_json::Value = serde_json::from_reader(file).expect("Not JSON file");
        let trim_match = ['\"', '\''];
        let suit = json.get("suit").unwrap().to_string().trim_start_matches(&trim_match).trim_end_matches(&trim_match).to_string();
        let value = json.get("value").unwrap().to_string().trim_start_matches(&trim_match).trim_end_matches(&trim_match).to_string();
        let is_face = json.get("is_face").unwrap().as_bool().unwrap();
        return Self {
            suit,
            value,
            is_face,
        }
    }
    pub fn list_mods(path: &str) -> Vec<String> {
        let paths = fs::read_dir(path).unwrap();
        let mut list = vec!();
        for p in paths {
            list.push(p.unwrap().path().display().to_string());
        }
        return list;
    }

    pub fn read_all_from_path(path: &str) -> Vec<Card> {
        let paths = fs::read_dir(path).unwrap();
        let mut list = vec!();

        for p in paths {
            list.push(Self::read(&p.unwrap().path().display().to_string()));
        }

        return list;
    }

    pub fn load_all(path: &str) -> Vec<Card> {
        let mods = Self::list_mods(path);
        let mut vec = vec!();
        for m in mods {
            println!("Loading {}", m);
            let read = Self::read_all_from_path(&m);
            for r in read {
                vec.push(r);
            }
        }
        return vec;
    }
}
