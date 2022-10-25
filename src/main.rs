use math_game::math_task::MathTask;

fn main() {
    for _ in 0..=10 {
        let task: MathTask = MathTask::gen();
        println!("{}", task.as_string_with_answer());
    }
}
