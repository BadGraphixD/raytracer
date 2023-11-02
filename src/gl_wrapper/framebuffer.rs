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
    color_textures: Vec<u32>,
    depth_texture: bool,
    stencil_texture: bool,
    depth_stencil_texture: bool,
}

impl Framebuffer {
    pub fn new() -> Self {
        Self {
            fbo: gen_framebuffer(),
            color_textures: vec![],
            depth_texture: false,
            stencil_texture: false,
            depth_stencil_texture: false,
        }
    }

    pub fn bind(&self) {
        unsafe { gl::BindFramebuffer(gl::FRAMEBUFFER, self.fbo) }
    }
    pub fn unbind(&self) {
        unsafe { gl::BindFramebuffer(gl::FRAMEBUFFER, 0) }
    }

    pub fn attach_texture(&mut self, texture: &Texture, attachment: TextureAttachment) {
        self.bind();
        match attachment {
            TextureAttachment::Color(idx) => self.color_textures.push(idx),
            TextureAttachment::Depth => self.depth_texture = true,
            TextureAttachment::Stencil => self.stencil_texture = true,
            TextureAttachment::DepthStencil => self.depth_stencil_texture = true,
        }
        texture.attach_to_framebuffer(attachment);
    }

    pub fn bind_draw_buffers(&self) {
        unsafe {
            let mut buffers: Vec<u32> = self
                .color_textures
                .iter()
                .map(|i| gl::COLOR_ATTACHMENT0 + i)
                .collect();
            if self.depth_texture {
                buffers.push(gl::DEPTH_ATTACHMENT)
            }
            if self.stencil_texture {
                buffers.push(gl::STENCIL_ATTACHMENT)
            }
            if self.depth_stencil_texture {
                buffers.push(gl::DEPTH_STENCIL_ATTACHMENT)
            }
            gl::DrawBuffers(buffers.len() as i32, buffers.as_ptr());
        }
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
