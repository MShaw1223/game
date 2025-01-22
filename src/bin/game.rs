use std::collections::HashMap;
use text_io::read;

pub fn capture_current_position(row: usize, col: usize) -> (usize, usize) {
    return (row, col);
}

pub fn get_current_position_value(current: (usize, usize), map: &Vec<Vec<i8>>) -> i8 {
    map[current.0][current.1]
}

pub fn get_next_move_value(curr: (usize, usize), map: &Vec<Vec<i8>>) -> HashMap<&str, i8> {
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

pub fn display(current: (usize, usize), next: &HashMap<&str, i8>, map: &Vec<Vec<i8>>) -> () {
    println!("\t{:?}", next["w"]);
    println!(
        "{:?}\t{:?}\t{:?}",
        next["a"], map[current.0 as usize][current.1 as usize], next["d"]
    );
    println!("\t{:?}", next["s"]);
}

pub fn move_character(current: (usize, usize), input: &str) -> Result<usize, ()> {
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

pub fn path_movement(
    curr: (usize, usize),
    input: String,
    row_col: &mut usize,
    moves: &mut Vec<String>,
) {
    match move_character(curr, &input) {
        Ok(new_val) => *row_col = new_val,
        Err(()) => eprintln!("Error moving character"),
    };
    moves.push(input);
}

pub fn door_movement(curr: (usize, usize), input: String, row_col: &mut usize, extra_step: usize) {
    loop {
        println!("Enter door? y/n: ");
        let choice: String = read!();
        match choice.as_str() {
            "y" => {
                if extra_step == 1 {
                    match move_character(curr, &input) {
                        Ok(new) => *row_col = new + 1,
                        Err(()) => eprintln!("Error moving character"),
                    }
                    break;
                } else if extra_step == 0 {
                    match move_character(curr, &input) {
                        Ok(new) => *row_col = new - 1,
                        Err(()) => eprintln!("Error moving character"),
                    }
                    break;
                } else {
                    eprintln!("Incorrect step value.");
                }
            }
            "n" => {
                break;
            }
            _ => {
                eprintln!("Incorrect input.")
            }
        }
    }
}

pub fn monster_movement(
    curr: (usize, usize),
    input: String,
    row_col: &mut usize,
    monsters: &Vec<String>,
    random: i8,
    score: &mut i32,
    health: &mut i32,
    extra_step: usize,
) {
    let mut monster_health = 10;
    let monster = &monsters[random as usize];
    loop {
        println!("Monster");
        println!("{}", monster);
        if monster_health <= 0 {
            if extra_step == 1 {
                match move_character(curr, &input) {
                    Ok(new) => *row_col = new + 1,
                    Err(()) => eprintln!("Error leaving battle"),
                }
                *score += 5;
                break;
            } else if extra_step == 0 {
                match move_character(curr, &input) {
                    Ok(new) => *row_col = new - 1,
                    Err(()) => eprintln!("Error leaving battle"),
                }
                *score += 5;
                break;
            } else {
                eprintln!("Incorrect step value.");
            }
        } else {
            println!("Attack the {}", monster);
            println!("b to attack");
            let attack: String = read!();
            match attack.as_str() {
                "b" => monster_health -= 5,
                _ => eprintln!("Invalid attack."),
            }
            *health -= 1
        }
    }
}
