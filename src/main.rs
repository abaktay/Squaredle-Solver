mod solve;
mod trie;

use crate::solve::*;
use crate::trie::*;

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
        .map(|row| {
            row.trim()
                .trim_matches(|c| c == '"' || c == '\\')
                .bytes()
                .collect::<Vec<u8>>()
        })
        .filter(|row| !row.is_empty())
        .collect();
    board
}

fn main() {
    let body: String = ureq::get("https://squaredle.app/api/today-puzzle-config.js")
        .call()
        .unwrap()
        .body_mut()
        .read_to_string()
        .unwrap();

    let mut root = TrieNode::new();
    let board = extract_board(body);
    root.load_file("word_list.txt");

    let found = solve(&board, &root);
    println!("Found: {:?}", found);
}
