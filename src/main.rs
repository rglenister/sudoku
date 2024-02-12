use std::collections::{BTreeSet, HashMap};
use std::process::exit;
use std::fmt;

const NUMBER_OF_SQUARES: i32 = 81;
const ROW_LENGTH: usize = 9;

struct Board {
    board: Vec<i32>,
}

fn main() {
    // let puzzle_definition = {"
    //     4--958--1
    //     -7316254-
    //     1--743-2-
    //     3--21---5
    //     71-39526-
    //     2--48-1--
    //     ---871-5-
    //     -3752491-
    //     5-1639--2
    // "};
    let puzzle_definition = {
        "
        -6-----8-
        ---234--7
        2-1------
        ---5----1
        --73--29-
        4----7---
        ----1-6--
        5--879---
        -8-------
    "
    };

    fn load_puzzle(definition: &str) -> Vec<i32> {
        let definition_without_whitespace: String = definition.chars().filter(|c| !c.is_whitespace()).collect();
        if definition_without_whitespace.len() != NUMBER_OF_SQUARES as usize {
            eprintln!("The puzzle definition has incorrect length");
            exit(1);
        }
        return definition_without_whitespace
            .chars().flat_map(|ch| ch.to_digit(10).or_else(|| Some(0))).map(|i| i as i32).collect();
    }

    let mut the_board = Board { board: load_puzzle(puzzle_definition) };
    let square_to_squares_map = create_square_to_squares_map();

    println!("Puzzle definition:");
    println!("{}", the_board);
    println!("{} solution(s) found", search(&mut the_board, &square_to_squares_map, 0, 0));
}

// Brute force search that populates each empty square with all legal values
fn search(
    board: &mut Board,
    square_to_squares_map: &HashMap<i32, BTreeSet<i32>>,
    square_index: i32,
    num_solutions_found: i32,
) -> i32 {
    if square_index == NUMBER_OF_SQUARES {
        // a solution was found
        println!("{}", &board);
        return num_solutions_found + 1;
    } else if board.board[square_index as usize] != 0 {
        // square already populated so move to the next one
        search(board, square_to_squares_map, square_index + 1, num_solutions_found)
    } else {
        // fill out the square with all legal square values
        fill_square_value(board, square_to_squares_map, square_index, 1, num_solutions_found)
    }
}

// Recursively populates the given square with each legal value and searches each resulting position
fn fill_square_value(
    board: &mut Board,
    square_to_squares_map: &HashMap<i32, BTreeSet<i32>>,
    square_index: i32,
    square_value: i32,
    num_solutions_found: i32,
) -> i32 {
    return if square_value > 9 {
        // all square values have been tried
        num_solutions_found
    } else if is_legal_square_value(board, square_to_squares_map, square_index, square_value) {
        // populate the square and search the new position
        board.board[square_index as usize] = square_value;
        let temp_num_solutions = search(board, &square_to_squares_map, square_index + 1, num_solutions_found);
        let next_num_solutions_found: i32 = fill_square_value(board, square_to_squares_map, square_index, square_value + 1, temp_num_solutions);
        board.board[square_index as usize] = 0;
        next_num_solutions_found
    } else {
        // try the next square value
        fill_square_value(board, square_to_squares_map, square_index, square_value + 1, num_solutions_found)
    };
}

// Checks that the given square can be legally populated with the given value
fn is_legal_square_value(
    board: &mut Board,
    square_to_squares_map: &HashMap<i32, BTreeSet<i32>>,
    square_index: i32,
    square_value: i32,
) -> bool {
    return square_to_squares_map.get(&square_index)
        .map(|sqs| sqs.iter().map(|sq| board.board[*sq as usize])).expect("Pre-populated")
        .find(|val| val == &square_value).is_none();
}

// Creates a map from each board square to the set of squares whose value must be checked before the key square ios populated
fn create_square_to_squares_map() -> HashMap<i32, BTreeSet<i32>> {
    let mut result = HashMap::new();

    for square_index in 0..NUMBER_OF_SQUARES {
        let x = square_index % 9;
        let y = square_index / 9;
        let mut square_set: BTreeSet<i32> = BTreeSet::new();
        for index in 0..9 {
            square_set.insert(y * 9 + index); // row squares
            square_set.insert(index * 9 + x); // column squares
            square_set.insert(9 * (y - y % 3 + index % 3) + x - x % 3 + index / 3); // the nine 3 by 3 inner squares
        }
        result.insert(square_index, square_set);
    }
    return result;
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result: String = self.board.chunks(ROW_LENGTH)
            .map(|line| line.iter()
                .map(|item: &i32| { if item > &0 { item.to_string() } else { " ".to_string() } })
                .collect::<Vec<_>>())
            .collect::<Vec<_>>()
            .iter().map(|line| line.join(" "))
            .collect::<Vec<_>>()
            .join("\n") + "\n";

        return write!(f, "{}", result);
    }
}
