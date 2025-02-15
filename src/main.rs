mod formulas;

use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::disable_raw_mode;

use formulas::*;

fn main() {
    let _res = enable_raw_mode();
    init_question_random_seed();
    let q: Question = Question::from(QUADRATIC_EASY);
    q.print();
    let _ = disable_raw_mode();
}
