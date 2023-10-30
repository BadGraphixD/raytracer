use crate::gl_wrapper::types::{TextureAttachment, TextureFormat};

fn gen_texture() -> u32 {
    let mut id: u32 = 0;
    unsafe { gl::GenTextures(1, &mut id) }
    id
}

fn reformat(texture: u32, width: u32, height: u32, format: &TextureFormat) {
    unsafe {
        gl::BindTexture(gl::TEXTURE_2D, texture);
        gl::TexImage2D(gl::TEXTURE_2D, 0,
                       format.to_gl_internal() as i32,
                       width as i32, height as i32,
                       0, gl::RGB, gl::UNSIGNED_BYTE,
                       0 as *const _
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
    }
}

pub struct Texture {
    texture: u32,
    width: u32,
    height: u32,
    format: TextureFormat,
}

impl Texture {
    pub fn new(width: u32, height: u32, format: TextureFormat) -> Self {
        let texture = gen_texture();
        reformat(texture, width, height, &format);
        Self { texture, width, height, format }
    }

    pub fn bind(&self) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D, self.texture) }
    }
    pub fn unbind(&self) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D, 0) }
    }

    pub fn reformat(&mut self, width: u32, height: u32, format: TextureFormat) {
        if self.width != width || self.height != height || self.format != format {
            reformat(self.texture, width, height, &format);
            self.width = width;
            self.height = height;
            self.format = format;
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if self.width != width || self.height != height {
            reformat(self.texture, width, height, &self.format);
            self.width = width;
            self.height = height;
        }
    }

    pub fn attach_to_framebuffer(&self, attachment: TextureAttachment) {
        self.bind();
        unsafe { gl::FramebufferTexture2D(gl::FRAMEBUFFER, attachment.to_gl_internal(), gl::TEXTURE_2D, self.texture, 0); }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &self.texture) }
    }
}