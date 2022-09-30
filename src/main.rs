use crate::task::Task;

fn main() {
    for _ in 0..=10 {
        let task = Task::gen();
        println!("{} = {}", task.as_string(), task.answer());
    }
}

pub mod math {
    #[derive(Debug)]
    pub enum MathOps {
        Add,
        Sub,
        Mul,
        Div,
    }

    impl MathOps {
        pub fn all() -> Vec<Self> {
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
}

pub mod generator {
    use crate::math::MathOps;
    use rand::Rng;
    use std::ops::Range;

    pub fn gen_num(range: Range<i8>) -> i8 {
        rand::thread_rng().gen_range(range)
    }

    pub fn gen_sign() -> MathOps {
        let mut signs: Vec<MathOps> = MathOps::all();
        let len: i8 = signs.len() as i8;
        let index: usize = gen_num(0..len) as usize;
        signs.remove(index)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_gen_sign() {
            for _ in 0..(MathOps::all().len() * 100) {
                gen_sign();
            }
        }
    }
}

mod task {
    use crate::generator::{gen_num, gen_sign};
    use crate::math::MathOps;
    use std::ops::Range;

    const NUM_RANGE: Range<i8> = -i8::MAX..i8::MAX;

    #[derive(Debug)]
    pub struct Task {
        pub a: i8,
        pub op: MathOps,
        pub b: i8,
        answer: i32,
    }

    impl Task {
        pub fn new(a: i8, op: MathOps, b: i8, answer: i32) -> Self {
            Task { a, op, b, answer }
        }

        pub fn gen() -> Task {
            let a: i8 = gen_num(NUM_RANGE);
            let op = gen_sign();
            let b: i8;
            let answer: i32;

            loop {
                let any: i8 = gen_num(NUM_RANGE);

                if let Ok(result) = op.join(a, any) {
                    b = any;
                    answer = result;
                    break;
                }
            }

            Task::new(a, op, b, answer)
        }

        pub fn as_string(&self) -> String {
            format!("{} {} {}", self.a, self.op.as_char(), self.b)
        }

        pub fn answer(&self) -> i32 {
            self.answer
        }
    }
}
