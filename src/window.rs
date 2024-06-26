use std::sync::mpsc::Receiver;

use glfw::{Context, Glfw, Key};

pub struct Window {
    pub glfw: glfw::Glfw,
    window_handle: glfw::Window,
    events: Receiver<(f64, glfw::WindowEvent)>,
}

pub enum WindowMode {
    Windowed,
    Fullscreen,
    Borderless,
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str, mode: WindowMode) -> Window {
        let mut glfw: Glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        let (mut window, events) = glfw
            .with_primary_monitor(|glfw, monitors| match mode {
                WindowMode::Windowed => {
                    glfw.create_window(width, height, title, glfw::WindowMode::Windowed)
                }
                WindowMode::Fullscreen => glfw.create_window(
                    width,
                    height,
                    title,
                    monitors.map_or(glfw::WindowMode::Windowed, |m| {
                        glfw::WindowMode::FullScreen(m)
                    }),
                ),
                WindowMode::Borderless => todo!(),
            })
            .expect("Failed to create glfw window");

        window.set_framebuffer_size_polling(true);
        window.set_key_polling(true);

        Window {
            glfw,
            window_handle: window,
            events,
        }
    }

    pub fn init_gl(&mut self) {
        self.window_handle.make_current();
        gl::load_with(|s| self.window_handle.get_proc_address(s) as *const _);
    }

    pub fn should_close(&self) -> bool {
        self.window_handle.should_close()
    }

    pub fn update(&mut self) {
        self.process_events();
        self.glfw.poll_events();
        self.window_handle.swap_buffers();
    }

    fn process_events(&mut self) {
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                    gl::Viewport(0, 0, width, height)
                },
                glfw::WindowEvent::Key(Key::Escape, _, glfw::Action::Press, _) => {
                    self.window_handle.set_should_close(true)
                }
                _ => {}
            }
        }
    }
}
