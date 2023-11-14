use crate::gl_wrapper::renderbuffer::Renderbuffer;
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
    pub fn unbind(&self) { Self::bind_default() }
    pub fn bind_default() { unsafe { gl::BindFramebuffer(gl::FRAMEBUFFER, 0) } }

    pub fn set_clear_color(r: f32, g: f32, b: f32, a: f32) {
        unsafe { gl::ClearColor(r, g, b, a) }
    }
    pub fn clear_color() {
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) }
    }
    pub fn clear_color_depth() {
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT) }
    }
    pub fn enable_depth_test() { unsafe { gl::Enable(gl::DEPTH_TEST) } }
    pub fn disable_depth_test() { unsafe { gl::Disable(gl::DEPTH_TEST) } }

    pub fn attach_texture(&mut self, texture: &Texture, attachment: TextureAttachment) {
        self.bind();
        self.attach(&attachment);
        texture.attach_to_framebuffer(attachment);
    }
    pub fn attach_renderbuffer(&mut self, rbo: &Renderbuffer, attachment: TextureAttachment) {
        self.bind();
        self.attach(&attachment);
        rbo.attach_to_framebuffer(attachment);
    }
    fn attach(&mut self, attachment: &TextureAttachment) {
        match attachment {
            TextureAttachment::Color(idx) => self.color_textures.push(*idx),
            TextureAttachment::Depth => self.depth_texture = true,
            TextureAttachment::Stencil => self.stencil_texture = true,
            TextureAttachment::DepthStencil => self.depth_stencil_texture = true,
        }
    }

    pub fn bind_draw_buffers(&self) {
        unsafe {
            let mut buffers: Vec<u32> = self
                .color_textures
                .iter()
                .map(|i| gl::COLOR_ATTACHMENT0 + i)
                .collect();
            if self.depth_texture { buffers.push(gl::DEPTH_ATTACHMENT) }
            if self.stencil_texture { buffers.push(gl::STENCIL_ATTACHMENT) }
            if self.depth_stencil_texture { buffers.push(gl::DEPTH_STENCIL_ATTACHMENT) }
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
