use std::cmp::Ordering;
use std::io;

fn main() {
    let player_one: Player = get_player('X');
    let player_two: Player = get_player('O');
    let mut table = Table {
        matrix: [['-'; 3], ['-'; 3], ['-'; 3]],
        players: [player_one, player_two],
        game_number: 1,
    };

    loop {
        println!("Game number {} start !", table.game_number);
        let mut iterations = 0;
        let mut curr_player = 1;

        while iterations < 9 {
            let curr_player: &Player = if curr_player == 0 {
                curr_player = 1;
                &table.players[1]
            } else {
                curr_player = 0;
                &table.players[0]
            };

            loop {
                println!("Player {} enter cell number : ", curr_player.name);
                let mut user_input = String::new();
                io::stdin()
                    .read_line(&mut user_input)
                    .expect("err cell number");

                match user_input.trim().parse::<i32>() {
                    Ok(number) => {
                        if number > 9 || number < 0 {
                            println!("The number can't be bigger than 9 and lower than 0");
                            continue;
                        }
                        let (x, y) = get_coordinates(number);
                        if table.matrix[x][y] == 'X' || table.matrix[x][y] == 'O' {
                            println!("The cell is already setted!");
                            continue;
                        } else {
                            table.matrix[x][y] = curr_player.id;
                        }
                        if let Some(player) = table.check_winner() {
                            println!("Player {} has won!", player.name);
                            table.game_number += 1;
                            iterations = 9;
                        }

                        iterations += 1;
                        break;
                    }
                    Err(err) => {
                        println!("The input is not a number, retry");
                        continue;
                    }
                }
            }

            table.print_matrix();
        }
        println!("Continue ? (Yes/No)");
        let mut user_choice = String::new();
        io::stdin()
            .read_line(&mut user_choice)
            .expect("Err reading user choice");
        let user_choice = user_choice.trim().to_lowercase();
        match &user_choice[..] {
            "yes" => {
                continue;
            }
            _ => {
                break;
            }
        }
    }
}

fn get_coordinates(mut grid_position: i32) -> (usize, usize) {
    for i in 0..3 {
        for j in 0..3 {
            grid_position -= 1;
            if grid_position == 0 {
                return (i, j);
            }
        }
    }

    (0, 0)
}

fn get_player(id: char) -> Player {
    loop {
        println!("Insert player name : ");
        let mut player_name = String::new();
        io::stdin()
            .read_line(&mut player_name)
            .expect("err reading name");

        player_name = String::from(player_name.trim());
        if let Ordering::Less = player_name.len().cmp(&(4_usize)) {
            println!("Player name to short");
            continue;
        }

        return Player {
            name: player_name,
            id: id,
        };
    }
}

struct Player {
    name: String,
    id: char,
}

struct Table {
    matrix: [[char; 3]; 3],
    players: [Player; 2],
    game_number: i32,
}
impl Table {
    fn print_matrix(&self) {
        for i in 0..3 {
            for j in 0..3 {
                print!("[{}]", self.matrix[i][j]);
            }

            println!("");
        }

        println!("");
    }

    fn check_winner(&mut self) -> Option<&Player> {
        let possibilities = [
            [[0; 3], [0, 1, 2]],
            [[1; 3], [0, 1, 2]],
            [[2; 3], [0, 1, 2]],
            [[0, 1, 2], [0; 3]],
            [[0, 1, 2], [1; 3]],
            [[0, 1, 2], [2; 3]],
            [[0, 1, 2], [0, 1, 2]],
            [[2, 1, 0], [0, 1, 2]],
        ];

        for group in possibilities.iter() {
            let x_cords = group[0];
            let y_cords = group[1];
            let mut x_count = 0;
            let mut o_count = 0;
            for i in 0..3 {
                match self.matrix[x_cords[i]][y_cords[i]] {
                    'X' => {
                        x_count += 1;
                    }
                    'O' => {
                        o_count += 1;
                    }
                    _ => {}
                }
            }

            if let Some(player) = self.get_winner(x_count, o_count) {
                return Some(player);
            }
        }

        return None;
    }

    fn get_winner(&self, first: i32, second: i32) -> Option<&Player> {
        if first == 3 {
            return Some(&self.players[0]);
        } else if second == 3 {
            return Some(&self.players[1]);
        }

        None
    }
}
