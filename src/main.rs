
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

    println!("Puzzle definition:");
    print(the_board.clone());
    println!("\nSolutions:");
    println!("\n{} solution(s) found", search(&mut the_board, 0, 0, 0));
}

fn search(board: &mut Vec<Vec<u32>>, x: i32, y: i32, num_solutions_found: i32) -> i32 {
    match (x, y) {
        (9, _) => { return search(board, 0, y + 1, num_solutions_found) }
        (0, 9) => { print(board.clone()); return num_solutions_found + 1 }
        _ => if board[x as usize][y as usize] != 0 {
            search(board, x + 1, y, num_solutions_found)
        } else {
            fill_square_value(board, x, y,1, num_solutions_found)
        }
    }
}

fn fill_square_value(board: &mut Vec<Vec<u32>>, x: i32, y: i32, square_value: u32, num_solutions_found: i32) -> i32 {
    return if square_value > 9 {
        num_solutions_found
    } else if is_legal_square_value(board, x, y, square_value, 0) {
        board[x as usize][y as usize] = square_value;
        let temp_num_solutions = search(board, x + 1, y, num_solutions_found);
        let next_num_solutions_found: i32 = fill_square_value(board, x, y, square_value + 1, temp_num_solutions);
        board[x as usize][y as usize] = 0;
        return next_num_solutions_found
    } else {
        return fill_square_value(board, x, y, square_value + 1, num_solutions_found)
    }
}


fn is_legal_square_value(board: &mut Vec<Vec<u32>>, x: i32, y: i32, square_value:u32, square_index: i32) -> bool {
    return square_index >= 9 ||
        (board[x as usize][square_index as usize] != square_value) && board[square_index as usize][y as usize] != (square_value) &&
            board[(x / 3 * 3 + square_index % 3) as usize][(y / 3 * 3 + square_index / 3) as usize] != square_value &&
            is_legal_square_value(board, x, y, square_value, square_index + 1);
}

fn print(board: Vec<Vec<u32>>) {
    println!();
    board.iter().for_each(| chunk| {
        chunk.iter().for_each(| element|
            print!("{element} ")
        );
        println!();
    });
}
