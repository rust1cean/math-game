use crate::math::MathOps;
use std::ops::Range;

const NUM_RANGE: Range<i8> = -i8::MAX..i8::MAX;

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
