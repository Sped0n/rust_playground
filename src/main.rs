mod guessing_game;
mod hello_world;

#[allow(dead_code)]
fn hello_world_practice() {
    use hello_world::{hello_world as hw, hello_world_alt as hwa};
    hw();
    hwa();
}

#[allow(dead_code)]
fn guessing_game_practice() {
    use guessing_game::guessing_game;
    guessing_game();
}

fn main() {
    guessing_game_practice();
}
