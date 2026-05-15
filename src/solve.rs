use crate::trie::*;

fn dfs(
    board: &Vec<Vec<u8>>,
    visited: &mut Vec<Vec<bool>>,
    node: &TrieNode,
    current_word: &mut String,
    found: &mut Vec<String>,
    row: usize,
    col: usize,
) {
    let c = board[row][col];
    if c == b' ' {
        return;
    }
    let i = (c as u8 - b'a') as usize;

    let next_node = match &node.children[i] {
        Some(n) => n,
        None => return,
    };

    current_word.push(c as char);
    visited[row][col] = true;

    if next_node.is_word {
        found.push(current_word.clone());
    }

    for dr in [-1i32, 0, 1] {
        for dc in [-1i32, 0, 1] {
            if dr == 0 && dc == 0 {
                continue;
            }
            let nr = row as i32 + dr;
            let nc = col as i32 + dc;
            if nr >= 0
                && nc >= 0
                && nr < board.len() as i32
                && nc < board[0].len() as i32
                && !visited[nr as usize][nc as usize]
            {
                dfs(
                    board,
                    visited,
                    next_node,
                    current_word,
                    found,
                    nr as usize,
                    nc as usize,
                );
            }
        }
    }

    current_word.pop();
    visited[row][col] = false;
}

pub fn solve(board: &Vec<Vec<u8>>, trie: &TrieNode) -> Vec<String> {
    // TODO the board might not be a rectangle
    // find max len for each use
    let rows = board.len();
    let cols = board[0].len();
    let mut visited = vec![vec![false; cols]; rows];
    let mut found = Vec::new();
    let mut current_word = String::new();

    for r in 0..rows {
        for c in 0..cols {
            dfs(
                board,
                &mut visited,
                &trie,
                &mut current_word,
                &mut found,
                r,
                c,
            );
        }
    }

    found.sort();
    found.dedup();
    found
}
