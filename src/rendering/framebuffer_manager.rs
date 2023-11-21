use std::sync::{Arc, Mutex};
use crate::gl_wrapper::framebuffer::Framebuffer;
use crate::gl_wrapper::renderbuffer::Renderbuffer;
use crate::gl_wrapper::texture::Texture;
use crate::gl_wrapper::types::{TextureAttachment, TextureFilter, TextureFormat};
use crate::window::window::Window;

pub struct FramebufferManager {
    window: Arc<Mutex<Window>>,
    framebuffers: Vec<Framebuffer>,
    textures: Vec<Texture>,
    renderbuffers: Vec<Renderbuffer>,
}

impl FramebufferManager {
    pub fn new(window: Arc<Mutex<Window>>) -> Self {
        Self {
            window,
            framebuffers: vec![],
            textures: vec![],
            renderbuffers: vec![],
        }
    }

    pub fn new_framebuffer(&mut self) -> usize {
        self.framebuffers.push(Framebuffer::new());
        self.framebuffers.len() - 1
    }

    pub fn attach_texture(&mut self, format: TextureFormat, attachment: TextureAttachment, attach: bool) -> usize {
        let window = self.window.lock().unwrap();
        self.framebuffers.last_mut().unwrap().bind();
        let texture = Texture::new(window.width(), window.height(), format, TextureFilter::Nearest);
        self.framebuffers.last_mut().unwrap().attach_texture(&texture, attachment, attach);
        self.textures.push(texture);
        self.textures.len() - 1
    }

    pub fn attach_renderbuffer(&mut self, format: TextureFormat, attachment: TextureAttachment, attach: bool) -> usize {
        let window = self.window.lock().unwrap();
        let rbo = Renderbuffer::new(window.width(), window.height(), format);
        self.framebuffers.last_mut().unwrap().attach_renderbuffer(&rbo, attachment, attach);
        self.renderbuffers.push(rbo);
        self.renderbuffers.len() - 1
    }

    pub fn build_framebuffers(&self) {
        self.framebuffers.iter().for_each(|fbo| {
            fbo.bind();
            fbo.bind_draw_buffers();
        })
    }

    pub fn update_buffers(&mut self) {
        let window = self.window.lock().unwrap();
        self.textures.iter_mut().for_each(|tex| tex.resize(window.width(), window.height()));
        self.renderbuffers.iter_mut().for_each(|rbo| rbo.resize(window.width(), window.height()));
    }

    pub fn bind_fbo(&self, handle: usize) {
        self.framebuffers[handle].bind();
    }

    pub fn bind_tex_to_slot(&self, handle: usize, slot: u32) -> u32 {
        self.textures[handle].bind_to_slot(slot)
    }
}