use rand::Rng;
use std::collections::HashMap;
use text_io::read;

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug, Clone)]
pub struct MoveOptions {
    pub options: HashMap<&'static str, i8>,
}

// Return values
pub fn capture_current_position(row: usize, col: usize) -> Position {
    Position { row, col }
}

pub fn get_current_position_value(current: Position, map: &Vec<Vec<i8>>) -> i8 {
    map[current.row][current.col]
}

pub fn get_next_move_value(curr: Position, map: &Vec<Vec<i8>>) -> MoveOptions {
    MoveOptions {
        options: HashMap::from([
            ("a", map[curr.row][curr.col - 1]),
            ("w", map[curr.row - 1][curr.col]),
            ("s", map[curr.row + 1][curr.col]),
            ("d", map[curr.row][curr.col + 1]),
        ]),
    }
}

pub fn display(current: Position, next: &MoveOptions, map: &Vec<Vec<i8>>) -> () {
    println!("\t{:?}", next.options["w"]);
    println!(
        "{:?}\t{:?}\t{:?}",
        next.options["a"], map[current.row][current.col], next.options["d"]
    );
    println!("\t{:?}", next.options["s"]);
}

pub fn move_character(current: Position, input: &str) -> Result<usize, ()> {
    match input {
        "a" => Ok(current.col - 1),
        "w" => Ok(current.row - 1),
        "s" => Ok(current.row + 1),
        "d" => Ok(current.col + 1),
        _ => Err(()),
    }
}

// No return values
pub fn path_movement(curr: Position, input: String, row_col: &mut usize, moves: &mut Vec<String>) {
    match move_character(curr, &input) {
        Ok(new_val) => *row_col = new_val,
        Err(()) => eprintln!("Error moving character"),
    };
    moves.push(input);
}

pub fn door_movement(curr: Position, input: String, row_col: &mut usize, extra_step: usize) {
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
    curr: Position,
    input: String,
    row_col: &mut usize,
    monsters: &Vec<String>,
    score: &mut i32,
    health: &mut i32,
    extra_step: usize,
) {
    let random = rand::thread_rng().gen_range(0..3);
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

pub fn loot_movement(
    curr: Position,
    input: String,
    row_col: &mut usize,
    moves: &mut Vec<String>,
    user_weapons: &mut Vec<(&str, i32)>,
) {
    let game_weapons: Vec<(&str, i32)> = vec![("sword", 12), ("bow", 10), ("axe", 15)];

    let random = rand::thread_rng().gen_range(0..3);

    user_weapons.push(game_weapons[random]);

    match move_character(curr, &input) {
        Ok(new_val) => *row_col = new_val,
        Err(()) => eprintln!("Error moving character"),
    };
    moves.push(input);
    println!("You have picked up a: {}", game_weapons[random].0)
}
