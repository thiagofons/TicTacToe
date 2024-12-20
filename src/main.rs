use macroquad::prelude::*;

#[derive(Debug, PartialEq, Clone)]
enum Players {
    Player1,
    Player2,
}

impl Players {
    fn color(&self) -> CellColors {
        match self {
            Players::Player1 => CellColors::Blue,
            Players::Player2 => CellColors::Green,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum CellColors {
    Purple,
    Green,
    Blue,
}

#[derive(Debug, PartialEq, Clone)]
enum CellPlayed {
    Yes(Players),
    No,
}

impl CellPlayed {
    fn color(&self) -> CellColors {
        match self {
            CellPlayed::Yes(p) => p.color(),
            CellPlayed::No => CellColors::Purple,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Cell {
    is_played: CellPlayed,
    color: macroquad::color::Color,
    x: f32,
    y: f32,
    size: f32,
}

impl Cell {
    fn new(x: f32, y: f32) -> Self {
        Self {
            is_played: CellPlayed::No,
            color: ttt_color_to_macroquad_color(CellPlayed::No.color()),
            x,
            y,
            size: 100f32,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Ttt {
    board: Vec<Cell>,
    player: Players,
}

impl Ttt {
    fn new() -> Self {
        Self {
            board: vec![
                Cell::new(100f32, 100f32),
                Cell::new(200f32, 100f32),
                Cell::new(300f32, 100f32),
                Cell::new(100f32, 200f32),
                Cell::new(200f32, 200f32),
                Cell::new(300f32, 200f32),
                Cell::new(100f32, 300f32),
                Cell::new(200f32, 300f32),
                Cell::new(300f32, 300f32),
            ],
            player: Players::Player1,
        }
    }

    fn play(&mut self, position: usize) -> bool {
        if position > 8 {
            return false;
        }
        if self.board[position].is_played != CellPlayed::No {
            return false;
        }
        self.board[position].is_played = CellPlayed::Yes(self.player.to_owned());
        self.board[position].color =
            ttt_color_to_macroquad_color(self.board[position].is_played.color());
        self.player = match self.player {
            Players::Player1 => Players::Player2,
            Players::Player2 => Players::Player1,
        };
        true
    }

    fn find_cell(&mut self, cell: &Cell) -> Option<&mut Cell> {
        let mut into_iter = self.board.iter_mut();
        into_iter.find(|x| *x == cell)
    }

    fn play_cell(&mut self, cell: Cell) -> bool {
        let player = self.player.to_owned();
        let color = player.color();
        let bcell = self.find_cell(&cell);
        match bcell {
            None => false,
            Some(x) => {
                if x.is_played != CellPlayed::No {
                    return true;
                }
                x.is_played = CellPlayed::Yes(player);
                x.color = ttt_color_to_macroquad_color(color);
                self.player = match self.player {
                    Players::Player1 => Players::Player2,
                    Players::Player2 => Players::Player1,
                };
                true
            }
        }
    }

    fn pos(&self, position: usize) -> &CellPlayed {
        if position > 8 {
            return &CellPlayed::No;
        }
        &self.board[position].is_played
    }

    fn show(&self) {
        println!("==========");
        println!(
            "{}  {}  {}",
            pos_to_string(self, 0),
            pos_to_string(self, 1),
            pos_to_string(self, 2)
        );
        println!(
            "{}  {}  {}",
            pos_to_string(self, 3),
            pos_to_string(self, 4),
            pos_to_string(self, 5)
        );
        println!(
            "{}  {}  {}",
            pos_to_string(self, 6),
            pos_to_string(self, 7),
            pos_to_string(self, 8)
        );
        println!("==========");
    }

    fn get_player(&self) -> &Players {
        &self.player
    }
}

fn pos_to_string(board: &Ttt, pos: usize) -> String {
    match board.pos(pos) {
        CellPlayed::No => "0".to_string(),
        CellPlayed::Yes(x) => match x {
            Players::Player1 => "1".to_string(),
            Players::Player2 => "2".to_string(),
        },
    }
}

fn player_to_string(p: &Players) -> String {
    match p {
        Players::Player1 => "1".to_string(),
        Players::Player2 => "2".to_string(),
    }
}

fn winner_player_based_on_current_player(board: &Ttt) -> &Players {
    match board.get_player() {
        Players::Player1 => &Players::Player2,
        Players::Player2 => &Players::Player1,
    }
}

fn is_victory(board: &Ttt) -> bool {
    // Todas as combinações vencedoras possíveis
    let winning_combinations = [
        [0, 1, 2], // Linhas
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6], // Colunas
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8], // Diagonais
        [2, 4, 6],
    ];

    for &combo in &winning_combinations {
        if board.pos(combo[0]) != &CellPlayed::No
            && board.pos(combo[0]) == board.pos(combo[1])
            && board.pos(combo[0]) == board.pos(combo[2])
        {
            return true;
        }
    }
    false
}


// impl<'a, 'b> PartialEq<

// fn calculate_win(b: Ttt) -> Vec<Ttt> {}

fn ttt_color_to_macroquad_color(cc: CellColors) -> macroquad::color::Color {
    match cc {
        CellColors::Purple => PURPLE,
        CellColors::Blue => BLUE,
        CellColors::Green => GREEN,
    }
}

fn draw_game(game: &Ttt) {
    clear_background(WHITE);
    for item in &game.board {
        draw_rectangle(item.x, item.y, item.size, item.size, item.color);
    }
    draw_text("RESTART", 500f32, 500f32, 70f32, BLACK);
}

fn on_top_of_cell((x, y): (f32, f32), cell: &Cell) -> bool {
    if x < cell.x || x > cell.x + cell.size {
        return false;
    }
    if y < cell.y || y > cell.y + cell.size {
        return false;
    }
    true
}

fn screen_pos_to_cell((x, y): (f32, f32), board: &Ttt) -> Option<Cell> {
    for item in &board.board {
        if on_top_of_cell((x, y), item) {
            return Some(item.clone());
        }
    }
    None
}

fn play_on_stdin(board: &mut Ttt) {
    use std::io::{stdin, stdout, Write};
    board.show();
    print!("Next play: ");
    let mut s = String::new();
    let _ = stdout().flush();
    while s.is_empty() {
        stdin().read_line(&mut s).expect("Incorrect string");
        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }
        if let Some('\r') = s.chars().next_back() {
            s.pop();
        }
    }
    if !board.play(s.parse::<usize>().unwrap() - 1) {
        println!("Problem playing it. Nothing happened");
    }
}

#[macroquad::main("TicTacToe")]
async fn main() {
    let mut board = Ttt::new();
    let stdin_play = false;

    loop {
        while !is_victory(&board) {
            if stdin_play {
                play_on_stdin(&mut board);
            } else {
                if is_mouse_button_pressed(MouseButton::Left) {
                    let cursor_position = mouse_position();
                    if (500f32..700f32).contains(&cursor_position.0)
                        && (450f32..500f32).contains(&cursor_position.1)
                    {
                        board = Ttt::new();
                    }
                    let cell_position = screen_pos_to_cell(cursor_position, &board);
                    match cell_position {
                        None => {}
                        Some(c) => {
                            board.play_cell(c);
                        }
                    }
                }
                if is_mouse_button_down(MouseButton::Right) {
                    let cursor_position = mouse_position();
                    let cell = screen_pos_to_cell(cursor_position, &board);
                    match cell {
                        None => {}
                        Some(cell) => match board.find_cell(&cell) {
                            None => {}
                            Some(c) => {
                                c.x = cursor_position.0 - cell.size / 2f32;
                                c.y = cursor_position.1 - cell.size / 2f32;
                            }
                        },
                    }
                }
                draw_game(&board);
                next_frame().await
            }
        }
        loop {
            draw_game(&board);
            draw_text(
                format!(
                    "Player {} won!",
                    player_to_string(winner_player_based_on_current_player(&board))
                )
                .as_str(),
                50f32,
                50f32,
                50f32,
                BLACK,
            );
            let mousepos = mouse_position();
            if (500f32..700f32).contains(&mousepos.0)
                && (450f32..500f32).contains(&mousepos.1)
                && is_mouse_button_pressed(MouseButton::Left)
            {
                board = Ttt::new();
                break;
            }
            next_frame().await
        }
        println!(
            "Player {} won!",
            player_to_string(winner_player_based_on_current_player(&board))
        );
    }
}
