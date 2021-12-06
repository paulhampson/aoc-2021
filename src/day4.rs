use std::borrow::BorrowMut;

use read_file::InputParser;

struct BingoBoard {
    numbers: Vec<Vec<i32>>,
    marked: Vec<Vec<bool>>,
    eliminated: bool,
}

struct BingoData {
    number_draw: Vec<i32>,
    boards: Vec<BingoBoard>,
}

// Load a bingo boards
fn load_bingo_boards(input_data: &Vec<String>) -> Vec<BingoBoard> {
    let mut bingo_boards = Vec::new();
    let mut board = BingoBoard {
        numbers: Vec::new(),
        marked: Vec::new(),
        eliminated: false,
    };

    for line in input_data {
        if line.len() > 0 {
            // parse board line
            let mut board_line = Vec::new();
            let mut marker_line = Vec::new();
            for board_number in line.split_whitespace() {
                board_line.push(board_number.parse::<i32>().unwrap());
                marker_line.push(false);
            }
            board.numbers.push(board_line);
            board.marked.push(marker_line);
        } else {
            // store current board and start new board
            bingo_boards.push(board);
            board = BingoBoard {
                numbers: Vec::new(),
                marked: Vec::new(),
                eliminated: false,
            };
        }
    }
    // save the last board
    bingo_boards.push(board);

    return bingo_boards;
}

fn load_bingo_data(input_data: &Vec<String>) -> BingoData {
    let mut number_draw: Vec<i32> = Vec::new();
    // first line is always number draw as a line of CSV
    let number_draw_str = input_data[0].split(",");
    for number_str in number_draw_str {
        number_draw.push(number_str.parse().unwrap());
    }

    // now read an arbitrary number of 5x5 boards with space separated integers - always 3 chars long
    let board_input_data = input_data[2..].to_vec();
    let bingo_boards = load_bingo_boards(&board_input_data);

    BingoData {
        number_draw,
        boards: bingo_boards,
    }
}

fn mark_board(drawn_number: i32, board: &mut BingoBoard) {
    for row_idx in 0..board.numbers.len() {
        let board_row = &board.numbers[row_idx];
        if board_row.contains(&drawn_number) {
            let drawn_number_idx = board_row.iter().position(|&x| x == drawn_number).unwrap();
            board.marked[row_idx][drawn_number_idx] = true;
        }
    }
}

fn is_board_winner(board: &BingoBoard) -> bool {
    // check rows
    let marked = board.marked.clone();
    for row in marked {
        if row.iter().all(|mark| *mark == true) {
            return true;
        }
    }

    // check columns
    let col_count = board.marked[0].len();
    for col_idx in 0..col_count {
        let mut col_marks = Vec::new();
        let marked = board.marked.clone();
        for row in marked {
            col_marks.push(row[col_idx]);
        }
        // check if all column marks are true
        if col_marks.iter().all(|mark| *mark == true) {
            return true;
        }
    }
    return false;
}

fn sum_unmarked_values(bingo_board: &BingoBoard) -> i32 {
    let mut sum = 0;

    for row_idx in 0..bingo_board.numbers.len() {
        for it in bingo_board.numbers[row_idx].iter().zip(bingo_board.marked[row_idx].iter()) {
            let (value, marked) = it;
            if !*marked {
                sum += *value;
            }
        }
    }

    return sum;
}

fn play_bingo_and_get_score(bingo_data: &mut BingoData) -> i32 {
    let number_draw = bingo_data.number_draw.clone();
    for drawn_number in number_draw {
        println!("Drawn {}", drawn_number);
        let boards: &mut [BingoBoard] = bingo_data.boards.borrow_mut();
        for board in boards {
            mark_board(drawn_number, board);
            if is_board_winner(&board) {
                println!("Winner! {:?}", board.numbers);
                println!("Winner! {:?}", board.marked);
                let unmarked_sum = sum_unmarked_values(board);
                println!("Sum: {}", unmarked_sum);
                return unmarked_sum * drawn_number;
            }
        }
    }
    return 0;
}

fn play_bingo_and_get_score_of_last_winner(bingo_data: &mut BingoData) -> i32 {
    let number_draw = bingo_data.number_draw.clone();
    let mut last_sum = 0;
    for drawn_number in number_draw {
        println!("Drawn {}", drawn_number);
        let boards: &mut [BingoBoard] = bingo_data.boards.borrow_mut();
        for board in boards {
            if !board.eliminated {
                mark_board(drawn_number, board);
                if is_board_winner(&board) {
                    println!("Winner! {:?}", board.numbers);
                    println!("Winner! {:?}", board.marked);
                    let unmarked_sum = sum_unmarked_values(board);
                    last_sum = unmarked_sum * drawn_number;
                    board.eliminated = true;
                }
            }
        }
    }
    return last_sum;
}

pub fn day4a() -> String {
    let input_lines: Vec<String> = Vec::read_input("./inputs/day4/full".to_string());
    let mut bingo_data = load_bingo_data(&input_lines);
    play_bingo_and_get_score(&mut bingo_data).to_string()
}


pub fn day4b() -> String {
    let input_lines: Vec<String> = Vec::read_input("./inputs/day4/full".to_string());
    let mut bingo_data = load_bingo_data(&input_lines);
    play_bingo_and_get_score_of_last_winner(&mut bingo_data).to_string()
}