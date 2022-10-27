mod api;
use api::App;

fn main() {
    App::new().add_task(&home).run();
}

pub fn home(_: &mut App) {
    
}
