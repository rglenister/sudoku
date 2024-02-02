use std::collections::{BTreeSet, HashMap};

const NUMBER_OF_SQUARES: i32 = 81;
const ROW_LENGTH: usize = 9;

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

    fn load_puzzle(definition: &str) -> Vec<i32> {
        let definition_without_whitespace: String = definition.chars().filter(|c| !c.is_whitespace()).collect();
        return definition_without_whitespace
            .chars().flat_map(|ch| ch.to_digit(10).or_else(|| Some(0))).map(|i| i as i32).collect();
    }

    let mut the_board: Vec<i32> = load_puzzle(puzzle_definition);
    let mut square_to_squares_map= create_square_to_squares_map();
    println!("Puzzle definition:");
    print_board(the_board.clone());
    println!("\nSolutions:");
    println!("\n{} solution(s) found", search(&mut the_board, &mut square_to_squares_map, 0, 0));
}

fn search(
    board: &mut Vec<i32>,
    square_to_squares_map: &mut HashMap<i32, BTreeSet<i32>>,
    square_index: i32,
    num_solutions_found: i32
) -> i32 {
    match square_index {
        NUMBER_OF_SQUARES => { print_board(board.clone()); return num_solutions_found + 1 }
        _ => if board[square_index as usize] != 0 {
            search(board, square_to_squares_map,square_index + 1, num_solutions_found)
        } else {
            fill_square_value(board, square_to_squares_map, square_index,1, num_solutions_found)
        }
    }
}

fn fill_square_value(
    board: &mut Vec<i32>,
    square_to_squares_map: &mut HashMap<i32, BTreeSet<i32>>,
    square_index: i32,
    square_value: i32,
    num_solutions_found: i32
) -> i32 {
    return if square_value > 9 {
        num_solutions_found
    } else if is_legal_square_value(board, square_to_squares_map, square_index, square_value) {
        board[square_index as usize] = square_value;
        let temp_num_solutions = search(board, square_to_squares_map, square_index + 1, num_solutions_found);
        let next_num_solutions_found: i32 = fill_square_value(board, square_to_squares_map, square_index, square_value + 1, temp_num_solutions);
        board[square_index as usize] = 0;
        return next_num_solutions_found
    } else {
        return fill_square_value(board, square_to_squares_map, square_index,square_value + 1, num_solutions_found)
    }
}

fn is_legal_square_value(
    board: &mut Vec<i32>,
    square_to_squares_map: &HashMap<i32, BTreeSet<i32>>,
    square_index: i32,
    square_value: i32
) -> bool {
    return !square_to_squares_map.get(&square_index)
        .map(|sqs| sqs.iter().map(|sq| board[*sq as usize])).expect("REASON")
        .find(|val| val == &square_value).is_some();
}
fn print_board(
    board: Vec<i32>
) {
    println!();
    let result: String = board.chunks(ROW_LENGTH)
        .map(|line| line.iter().map(|item: &i32| { if item > &0 { item.to_string()} else {" ".to_string() }}).collect::<Vec<_>>())
        .collect::<Vec<_>>()
        .iter().map(|line| line.join("  "))
        .collect::<Vec<_>>()
        .join("\n");
    println!("{}", result);
}

fn create_square_to_squares_map() -> HashMap<i32, BTreeSet<i32>> {
    let mut result = HashMap::new();

    for square_index in 0..NUMBER_OF_SQUARES {
        let x = square_index % 9;
        let y = square_index / 9;
        let mut square_set: BTreeSet<i32> = BTreeSet::new();
        for index in 0..9 {
            square_set.insert(y * 9 + index); // row squares
            square_set.insert(index * 9 + x); // column squares
            square_set.insert(9 * (y / 3 * 3 + index % 3) + x / 3 * 3 + index / 3); // the nine 3 by 3 inner squares
        }
        result.insert(square_index, square_set);
    }
    return result;
}