use std::collections::BTreeMap;
use std::io::{self, BufRead};

use serde::Serialize;

#[derive(Serialize)]
struct Launch {
    env: BTreeMap<String, String>,
}

fn main() {
    let stdin = io::stdin();
    let mut launch = Launch {
        env: BTreeMap::new(),
    };

    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        if line.len() > 0 {
            let line = eat_char(&line, ' ');
            if is_comment(&line) {
                continue;
            }

            // skip 'export'
            let mut toks = line.splitn(2, " ");
            if Some("export") != toks.next() {
                continue;
            }
            let toks = toks.next().unwrap();

            // get var name and var val
            let mut toks = toks.splitn(2, "=");
            let key = toks.next().unwrap();

            // trim leading/trailing double quotes
            let val = toks.next().unwrap();
            let val = val.trim_start_matches('"').trim_end_matches('"');

            launch.env.insert(key.to_owned(), val.to_owned());
        }
    }

    println!("{}", serde_json::to_string_pretty(&launch).unwrap());
}

fn eat_char(inp: &str, ch: char) -> &str {
    inp.trim_start_matches(ch)
}

fn is_comment(inp: &str) -> bool {
    // if line begins with # then its a comment
    if inp.chars().next().filter(|c| *c == '#').is_some() {
        true
    } else {
        false
    }
}
