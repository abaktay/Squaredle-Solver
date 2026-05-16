mod solve;
mod trie;

use crate::solve::*;
use crate::trie::*;

use std::env;
use std::io;

fn extract_board(js: String) -> Vec<Vec<u8>> {
    let start = js.find("gTodayDateStr = '").unwrap() + "gTodayDateStr = '".len();
    let end = js[start..].find('\'').unwrap() + start;
    let date = &js[start..end];

    let date_js = date.replace("/", "\\/") + "\""; // to avoid fetching xp edition
    // println!("{}", date_js);
    let date_pos = js.find(&date_js).unwrap();
    let board_pos = js[date_pos..].find("\"board\"").unwrap() + date_pos;

    let arr_start = js[board_pos..].find('[').unwrap() + board_pos;
    let arr_end = js[arr_start..].find(']').unwrap() + arr_start;

    let board: Vec<Vec<u8>> = js[arr_start + 1..arr_end]
        .split(',')
        .filter_map(|row| {
            let row = row
                .trim()
                .strip_prefix('"')
                .and_then(|s| s.strip_suffix('"'))?;
            Some(row.bytes().collect())
        })
        .collect();
    board
}

// denote empty cells with #
// inputs: daily, manual

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut board: Vec<Vec<u8>> = vec![];

    if args[1] == "daily" {
        let body: String = ureq::get("https://squaredle.app/api/today-puzzle-config.js")
            .call()
            .unwrap()
            .body_mut()
            .read_to_string()
            .unwrap();
        board = extract_board(body);
    } else if args[1] == "manual" {
        println!("Enter the board. Use # to denote empty cells. Enter a newline when you're done.");
        let mut input = String::new();

        loop {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();

            if input.trim().is_empty() {
                break;
            }
            input = input.trim().to_lowercase().replace("#", " ");
            board.push(input.bytes().collect());
        }
    } else {
        eprintln!("Unknown argument: {}", args[1]);
        return;
    }

    let mut root = TrieNode::new();
    root.load_file("word_list.txt");

    let found = solve(&board, &root);
    println!("Found: {:?}", found);
}
