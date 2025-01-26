use std::collections::HashMap;

use text_io::read;

mod game;
use game::*;

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

    let mut user_weapons: Vec<(&str, i32)> = vec![("dagger", 5)];

    let mut score: i32 = 0;
    let mut health: i32 = 30;

    let mut curr_row: usize = 5;
    let mut curr_col: usize = 5;
    let monsters = vec![
        "Zombie".to_string(),
        "Werewolf".to_string(),
        "Ghost".to_string(),
    ];

    loop {
        let curr = capture_current_position(curr_row, curr_col);
        let curr_value = get_current_position_value(curr, &map);
        let next_values = get_next_move_value(curr, &map);

        println!("Health: {}", health);
        println!("Score: {}", score);
        println!(
            "Pos: {:?}\nTile: {:?}\nStanding on: {}",
            curr, curr_value, tiles[curr_value as usize]
        );
        display(curr, &next_values, &map);
        println!("AWSD to move, q to quit");

        let input: String = read!();

        match input.as_str() {
            "a" => match next_values[input.as_str()] {
                0 => println!("Ow, there is a wall there."),
                1 => path_movement(curr, input, &mut curr_col, &mut moves),
                2 => loot_movement(curr, input, &mut curr_col, &mut moves, &mut user_weapons),
                3 => door_movement(curr, input, &mut curr_col, 0),
                4 => monster_movement(
                    curr,
                    input,
                    &mut curr_col,
                    &monsters,
                    &mut score,
                    &mut health,
                    0,
                ),
                _ => eprintln!("Tile doesn't exist."),
            },
            "w" => match next_values[input.as_str()] {
                0 => println!("Ow, there is a wall there."),
                1 => path_movement(curr, input, &mut curr_row, &mut moves),
                2 => loot_movement(curr, input, &mut curr_row, &mut moves, &mut user_weapons),
                3 => door_movement(curr, input, &mut curr_row, 0),
                4 => monster_movement(
                    curr,
                    input,
                    &mut curr_row,
                    &monsters,
                    &mut score,
                    &mut health,
                    0,
                ),
                _ => eprintln!("Tile doesn't exist."),
            },
            "s" => match next_values[input.as_str()] {
                0 => println!("Ow, there is a wall there."),
                1 => path_movement(curr, input, &mut curr_row, &mut moves),
                2 => loot_movement(curr, input, &mut curr_row, &mut moves, &mut user_weapons),
                3 => door_movement(curr, input, &mut curr_row, 1),
                4 => monster_movement(
                    curr,
                    input,
                    &mut curr_row,
                    &monsters,
                    &mut score,
                    &mut health,
                    1,
                ),
                _ => eprintln!("Tile doesn't exist."),
            },
            "d" => match next_values[input.as_str()] {
                0 => println!("Ow, there is a wall there."),
                1 => path_movement(curr, input, &mut curr_col, &mut moves),
                2 => loot_movement(curr, input, &mut curr_col, &mut moves, &mut user_weapons),
                3 => door_movement(curr, input, &mut curr_col, 1),
                4 => monster_movement(
                    curr,
                    input,
                    &mut curr_col,
                    &monsters,
                    &mut score,
                    &mut health,
                    1,
                ),
                _ => eprintln!("Tile doesn't exist."),
            },
            "q" => break,
            _ => {
                eprintln!("Invalid input")
            }
        }
    }
    println!("Final Score: {}", score);
    println!("Last move: {}", moves.last().unwrap());
    println!("Last position: ({},{})", curr_row, curr_col);
}
