extern crate ncurses;
use std::cell;

use ncurses::*;
use rand::thread_rng;
use rand::seq::SliceRandom;
const GRID_SIZE: usize = 4;

struct Game {
    board: Vec<Vec<String>>,
    empty_tile : (usize,usize),
    num_moves :u32,
}
fn is_solvable(initial_state: &Vec<i32>) -> bool {
    let mut inversions = 0;

    for i in 0..initial_state.len() {
        for j in i + 1..initial_state.len() {
            if initial_state[i] > initial_state[j] {
                inversions += 1;
            }
        }
    }
    inversions % 2 == 0
}
impl Game {
    
    fn new()->Game {

        let mut board = vec![vec![".".to_string(); GRID_SIZE]; GRID_SIZE];
        let mut initial_state: Vec<i32> = (1..16).collect();
        initial_state.shuffle(&mut thread_rng());
        while !is_solvable(&initial_state) {
            initial_state.shuffle(&mut thread_rng());
        }

        let mut cnt=0;
        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                if cnt < 15 {
                    board[i][j]=format!("{}",initial_state[cnt]);
                }
                cnt+=1;
            }
        }
        println!("initialized");
        Game { 
            board,
            empty_tile: (GRID_SIZE-1,GRID_SIZE-1),
            num_moves: 0,
        }
    }
    fn show_grid(&self){
        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                // print the board
                let cell_content = format!("{:width$}", self.board[row][col], width = 4);
                printw(&format!("{}", cell_content));
            }
            addch('\n' as chtype);
        }
        mvprintw(GRID_SIZE as i32 + 1, 0, &format!("Moves: {}", self.num_moves));
        refresh();
    }
    fn function_to_swap(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        let temp = self.board[x1][y1].clone();
        self.board[x1][y1] = self.board[x2][y2].clone();
        self.board[x2][y2] = temp;
    }
    fn move_up(&mut self) {
        if self.empty_tile.0 < GRID_SIZE - 1 {
            self.function_to_swap(self.empty_tile.0, self.empty_tile.1, self.empty_tile.0 + 1, self.empty_tile.1);
            self.empty_tile.0 += 1;
            self.num_moves += 1;
        }
    }

    fn move_down(&mut self) {
        if self.empty_tile.0 > 0 {
            self.function_to_swap(self.empty_tile.0, self.empty_tile.1, self.empty_tile.0 - 1, self.empty_tile.1);
            self.empty_tile.0 -= 1;
            self.num_moves += 1;
        }
    }

    fn move_left(&mut self) {
        if self.empty_tile.1 < GRID_SIZE - 1 {
            self.function_to_swap(self.empty_tile.0, self.empty_tile.1, self.empty_tile.0, self.empty_tile.1 + 1);
            self.empty_tile.1 += 1;
            self.num_moves += 1;
        }
    }

    fn move_right(&mut self) {
        if self.empty_tile.1 > 0 {
            self.function_to_swap(self.empty_tile.0, self.empty_tile.1, self.empty_tile.0, self.empty_tile.1 - 1);
            self.empty_tile.1 -= 1;
            self.num_moves += 1;
        }
    }
    fn did_i_win(&mut self)->bool {
        let mut bad=0;
        let mut cnt=1;
        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                if self.board[i][j]==format!("{}",cnt){
                    cnt+=1;
                }
                else if cnt<16 {
                    bad=1;
                }
            }
        }
        if bad == 0 {
            return true;
        }
        return false;
    }
    
}
fn main() {
    initscr();
    noecho(); 
    keypad(stdscr(), true);

    

    let mut game = Game::new();

    loop {
        clear();
        game.show_grid();
        refresh();

        if game.did_i_win() {
            printw("Congratulations! You won!");
            refresh();
            napms(2000);
            break;
        }

        let ch=getch();
        if ch == 'w' as i32{
            game.move_up();
        }
        if ch == 's' as i32{
            game.move_down();
        }
        if ch == 'a' as i32{
            game.move_left();
        }
        if ch == 'd' as i32{
            game.move_right();
        }
    }

    // End ncurses session
    endwin();
}
