use std::collections::{BTreeSet, HashMap};
use fix_fn::fix_fn;

const NUMBER_OF_SQUARES: i32 = 81;

fn main() {
    let puzzle_definition = {"
        4--958--1
        -7316254-
        1--743-2-
        3--21---5
        71-39526-
        2--48-1--
        ---871-5-
        -3752491-
        5-1639--2
    "};

    fn load_puzzle(definition: &str) -> Vec<Vec<u32>> {
        let definition_without_whitespace: String = definition.chars().filter(|c| !c.is_whitespace()).collect();
        return definition_without_whitespace.chars().flat_map(|ch| ch.to_digit(10).or_else(|| Some(0))).collect::<Vec<_>>()
            .chunks(9).map(|s| s.into()).collect();
    }

    let mut the_board: Vec<Vec<u32>> = load_puzzle(puzzle_definition);
//    println!("{:?}", the_board);

    let search = fix_fn!(|search, board: &mut Vec<Vec<u32>>, x: i32, y: i32, num_solutions_found: i32| -> i32 {
        let fill_square_value = fix_fn!(|fill_square_value, board: &mut Vec<Vec<u32>>, square_value: u32,num_solutions_found: i32| -> i32 {
            let is_legal_square_value = fix_fn!(|is_legal_square_value, board: &mut Vec<Vec<u32>>, square_value:u32, square_index: i32| -> bool {
                return square_index >= 9 ||
                    (board[x as usize][square_index as usize] != square_value) && board[square_index as usize][y as usize] != (square_value) &&
                        board[(x / 3 * 3 + square_index % 3) as usize][(y / 3 * 3 + square_index / 3) as usize] != square_value &&
                        is_legal_square_value(board, square_value, square_index + 1);
            });
            return if square_value > 9 {
                num_solutions_found
            } else if is_legal_square_value(board, square_value, 0) {
                board[x as usize][y as usize] = square_value;
                let temp_num_solutions = search(board, x + 1, y, num_solutions_found);
                let next_num_solutions_found: i32 = fill_square_value(board, square_value + 1, temp_num_solutions);
                board[x as usize][y as usize] = 0;
                return next_num_solutions_found
            } else {
                return fill_square_value(board, square_value + 1, num_solutions_found)
            }
        });
        match (x, y) {
            (9, _) => { return search(board, 0, y + 1, num_solutions_found) }
            (0, 9) => { print_board(board.clone()); return num_solutions_found + 1 }
            _ => if board[x as usize][y as usize] != 0 {
                search(board, x + 1, y, num_solutions_found)
            } else {
                fill_square_value(board, 1, num_solutions_found)
            }
        }
    });

    println!("Puzzle definition:");
    print_board(the_board.clone());
    println!("\nSolutions:");
    println!("\n{} solution(s) found", search(&mut the_board, 0, 0, 0));
}

fn print_board(board: Vec<Vec<u32>>) {
    println!();
    board.iter().for_each(| chunk| {
        chunk.iter().for_each(| element| {
            let value_to_print = if element > &0 { element.to_string() } else { " ".to_string() };
            print!("{}  ", value_to_print);
        });
        println!();
    });
}

fn create_square_to_squares_to_check_map() -> HashMap<i32, BTreeSet<i32>> {
    let mut result = HashMap::new();

    for square_index in 0..NUMBER_OF_SQUARES {
        let x = square_index % 9;
        let y = square_index / 9;
        let mut square_set = BTreeSet::new();
        for index in 0..9 {
            square_set.insert(y * 9 + index); // row squares
            square_set.insert(index * 9 + x); // column squares
            square_set.insert(9 * (y / 3 * 3 + index % 3) + x / 3 * 3 + index / 3); // the nine 3 by 3 inner squares
        }
        result.insert(square_index, square_set);
    }
    return result;
}
