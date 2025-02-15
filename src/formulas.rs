use std::io::stdout;
use std::io::Write;
use std::os::raw::c_char;
use std::time::SystemTime;

extern "C" {
    fn init_question(this: *mut Question, t: i32) -> i32;
    fn verify(q: *const Question, given: *const f32) -> i32;
    fn init_random_seed(seed: u64);
}

pub const LINEAR: i32 = 97;
pub const QUADRATIC_EASY: i32 = 98;
pub const QUADRATIC_HARD: i32 = 99;
pub const LINEAR_TWO: i32 = 100;
pub const LINEAR_THREE: i32 = 101;

#[repr(C)]
pub struct Question {
    pub prompt: [c_char; 91],
    pub ans: [f32; 9],
    pub cnt: u32
}

impl Question {
    pub fn from(t: i32) -> Question {
        let mut q: Question = Question{prompt: [0; 91], ans: [0.0; 9], cnt: 0};
        unsafe {
            init_question(&mut q as *mut Question, t);
        }
        q
    }
    pub fn verify(&self, given: &[f32]) -> bool {
        unsafe {
            verify(self as *const Question, given.as_ptr()) == 1
        }
    }
    pub fn print(&self) {
        let mut arr: Vec<u8> = Vec::new();
        for v in self.prompt.iter() {
            if *v == 0 {
                break;
            }
            arr.push(*v as u8);
        }
        arr.push(10);
        stdout().write(arr.as_slice());
    }
}

pub fn init_question_random_seed() {
    let curr = SystemTime::now();
    let dura = curr.duration_since(SystemTime::UNIX_EPOCH);
    unsafe {
        init_random_seed(dura.unwrap().as_secs());
    }
}
