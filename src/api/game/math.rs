use rand::Rng;
use std::ops::Range;

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

    pub fn gen_num(range: Range<i8>) -> i8 {
        rand::thread_rng().gen_range(range)
    }

    // Generates any math operation from MathOps enum
    pub fn gen_sign() -> MathOps {
        let mut signs: Vec<MathOps> = MathOps::all();
        let len: i8 = signs.len() as i8;
        let index: usize = Self::gen_num(0..len) as usize;
        signs.remove(index)
    }
}
