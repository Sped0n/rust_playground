mod hello_world;

fn hello_world_practice() {
    use hello_world::{hello_world as hw, hello_world_alt as hwa};
    hw();
    hwa();
}

fn main() {
    hello_world_practice();
}
