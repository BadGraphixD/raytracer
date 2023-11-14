use crate::gl_wrapper::types::{TextureAttachment, TextureFormat};

fn gen_renderbuffer() -> u32 {
    let mut id: u32 = 0;
    unsafe { gl::GenRenderbuffers(1, &mut id) }
    id
}

fn reformat(rbo: u32, width: u32, height: u32, format: &TextureFormat) {
    unsafe {
        gl::BindRenderbuffer(gl::RENDERBUFFER, rbo);
        gl::RenderbufferStorage(
            gl::RENDERBUFFER,
            format.to_gl_internal(),
            width as i32,
            height as i32,
        );
    }
}

pub struct Renderbuffer {
    rbo: u32,
    width: u32,
    height: u32,
    format: TextureFormat,
}

impl Renderbuffer {
    pub fn new(width: u32, height: u32, format: TextureFormat) -> Self {
        let rbo = gen_renderbuffer();
        reformat(rbo, width, height, &format);
        Self {
            rbo,
            width,
            height,
            format,
        }
    }

    pub fn bind(&self) { unsafe { gl::BindRenderbuffer(gl::FRAMEBUFFER, self.rbo) } }
    pub fn unbind(&self) { unsafe { gl::BindRenderbuffer(gl::FRAMEBUFFER, 0) } }

    pub fn reformat(&mut self, width: u32, height: u32, format: TextureFormat) {
        if self.width != width || self.height != height || self.format != format {
            reformat(self.rbo, width, height, &format);
            self.width = width;
            self.height = height;
            self.format = format;
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.reformat(width, height, self.format.clone());
    }

    pub fn attach_to_framebuffer(&self, attachment: TextureAttachment) {
        self.bind();
        unsafe {
            gl::FramebufferRenderbuffer(
                gl::FRAMEBUFFER,
                attachment.to_gl_internal(),
                gl::RENDERBUFFER,
                self.rbo,
            );
        }
    }
}

impl Drop for Renderbuffer {
    fn drop(&mut self) {
        unsafe { gl::DeleteRenderbuffers(1, &self.rbo) }
    }
}