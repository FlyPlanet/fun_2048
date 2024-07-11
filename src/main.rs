use rand::*;
use std::fmt::Display;

use cstr::cstr;
use num_enum::TryFromPrimitive;
use qmetaobject::prelude::*;
#[derive(PartialEq, TryFromPrimitive)]
#[repr(u8)]
enum Direction {
    Up = 1,
    Down = 2,
    Left = 3,
    Right = 4,
}
#[derive(Default)]
pub struct Game {
    board: [[i32; 4]; 4],
    score: i32,
    availble_postions: i32,
    gameover: bool,
}
impl Game {
    pub fn new() -> Self {
        let mut g = Game {
            board: [[0; 4]; 4],
            score: 0,
            availble_postions: 16,
            gameover: false,
        };
        g.new_number();
        g.new_number();
        g
    }
    fn new_number(&mut self) {
        if self.gameover || self.availble_postions == 0 {
            self.gameover = true;
            return;
        }
        let mut rng = rand::thread_rng();
        let mut i = rng.gen_range(0..4);
        let mut j = rng.gen_range(0..4);
        while self.board[i][j] != 0 {
            i = rng.gen_range(0..4);
            j = rng.gen_range(0..4);
        }

        self.board[i][j] = rng.gen_range(1..=2) * 2;
        self.availble_postions -= 1;
    }
    fn move_line(&mut self, line: &mut [i32], to_upper: bool, moved: &mut bool) {
        if to_upper {
            line.reverse();
        }
        let mut offset: usize = 0;
        let mut previous: usize = usize::MAX;
        let mut flag=false;
        for i in 0..line.len() {
            if flag&&line[i]!=0{
                *moved = true;

            }
            
            if line[i] == 0 {
                offset += 1;
                flag=true;

            } else {
                if previous == usize::MAX {
                    line.swap(i, i - offset);
                    previous = i - offset;
                } else {
                    if line[i] == line[previous] {
                        line[previous] *= 2;
                        self.score += line[previous];
                        line[i] = 0;
                        previous = usize::MAX;
                        offset += 1;
                        self.availble_postions += 1;
                        *moved = true;
                    } else {
                        line.swap(i, i - offset);
                        previous = i - offset;
                    }
                }
            }
        }
        if to_upper {
            line.reverse();
        }
    }
    fn move_d(&mut self, direction: Direction) {
        let mut line_: [i32; 4];
        let mut moved = false;
        match direction {
            Direction::Up => {
                for i in 0..4 {                       
                    line_ = [
                        self.board[0][i],
                        self.board[1][i],
                        self.board[2][i],
                        self.board[3][i],
                    ];
                    self.move_line(&mut line_, false, &mut moved);
                    for j in 0..4 {
                        self.board[j][i] = line_[j];
                    }
                }
            }
            Direction::Down => {
                for i in 0..4 {
                    line_ = [
                        self.board[0][i],
                        self.board[1][i],
                        self.board[2][i],
                        self.board[3][i],
                    ];
                    self.move_line(&mut line_, true, &mut moved);
                    for j in 0..4 {
                        self.board[j][i] = line_[j];
                    }
                }
            }
            Direction::Left => {
                for i in 0..4 {
                    line_ = [
                        self.board[i][0],
                        self.board[i][1],
                        self.board[i][2],
                        self.board[i][3],
                    ];
                    self.move_line(&mut line_, false, &mut moved);
                    for j in 0..4 {
                        self.board[i][j] = line_[j];
                    }
                }
            }
            Direction::Right => {
                for i in 0..4 {
                    line_ = [
                        self.board[i][0],
                        self.board[i][1],
                        self.board[i][2],
                        self.board[i][3],
                    ];
                    self.move_line(&mut line_, true, &mut moved);
                    for j in 0..4 {
                        self.board[i][j] = line_[j];
                    }
                }
            }
        }
        if moved {
            self.new_number();
        }
    }
    fn _gameover(&mut self) -> bool {
        self.gameover
    }
}
impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //先打印棋盘 再是分数
        for i in 0..4 {
            for j in 0..4 {
                write!(f, "{}  ", self.board[i][j])?;
            }
            writeln!(f)?;
        }
        write!(
            f,
            "Score: {} with availble postions: {}",
            self.score, self.availble_postions
        )
    }
}
// fn save_to_csv(grade:&HashMap<i128,i32>){
//     #[derive(serde::Serialize)]
//     struct Record{
//         key:String,
//         value:String,
//     }
//     let file=std::fs::File::create("./grade.csv").unwrap();
//     let mut csv=csv::Writer::from_writer(file);
//     for (key,value) in grade{
//         csv.serialize(Record{key:key.to_string(),value:value.to_string()}).unwrap();
//     }
//     csv.flush().unwrap();
// }
#[derive(Default, QObject)]
pub struct App {
    base: qt_base_class!(trait QObject),
    game: Game,
    get_score: qt_method!(
        fn get_score(&self) -> i32 {
            self.game.score
        }
    ),
    move_d: qt_method!(
        fn move_d(&mut self, d: i32) {
            if d < 1 || d > 4 {
                return;
            }
            self.game.move_d(Direction::try_from(d as u8).unwrap());
            self.game_changed();
            self.score = self.game.score;
        }
    ),
    game_changed: qt_signal!(),
    init_game: qt_method!(
        fn init_game(&mut self) {
            self.game = Game::new();
            println!("Game inited \n{}", self.game);
            self.score = self.game.score
        }
    ),
    get_board: qt_method!(
        fn get_board(&self, x: i32) -> QVariant {
            if x < 0 || x > 15 {
                return 1919810.into();
            }
            let i = x / 4;
            let j = x % 4;
            self.game.board[i as usize][j as usize].into()
        }
    ),
    score: qt_property!(i32;READ get_score),
}

fn main() {
    qml_register_type::<App>(cstr!("Rustcode"), 1, 0, cstr!("App"));
    let mut engine = QmlEngine::new();
    engine.load_file("./qml/App.qml".into());
    engine.exec();
}
