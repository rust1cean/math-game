use rand::Rng;
use std::ops::Range;

const NUM_RANGE: Range<i8> = -i8::MAX..i8::MAX;

#[derive(Debug)]
pub enum MathOps {
    Add,
    Sub,
    Mul,
    Div,
}

impl MathOps {
    pub fn all() -> Vec<MathOps> {
        vec![Self::Add, Self::Sub, Self::Mul, Self::Div]
    }

    pub fn as_char(&self) -> char {
        match self {
            MathOps::Add => '+',
            MathOps::Sub => '-',
            MathOps::Mul => '*',
            MathOps::Div => '/',
        }
    }

    pub fn join(&self, a: i8, b: i8) -> Result<i32, &'static str> {
        match self {
            MathOps::Add => Ok(a as i32 + b as i32),
            MathOps::Sub => Ok(a as i32 - b as i32),
            MathOps::Mul => Ok(a as i32 * b as i32),
            MathOps::Div => Ok(a as i32 / b as i32),
        }
    }
}

pub trait Generator {
    fn gen_num(range: Range<i8>) -> i8 {
        rand::thread_rng().gen_range(range)
    }

    fn gen_sign() -> MathOps {
        let mut signs: Vec<MathOps> = MathOps::all();
        let len: i8 = signs.len() as i8;
        let index: usize = Self::gen_num(0..len) as usize;
        
        signs.remove(index)
    }
}

impl Generator for MathOps {}

#[derive(Debug)]
pub struct MathTask {
    pub a: i8,
    pub op: MathOps,
    pub b: i8,
    answer: i32,
}

impl MathTask {
    pub fn new(a: i8, op: MathOps, b: i8, answer: i32) -> Self {
        MathTask { a, op, b, answer }
    }

    pub fn gen() -> MathTask {
        let a: i8 = MathOps::gen_num(NUM_RANGE);
        let op: MathOps = MathOps::gen_sign();
        let b: i8;
        let answer: i32;

        loop {
            let any: i8 = MathOps::gen_num(NUM_RANGE);

            if let Ok(result) = op.join(a, any) {
                b = any;
                answer = result;
                break;
            }
        }

        MathTask::new(a, op, b, answer)
    }

    pub fn as_string(&self) -> String {
        let a: i8 = self.a;
        let op: char = self.op.as_char();
        let b: i8 = self.b;

        format!("{a} {op} {b}")
    }

    pub fn as_string_with_answer(&self) -> String {
        let a: i8 = self.a;
        let op: char = self.op.as_char();
        let b: i8 = self.b;
        let answer: i32 = self.answer;

        format!("{a} {op} {b} {answer}")
    }

    pub fn answer(&self) -> i32 {
        self.answer
    }
}
