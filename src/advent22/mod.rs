use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Lines, prelude::*};

use indextree::Arena;
use regex::Regex;
use std::io::{self, Write};

use crate::advent22::grid_utils::{Cell, CellValue, Grid, Position};
use crate::advent22::grid_utils::CellValue::{EMPTY, PATH, WALL};

mod grid_utils;

fn part_one(file_path: &str) -> u32 {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().into_iter().map(|x| x.unwrap()).collect();

    println!("parsing input");
    let input = parse_input(lines);

    println!("applying commands");
    let last_move = apply_commands(input);

    let direction_score = match last_move.current_direction {
        Direction::NORTH => 3,
        Direction::EAST => 0,
        Direction::SOUTH => 1,
        Direction::WEST => 2,
    };

    return 1000 * last_move.end.y as u32 + 4 * last_move.end.x as u32 + direction_score;
}

fn parse_input(lines: Vec<String>) -> Input {
    let mut grid = Grid::new();
    let mut line_index = 1;

    let raw_commands = lines.last().unwrap();
    let mut commands: Vec<Command> = Vec::new();
    let steps_and_directions_separator = Regex::new(r"([0-9]+)|([RL])").unwrap();
    for command in steps_and_directions_separator.find_iter(raw_commands) {
        let command = command.as_str();
        if Regex::new(r"^[0-9]+$").unwrap().is_match(command) {
            commands.push(Command::Move(command.parse::<u32>().unwrap()));
        } else if command == "L" || command == "R" {
            commands.push(Command::RotateLeft(command == "L"));
        } else {
            panic!();
        }
    }

    for line_res in lines {
        let line = line_res;
        if line.is_empty() {
            break;
        }
        let mut column_index = 1;
        for cell in line.chars().into_iter() {
            let value = CellValue::try_from(cell).unwrap();
            grid.add_new_cell(column_index, line_index, value);
            column_index += 1;
        }
        line_index += 1;
    }

    return Input { grid, commands };
}

#[derive(Debug, Clone)]
pub enum Command {
    Move(u32),
    RotateLeft(bool),
}

fn apply_commands(input: Input) -> AppliedMove {
    let mut last_position = AppliedMove {
        end: find_starting_position(&input.grid),
        current_direction: Direction::EAST,
    };
    let mut position_history: Vec<AppliedMove> = Vec::new();

    for command in input.commands {
        println!("command: {:?}", command);
        io::stdout().flush().expect("TODO: panic message");
        match command {
            Command::Move(steps) => {
                last_position = apply_move(
                    &input.grid,
                    MoveInput {
                        start: last_position.end.clone(),
                        steps,
                        direction: last_position.current_direction.clone(),
                    });
                position_history.push(last_position.clone());
                // print_grid(&input.grid, position_history.clone());
            }
            Command::RotateLeft(is_rotating_left) => { last_position.rotate(is_rotating_left); }
        }
    };
    return last_position;
}

fn find_starting_position(grid: &Grid) -> Position {
    for x in 1..grid.max_x {
        let cell = grid.get(x, 1);
        if cell.is_some() && cell.unwrap().value == PATH {
            return Position::new(cell.unwrap().x, cell.unwrap().y);
        }
    }
    panic!();
}

fn print_grid(grid: &Grid, move_history: Vec<AppliedMove>) {
    let mut move_history = move_history;
    for y in 1..grid.max_y + 1 {
        for x in 1..grid.max_x + 1 {
            let cell = grid.get(x, y);
            let opt_move = move_history.iter().find(|m| m.end.x == x && m.end.y == y);
            if opt_move.is_some() {
                print!("{}",
                       match opt_move.unwrap().current_direction {
                           Direction::NORTH => "^",
                           Direction::EAST => ">",
                           Direction::SOUTH => "v",
                           Direction::WEST => "<",
                       }
                );
            } else if cell.is_none() || cell.unwrap().value == EMPTY {
                print!(" ");
            } else if cell.unwrap().value == WALL {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
    println!("==============================================");
    println!();
}

fn apply_move(grid: &Grid, move_input: MoveInput) -> AppliedMove {
    let x_increment: i32;
    let y_increment: i32;
    match move_input.direction {
        Direction::NORTH => {
            x_increment = 0;
            y_increment = -1;
        }
        Direction::EAST => {
            x_increment = 1;
            y_increment = 0;
        }
        Direction::SOUTH => {
            x_increment = 0;
            y_increment = 1;
        }
        Direction::WEST => {
            x_increment = -1;
            y_increment = 0;
        }
    }

    let mut x: i32 = move_input.start.x as i32;
    let mut y: i32 = move_input.start.y as i32;
    let mut steps_to_do = move_input.steps;
    while steps_to_do > 0 {
        x += x_increment;
        y += y_increment;
        let cell = grid.get(x as u32, y as u32);
        if cell.is_none() || cell.unwrap().value == EMPTY {
            // wrap around
            let mut wrap_around_x = x;
            let mut wrap_around_y = y;
            let mut opposite_cell: Option<&Cell> = None;
            let mut previous_cell : Option<&Cell>= Option::None;
            loop {
                wrap_around_x -= x_increment;
                wrap_around_y -= y_increment;
                let wrap_around_cell = grid.get(wrap_around_x as u32, wrap_around_y as u32);
                if wrap_around_cell.is_none() || wrap_around_cell.unwrap().value == EMPTY {
                    opposite_cell = previous_cell;
                    break;
                }
                previous_cell = wrap_around_cell;
            }
            if opposite_cell.is_none() { panic!() }
            if opposite_cell.unwrap().value == WALL {
                x -= x_increment;
                y -= y_increment;
            } else {
                x = opposite_cell.unwrap().x as i32;
                y = opposite_cell.unwrap().y as i32;
            }
        } else if cell.unwrap().value == WALL {
            // revert move
            x -= x_increment;
            y -= y_increment;
            break;
        } else if cell.unwrap().value == PATH {
            // move
        } else {
            panic!();
        }
        steps_to_do -= 1;
    }

    return AppliedMove { end: Position::new(x as u32, y as u32), current_direction: move_input.direction };
}

struct Input {
    grid: Grid,
    commands: Vec<Command>,
}

#[derive(Clone, PartialEq, Debug)]
struct MoveInput {
    start: Position,
    steps: u32,
    direction: Direction,
}

#[derive(Clone, PartialEq, Debug)]
struct AppliedMove {
    end: Position,
    current_direction: Direction,
}

impl AppliedMove {
    fn rotate(&mut self, left: bool) {
        let to_rotate = if left { -90 } else { 90 };
        let mut current_rotation = match self.current_direction {
            Direction::NORTH => 0,
            Direction::EAST => 90,
            Direction::SOUTH => 180,
            Direction::WEST => 270,
        };
        current_rotation = current_rotation + to_rotate;
        if current_rotation < 0 {
            current_rotation += 360;
        } else if current_rotation >= 360 {
            current_rotation -= 360;
        }

        self.current_direction = match current_rotation {
            0 => Direction::NORTH,
            90 => Direction::EAST,
            180 => Direction::SOUTH,
            270 => Direction::WEST,
            _ => panic!()
        };
    }
}

#[derive(Clone, PartialEq, Debug)]
enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

#[cfg(test)]
mod tests {
    use crate::advent22::{AppliedMove, apply_commands, apply_move, Direction, MoveInput, parse_input, part_one};
    use crate::advent22::Direction::{EAST, NORTH, SOUTH, WEST};
    use crate::advent22::grid_utils::{Grid, Position};

    #[test]
    fn ok_example_1() {
        assert_eq!(part_one("./src/advent22/input_example.txt"), 6032);
    }

    #[test]
    fn apply_move_ok() {
        let input = parse_input(vec!["...#".to_string()
                                     , ".#..".to_string(),
                                     "....".to_string(),
                                     "".to_string(),
                                     "2R1".to_string(),
        ]);
        let move_input = MoveInput {
            start: Position { x: 1, y: 1 },
            steps: 2,
            direction: EAST,
        };

        let res = apply_move(&input.grid, move_input);
        assert_eq!(res.end, Position::new(3, 1));
    }

    #[test]
    fn rotate_ok() {
        let mut m = AppliedMove { end: Position::new(1, 1), current_direction: Direction::NORTH };
        m.rotate(false);
        assert_eq!(m.current_direction, EAST);
        m.rotate(false);
        assert_eq!(m.current_direction, SOUTH);
        m.rotate(false);
        assert_eq!(m.current_direction, WEST);
        m.rotate(false);
        assert_eq!(m.current_direction, NORTH);
        m.rotate(true);
        assert_eq!(m.current_direction, WEST);
    }

    #[test]
    fn apply_commands_ok() {
        let input = parse_input(vec!["...#".to_string()
                                     , ".#..".to_string(),
                                     "....".to_string(),
                                     "".to_string(),
                                     "2R1".to_string(),
        ]);

        let res = apply_commands(input);
        assert_eq!(res.end, Position::new(3, 2));
    }
}