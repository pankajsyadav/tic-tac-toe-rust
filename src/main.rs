use std::io;

#[derive(Clone, Copy, PartialEq)]
enum Player {
    X,
    O,
}

impl Player {
    fn symbol(&self) -> char {
        match self {
            Player::X => 'X',
            Player::O => 'O',
        }
    }

    fn next(&self) -> Player {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

struct Board {
    cells: [[Option<Player>; 3]; 3],
}

impl Board {
    fn new() -> Self {
        Board { cells: [[None; 3]; 3] }
    }

    fn print(&self) {
        println!("\n   0   1   2");
        for (i, row) in self.cells.iter().enumerate() {
            print!(" {} ", i);
            for (j, cell) in row.iter().enumerate() {
                let symbol = match cell {
                    Some(p) => p.symbol(),
                    None => ' ',
                };
                print!(" {} ", symbol);
                if j < 2 {
                    print!("|");
                }
            }
            println!();
            if i < 2 {
                println!("   ---------");
            }
        }
        println!();
    }

    fn make_move(&mut self, row: usize, col: usize, player: Player) -> Result<(), &'static str> {
        if row > 2 || col > 2 {
            return Err("Row and column must be between 0 and 2");
        }
        if self.cells[row][col].is_some() {
            return Err("Cell already occupied");
        }
        self.cells[row][col] = Some(player);
        Ok(())
    }

    fn check_winner(&self) -> Option<Player> {
        // Check rows
        for row in &self.cells {
            if row[0] == row[1] && row[1] == row[2] {
                if let Some(p) = row[0] {
                    return Some(p);
                }
            }
        }

        // Check columns
        for col in 0..3 {
            if self.cells[0][col] == self.cells[1][col] && self.cells[1][col] == self.cells[2][col] {
                if let Some(p) = self.cells[0][col] {
                    return Some(p);
                }
            }
        }

        // Check diagonals
        if self.cells[0][0] == self.cells[1][1] && self.cells[1][1] == self.cells[2][2] {
            if let Some(p) = self.cells[0][0] {
                return Some(p);
            }
        }
        if self.cells[0][2] == self.cells[1][1] && self.cells[1][1] == self.cells[2][0] {
            if let Some(p) = self.cells[0][2] {
                return Some(p);
            }
        }

        None
    }

    fn is_full(&self) -> bool {
        self.cells.iter().all(|row| row.iter().all(|cell| cell.is_some()))
    }
}

fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn main() {
    println!("=== Tic Tac Toe ===");
    println!("Players take turns entering row and column (0-2)");

    let mut board = Board::new();
    let mut current_player = Player::X;
    let mut moves = 0;

    loop {
        board.print();
        println!("Player {}'s turn", current_player.symbol());

        let row_str = read_input("Enter row (0-2):");
        let col_str = read_input("Enter column (0-2):");

        let row: usize = match row_str.parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid input. Please enter a number between 0 and 2.");
                continue;
            }
        };

        let col: usize = match col_str.parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid input. Please enter a number between 0 and 2.");
                continue;
            }
        };

        match board.make_move(row, col, current_player) {
            Ok(_) => {
                moves += 1;
                if let Some(winner) = board.check_winner() {
                    board.print();
                    println!("Player {} wins!", winner.symbol());
                    break;
                }
                if board.is_full() {
                    board.print();
                    println!("It's a draw!");
                    break;
                }
                current_player = current_player.next();
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    println!("Game over!");
}
