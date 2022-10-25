use math_game::{input, math_task::MathTask};

fn main() {
    let task: MathTask = MathTask::gen();

    println!("Print the answer to the task: {}", task.as_string());
    let answer: String = input::get_input();
    let answer: i32 = answer.trim().parse::<i32>().unwrap();

    assert_eq!(task.answer(), answer);
}
