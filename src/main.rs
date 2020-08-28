use std::io;

fn main() {
    let table = Table {
        matrix: [[0, 0, 0], [0, 0, 0], [0, 0, 0]],
    };

    table.print_matrix();
    let mut test = String::new();

    get_player(2, &mut test);
}

fn get_player(player_num: u32, player_name: &mut String) {
    println!("Insert player {} name: ", player_num);
    match io::stdin().read_line(player_name) {
        Ok(i) => {
            println!("Player 1 : {}", player_name);
        }
        Err(error) => {
            println!("err");
            //player_name = &mut format!("player_{}", player_num)
        }
    }
}

struct Table {
    matrix: [[u8; 3]; 3],
}

struct Player {
    name: String,
    id: char,
}

impl Table {
    fn print_matrix(&self) {
        for i in 0..3 {
            for j in 0..3 {
                println!("{}", self.matrix[i][j]);
            }
        }
    }
}
