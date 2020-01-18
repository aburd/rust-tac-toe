use std::io;
use std::fmt::Display;

const BOARD_WIDTH: usize = 3;
const BOARD_HEIGHT: usize = 3;

enum Player {
    First,
    Second,
}

#[derive(Clone)]
enum Space {
    Blank,
    X,
    O,
}

struct Board {
    spaces: Vec<Space>,
    turn: Player,
}

impl Board {
    fn new() -> Self {
        let spaces = vec![Space::Blank; BOARD_WIDTH * BOARD_HEIGHT];
        Board {
            spaces,
            turn: Player::First,
        }
    }
}

impl Board {
    fn update_space(&mut self, row: usize, col: usize) {
        let space = match self.turn {
            Player::First => Space::X,
            Player::Second => Space::O,
        };
        let next_turn = match self.turn {
            Player::First => Player::Second,
            Player::Second => Player::First,
        };
        let current_space = &self.spaces[row * BOARD_WIDTH + col];
        match *current_space {
            Space::Blank => {
                self.spaces[row * BOARD_WIDTH + col] = space;
                self.turn = next_turn;
            },
            _ => println!("That space has already been played."),
        };
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s: String = self.spaces.iter().enumerate()
            .map(|(i, space)| {
                let last_in_row = (i + 1) % 3 == 0;
                let is_last_space = i == self.spaces.len() - 1;
                let cell_content = format!(" {} ", space_to_str(&space));

                if last_in_row && is_last_space {
                    format!("{}", cell_content)
                } else if last_in_row {
                    format!("{}\n---+---+---\n", cell_content)
                } else {
                    format!("{}|", cell_content)
                }
            })
            .collect();
        write!(f, "{}", s)
    }
}

fn space_to_str<'a>(space: &'a Space) -> &'a str {
    match space {
        Space::Blank => " ",
        Space::X => "X",
        Space::O => "O",
    }
}

fn get_user_move() -> String {
    let mut buffer = String::new();
    println!("What's your move?");
    match io::stdin().read_line(&mut buffer) {
        Err(e) => {
            println!("Something went wrong reading stdin: {}", e);
            std::process::exit(1);
        },
        _ => (),
    };
    buffer
}

fn clear_terminal() {
    print!("{}[2J", 27 as char);
}

fn convert_user_input(user_input: String) -> Result<(usize, usize), String> {
    let chars: Vec<char> = user_input.chars().collect();
    println!("{:?}", chars);
    if chars.len() != 3 {
        return Err(String::from("Must only input row and column"));
    }
    let row = chars[0].to_string().parse::<usize>().expect("Row inputed was not a number");
    let col = chars[1].to_string().parse::<usize>().expect("Row inputed was not a number");
    if row < 1 || row > 3 || col < 1 || col > 3 {
        return Err(String::from("Row and column values must be 1, 2, or 3."));
    }
    Ok((row, col))
}

fn main() {
    let mut board = Board::new();
    
    loop {
        clear_terminal();
        println!("{}", board);

        let mut user_input_valid = false;
        let mut row = 0;
        let mut col = 0;
        while !user_input_valid {
            let user_move = get_user_move();
            match convert_user_input(user_move) {
                Ok((r, c)) => {
                    user_input_valid = true;
                    row = r;
                    col = c;
                },
                Err(msg) => println!("{}", msg),
            }
        };

        board.update_space(row - 1, col - 1);
    }
}
