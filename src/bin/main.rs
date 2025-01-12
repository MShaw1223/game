use std::collections::HashMap;

use text_io::read;

fn main() {
    let map: Vec<Vec<i8>> = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
        vec![0, 1, 0, 0, 0, 4, 0, 0, 0, 1, 0],
        vec![0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0],
        vec![0, 1, 1, 0, 0, 1, 0, 0, 0, 1, 0],
        vec![0, 0, 1, 3, 1, 1, 1, 4, 1, 1, 0],
        vec![0, 1, 1, 0, 0, 1, 0, 1, 0, 1, 0],
        vec![0, 1, 0, 0, 0, 1, 1, 2, 1, 1, 0],
        vec![0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0],
        vec![0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ];

    let tiles: Vec<String> = vec![
        "wall".to_string(),
        "path".to_string(),
        "loot".to_string(),
        "door".to_string(),
        "monster".to_string(),
    ];

    let mut moves: Vec<String> = vec!["None".to_string()];

    let mut score: i32 = 0;

    let mut curr_row: usize = 5;
    let mut curr_col: usize = 5;

    loop {
        let curr = capture_current_position(curr_row, curr_col);
        let curr_value = get_current_position_value(curr, &map);
        let next_values = get_next_move_value(curr, &map);

        println!("Score: {}", score);
        println!(
            "Pos: {:?}\nTile: {:?}\nStanding on: {}",
            curr, curr_value, tiles[curr_value as usize]
        );
        display(curr, &next_values, &map);
        println!("AWSD to move, q to quit");

        let input: String = read!();

        match input.as_str() {
            "a" => {
                println!("press: a");
                if next_values[input.as_str()] != 0 {
                    match move_character(curr, &input) {
                        Ok(new_val) => curr_col = new_val,
                        Err(()) => eprintln!("Error moving character"),
                    };
                    moves.push(input);
                }
            }
            "w" => {
                println!("press: w");
                if next_values[input.as_str()] != 0 {
                    match move_character(curr, &input) {
                        Ok(new_val) => curr_row = new_val,
                        Err(()) => eprintln!("Error moving character"),
                    };
                    moves.push(input);
                }
            }
            "s" => {
                println!("press: s");
                if next_values[input.as_str()] != 0 {
                    match move_character(curr, &input) {
                        Ok(new_val) => curr_row = new_val,
                        Err(()) => eprintln!("Error moving character"),
                    };
                    moves.push(input);
                }
            }
            "d" => {
                println!("press: d");
                if next_values[input.as_str()] != 0 {
                    match move_character(curr, &input) {
                        Ok(new_val) => curr_col = new_val,
                        Err(()) => eprintln!("Error moving character"),
                    };
                    moves.push(input);
                }
            }
            "q" => break,
            _ => {
                println!("Invalid input")
            }
        }
    }
    println!("Last move: {}", moves.last().unwrap());
    println!("Last position: {},{}", curr_row, curr_col);
}

fn capture_current_position(row: usize, col: usize) -> (usize, usize) {
    return (row, col);
}

fn get_current_position_value(current: (usize, usize), map: &Vec<Vec<i8>>) -> i8 {
    map[current.0][current.1]
}

fn get_next_move_value(curr: (usize, usize), map: &Vec<Vec<i8>>) -> HashMap<&str, i8> {
    let row = curr.0 as usize;
    let col = curr.1 as usize;

    let dict: HashMap<&str, i8> = HashMap::from([
        ("a", map[row][col - 1]),
        ("w", map[row - 1][col]),
        ("s", map[row + 1][col]),
        ("d", map[row][col + 1]),
    ]);

    dict
}

fn display(current: (usize, usize), next: &HashMap<&str, i8>, map: &Vec<Vec<i8>>) -> () {
    println!("\t{:?}", next["w"]);
    println!(
        "{:?}\t{:?}\t{:?}",
        next["a"], map[current.0 as usize][current.1 as usize], next["d"]
    );
    println!("\t{:?}", next["s"]);
}

fn move_character(current: (usize, usize), input: &str) -> Result<usize, ()> {
    let row = current.0 as usize;
    let col = current.1 as usize;

    match input {
        "a" => Ok(col - 1),
        "w" => Ok(row - 1),
        "s" => Ok(row + 1),
        "d" => Ok(col + 1),
        _ => Err(()),
    }
}
