use rand::Rng;
use std::io::{self, Write};

#[derive(Clone)]
struct Cell {
    mine: bool,
    revealed: bool,
    flagged: bool,
    adjacent: u8,
}

impl Cell {
    fn new() -> Self {
        Self {
            mine: false,
            revealed: false,
            flagged: false,
            adjacent: 0,
        }
    }
}

struct Board {
    width: usize,
    height: usize,
    mine_count: usize,
    cells: Vec<Vec<Cell>>,
    game_over: bool,
}

impl Board {
    fn new(width: usize, height: usize, mine_count: usize) -> Self {
        let mut board = Self {
            width,
            height,
            mine_count,
            cells: vec![vec![Cell::new(); width]; height],
            game_over: false,
        };
        board.place_mines();
        board.calculate_adjacency();
        board
    }

    fn place_mines(&mut self) {
        let mut rng = rand::thread_rng();
        let mut placed = 0;
        while placed < self.mine_count {
            let x = rng.gen_range(0..self.width);
            let y = rng.gen_range(0..self.height);
            if !self.cells[y][x].mine {
                self.cells[y][x].mine = true;
                placed += 1;
            }
        }
    }

    fn calculate_adjacency(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.cells[y][x].mine {
                    continue;
                }
                let mut count = 0;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        let nx = x as isize + dx;
                        let ny = y as isize + dy;
                        if nx >= 0
                            && nx < self.width as isize
                            && ny >= 0
                            && ny < self.height as isize
                            && self.cells[ny as usize][nx as usize].mine
                        {
                            count += 1;
                        }
                    }
                }
                self.cells[y][x].adjacent = count;
            }
        }
    }

    fn reveal(&mut self, x: usize, y: usize) {
        if x >= self.width || y >= self.height {
            println!("Coordinates out of bounds.");
            return;
        }
        if self.cells[y][x].revealed || self.cells[y][x].flagged {
            return;
        }
        self.cells[y][x].revealed = true;
        if self.cells[y][x].mine {
            self.game_over = true;
        } else if self.cells[y][x].adjacent == 0 {
            // Flood fill for blank cells.
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let nx = x as isize + dx;
                    let ny = y as isize + dy;
                    if nx >= 0 && nx < self.width as isize && ny >= 0 && ny < self.height as isize {
                        if !self.cells[ny as usize][nx as usize].revealed {
                            self.reveal(nx as usize, ny as usize);
                        }
                    }
                }
            }
        }
    }

    fn toggle_flag(&mut self, x: usize, y: usize) {
        if x >= self.width || y >= self.height {
            println!("Coordinates out of bounds.");
            return;
        }
        if self.cells[y][x].revealed {
            return;
        }
        self.cells[y][x].flagged = !self.cells[y][x].flagged;
    }

    fn display(&self) {
        // Print column headers.
        print!("   ");
        for x in 0..self.width {
            print!("{:2} ", x);
        }
        println!();

        // Print each row.
        for y in 0..self.height {
            print!("{:2} ", y);
            for x in 0..self.width {
                let cell = &self.cells[y][x];
                let symbol = if cell.revealed {
                    if cell.mine {
                        "*".to_string()
                    } else if cell.adjacent > 0 {
                        cell.adjacent.to_string()
                    } else {
                        " ".to_string()
                    }
                } else if cell.flagged {
                    "F".to_string()
                } else {
                    "#".to_string()
                };
                print!(" {} ", symbol);
            }
            println!();
        }
    }

    fn all_safe_cells_revealed(&self) -> bool {
        for row in &self.cells {
            for cell in row {
                if !cell.mine && !cell.revealed {
                    return false;
                }
            }
        }
        true
    }
}

fn main() {
    // You can adjust the board dimensions and mine count here.
    let width = 9;
    let height = 9;
    let mine_count = 10;
    let mut board = Board::new(width, height, mine_count);

    println!("Welcome to Minesweeper!");
    println!("Commands:");
    println!("  r x y  => Reveal cell at (x, y)");
    println!("  f x y  => Toggle flag at (x, y)\n");

    while !board.game_over {
        board.display();
        print!("Enter command: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Failed to read input.");
            continue;
        }
        let args: Vec<&str> = input.trim().split_whitespace().collect();
        if args.len() != 3 {
            println!("Invalid command format. Use r/f x y");
            continue;
        }
        let command = args[0];
        let x: usize = match args[1].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid x coordinate.");
                continue;
            }
        };
        let y: usize = match args[2].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid y coordinate.");
                continue;
            }
        };

        match command {
            "r" => board.reveal(x, y),
            "f" => board.toggle_flag(x, y),
            _ => println!("Unknown command. Use 'r' to reveal or 'f' to flag."),
        }

        if board.all_safe_cells_revealed() {
            println!("\nCongratulations! You've cleared the minefield!");
            board.display();
            return;
        }
    }

    board.display();
    println!("\nBoom! You hit a mine. Game Over.");
}