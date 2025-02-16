mod formulas;

use std::collections::HashSet;
use std::io::Bytes;
use std::io::Read;
use std::io::Stdin;
use std::io::stdin;
use std::io::stdout;

use crossterm::cursor::*;
use crossterm::execute;
use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::disable_raw_mode;
use crossterm::style::*;

use formulas::*;

fn parse_esc(it: &mut Bytes<Stdin>) -> u8 {
    match it.next() {
        Some(res) => match res {
            Ok(91) => {
                match it.next() {
                    Some(res2) => match res2 {
                        Ok(65) => {
                            return 65;
                        }
                        Ok(66) => {
                            return 66;
                        }
                        Ok(_) => {}
                        Err(_) => {}
                    }
                    None => {}
                }
            }
            Ok(_) => {}
            Err(_) => {}
        }
        None => {}
    }
    return 0;
}

fn main() {
    let _res = enable_raw_mode();
    init_question_random_seed();

    let options = [LINEAR, QUADRATIC_EASY, QUADRATIC_HARD, LINEAR_TWO, LINEAR_THREE];
    let mut optind: usize = 0;

    let opttext = [
        " Linear Equation One Unknown\r".to_string(),
        " Quadratic Equation One Unknown, Leading Coeficient 1\r".to_string(),
        " Quadratic Equation One Unknown\r".to_string(),
        " Linear Equation Two Unknown\r".to_string(),
        " Linear Equation Three Unknown\r".to_string()
    ];

    println!("Select the types of equations using arrow keys and space bar. Press enter to start.\r");
    let mut initer = stdin().bytes();
    let mut selected: HashSet<usize> = HashSet::new();

    loop {

        for (i, v) in opttext.iter().enumerate() {

            if i == optind {
                execute!(stdout(), SetBackgroundColor(Color::White), SetForegroundColor(Color::Black));
            }

            if selected.contains(&i) {
                print!("[*]");
            } else {
                print!("[ ]");
            }
            
            if i == optind {
                execute!(stdout(), ResetColor);
            }
            println!("{}", *v);

        }

        match initer.next() {
            Some(res) => match res {
                Ok(13) => {
                    break;
                }
                Ok(27) => {
                    let v = parse_esc(&mut initer);
                    if v == 65 {
                        if optind > 0 {
                            optind -= 1;
                        }
                    } else if v == 66 {
                        if options.len() - 1 > optind {
                            optind += 1;
                        }
                    }
                }
                Ok(32) => {
                    if !selected.remove(&optind) {
                        selected.insert(optind);
                    }
                }
                Ok(ch) => {}
                Err(_) => {}
            }
            None => {}
        }

        execute!(stdout(), MoveUp(options.len() as u16));

    }

    execute!(stdout(), MoveDown(options.len() as u16));
    
    for i in selected.iter().cycle() {
        
        let q: Question = Question::from(options[*i]);
        q.print();

        match initer.next() {
            Some(res) => match res {
                Ok(13) => {
                }
                Ok(27) => {
                    break;
                }
                Ok(32) => {
                }
                Ok(ch) => {}
                Err(_) => {}
            }
            None => {}
        }

    }

    let _ = disable_raw_mode();
}
