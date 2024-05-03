use ashen_engine::logger;
use ashen_engine::window::Window;
fn main() {
    logger::init();

    let mut window: Window = Window::new(1080, 720, "test");
    window.init_gl();
    while !window.should_close() {
        window.update();
    }
}