use crate::gl_wrapper::texture::Texture;
use crate::gl_wrapper::types::TextureAttachment;
use crate::util::error::FramebufferError;

fn gen_framebuffer() -> u32 {
    let mut id: u32 = 0;
    unsafe { gl::GenFramebuffers(1, &mut id) }
    id
}

pub struct Framebuffer {
    fbo: u32,
}

impl Framebuffer {
    pub fn new() -> Self {
        Self { fbo: gen_framebuffer() }
    }

    pub fn bind(&self) {
        unsafe { gl::BindFramebuffer(gl::FRAMEBUFFER, self.fbo) }
    }
    pub fn unbind(&self) {
        unsafe { gl::BindFramebuffer(gl::FRAMEBUFFER, 0) }
    }

    pub fn attach_texture(&self, texture: &Texture, attachment: TextureAttachment) {
        self.bind();
        texture.attach_to_framebuffer(attachment);
    }

    pub fn check_status(&self) -> Result<(), FramebufferError> {
        unsafe {
            match gl::CheckFramebufferStatus(gl::FRAMEBUFFER) {
                gl::FRAMEBUFFER_COMPLETE => Ok(()),
                code => Err(FramebufferError::Error(code)),
            }
        }
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        unsafe { gl::DeleteFramebuffers(1, &self.fbo) }
    }
}