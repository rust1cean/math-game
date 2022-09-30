fn main() {}

pub mod math {
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

        pub fn join(&self, a: i32, b: i32) -> i32 {
            match self {
                MathOps::Add => a + b,
                MathOps::Sub => a - b,
                MathOps::Mul => a * b,
                MathOps::Div => a / b,
            }
        }
    }

    pub struct Task {
        pub a: i32,
        pub op: MathOps,
        pub b: i32,
    }

    impl Task {
        pub fn new(a: i32, op: MathOps, b: i32) -> Self {
            Task { a, op, b }
        }

        pub fn as_string(&self) -> String {
            format!("{} {} {}", self.a, self.op.as_char(), self.b)
        }

        pub fn answer(&self) -> i32 {
            self.op.join(self.a, self.b)
        }
    }
}

pub mod generator {
    use crate::math::{MathOps, Task};
    use rand::Rng;
    use std::ops::Range;

    const NUM_RANGE: Range<i32> = -i32::MAX..i32::MAX;

    pub fn gen_num(range: Range<i32>) -> i32 {
        rand::thread_rng().gen_range(range)
    }

    pub fn gen_sign() -> MathOps {
        let mut signs: Vec<MathOps> = MathOps::all();
        let len: i32 = signs.len() as i32;
        let index: usize = gen_num(0..len) as usize;
        signs.remove(index)
    }

    pub fn gen_task() -> Task {
        let (a, b) = (gen_num(NUM_RANGE), gen_num(NUM_RANGE));
        let op = gen_sign();

        Task::new(a, op, b)
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
