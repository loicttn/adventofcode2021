use std::fs;

type BoardLine = Vec<(u32, bool)>;
type Board = Vec<BoardLine>;

const BOARD_SIZE: usize = 5;

fn update_board(board: &mut Board, n: u32) -> bool {
    for a in 0..BOARD_SIZE {
        let mut col = 0;
        let mut row = 0;
        for b in 0..BOARD_SIZE {
            if board[a][b].0 == n {
                board[a][b].1 = true;
            }
            if board[a][b].1 == true {
                row += 1;
            }
            if board[b][a].0 == n {
                board[b][a].1 = true;
            }
            if board[b][a].1 == true {
                col += 1;
            }
        }
        if col == BOARD_SIZE || row == BOARD_SIZE {
            return true;
        }
    }
    false
}

fn compute_board_score(board: &Board, n: u32) -> u32 {
    let mut unmarked = 0;

    for row in board {
        for elem in row {
            if elem.1 == false {
                unmarked += elem.0;
            }
        }
    }
    unmarked * n
}

fn main() {
    let content = fs::read_to_string("input.txt").expect("Could not load input file");
    let lines = content.split("\n").collect::<Vec<_>>();
    let numbers = lines[0].split(",").map(|e| e.parse().unwrap()).collect::<Vec<u32>>();
    let mut boards: Vec<Board> = Vec::new();

    // collect boards
    for i in 1..lines.len() {
        if lines[i].len() == 0 {
            boards.push([].to_vec());
            continue;
        }
        let board = boards.last_mut().unwrap();
        let rawline = lines[i].split(" ").filter(|e| e.len() != 0);
        let line: BoardLine = rawline.map(|e| (e.parse().unwrap(), false)).collect::<BoardLine>();
        if line.len() != 0 {
            board.push(line);
        }
    }

    // compute game
    let mut res = 0;
    'o: for n in numbers {
        for i in 0..boards.len()-1 {
            let found = update_board(&mut boards[i], n);
            if found {
                res = compute_board_score(&boards[i], n);
                break 'o;
            }
        }
    }

    println!("Solution is {}", res);
}
