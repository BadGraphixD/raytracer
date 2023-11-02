use crate::util::error::WindowError;
use crate::window::input::Input;
use glfw::{Action, Context, Glfw, WindowEvent, WindowHint};
use std::sync::mpsc::Receiver;

pub struct Window {
    glfw: Glfw,
    window_handle: glfw::Window,
    events: Receiver<(f64, WindowEvent)>,
    input: Input,
    width: u32,
    height: u32,
    resized: bool,
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str) -> Result<Self, WindowError> {
        use glfw::fail_on_errors;
        let mut glfw: Glfw = match glfw::init(fail_on_errors!()) {
            Ok(glfw) => glfw,
            Err(_) => return Err(WindowError::GlfwInitError),
        };

        let (mut window, events) =
            match glfw.create_window(width, height, title, glfw::WindowMode::Windowed) {
                Some(we) => we,
                None => return Err(WindowError::CreateWindowError),
            };

        window.make_current();

        glfw.default_window_hints();
        glfw.window_hint(WindowHint::ContextVersion(3, 3));
        glfw.window_hint(WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        glfw.window_hint(WindowHint::OpenGlForwardCompat(true));
        glfw.window_hint(WindowHint::Resizable(true));

        window.set_key_polling(true);
        window.set_mouse_button_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_size_polling(true);

        gl::load_with(|s| window.get_proc_address(s) as *const _);
        glfw.set_swap_interval(glfw::SwapInterval::None);

        Ok(Window {
            glfw,
            window_handle: window,
            events,
            input: Input::new(),
            width,
            height,
            resized: false,
        })
    }

    pub fn should_close(&self) -> bool {
        self.window_handle.should_close()
    }

    pub fn close(self) {
        self.window_handle.close()
    }

    pub fn handle_events(&mut self) {
        self.glfw.poll_events();
        self.resized = false;
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                WindowEvent::Key(key, _, action, _) => {
                    self.input.set_key_pressed(key, action != Action::Release)
                }
                WindowEvent::MouseButton(button, action, _) => self
                    .input
                    .set_button_pressed(button, action != Action::Release),
                WindowEvent::CursorPos(x, y) => self.input.set_cursor_pos(x as f32, y as f32),
                WindowEvent::Size(w, h) => {
                    let width = w as u32;
                    let height = h as u32;
                    if self.width != width || self.height != height {
                        self.width = width;
                        self.height = height;
                        self.resized = true;
                    }
                }
                _ => {}
            }
        }
    }

    pub fn update(&mut self) {
        self.window_handle.swap_buffers();
    }

    pub fn input(&self) -> &Input {
        &self.input
    }

    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn aspect(&self) -> f32 {
        self.width as f32 / self.height as f32
    }

    pub fn resized(&self) -> bool {
        self.resized
    }
}
