use std::os::raw::c_void;
use image::{EncodableLayout, RgbImage};
use crate::gl_wrapper::types::{TextureAttachment, TextureFilter, TextureFormat};

fn gen_texture() -> u32 {
    let mut id: u32 = 0;
    unsafe { gl::GenTextures(1, &mut id) }
    id
}

fn reformat(texture: u32, width: u32, height: u32, format: &TextureFormat, data: *const c_void) {
    unsafe {
        gl::BindTexture(gl::TEXTURE_2D, texture);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            format.to_gl_internal() as i32,
            width as i32,
            height as i32,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            data,
        );
    }
}

fn change_filter(texture: u32, filter: &TextureFilter) {
    unsafe {
        gl::BindTexture(gl::TEXTURE_2D, texture);
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            filter.to_gl_internal() as i32,
        );
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MAG_FILTER,
            filter.to_gl_internal() as i32,
        );
    }
}

pub struct Texture {
    texture: u32,
    width: u32,
    height: u32,
    format: TextureFormat,
    filter: TextureFilter,
}

impl Texture {
    pub fn new(width: u32, height: u32, format: TextureFormat, filter: TextureFilter) -> Self {
        let texture = gen_texture();
        reformat(texture, width, height, &format, 0 as *const _);
        change_filter(texture, &filter);
        Self {
            texture,
            width,
            height,
            format,
            filter,
        }
    }
    pub fn from_data(format: TextureFormat, filter: TextureFilter, data: &RgbImage) -> Self {
        let texture = gen_texture();
        reformat(
            texture, data.width(), data.height(), &format,
            data.as_bytes().as_ptr() as *const _
        );
        change_filter(texture, &filter);
        Self {
            texture,
            width: data.width(),
            height: data.height(),
            format,
            filter,
        }
    }

    pub fn bind(&self) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D, self.texture) }
    }
    pub fn unbind(&self) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D, 0) }
    }

    pub fn bind_to_slot(&self, slot: u32) -> u32 {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + slot);
        }
        self.bind();
        slot
    }

    pub fn reformat(&mut self, width: u32, height: u32, format: TextureFormat) {
        if self.width != width || self.height != height || self.format != format {
            reformat(self.texture, width, height, &format, 0 as *const _);
            self.width = width;
            self.height = height;
            self.format = format;
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if self.width != width || self.height != height {
            reformat(self.texture, width, height, &self.format, 0 as *const _);
            self.width = width;
            self.height = height;
        }
    }

    pub fn change_filter(&mut self, filter: TextureFilter) {
        if self.filter != filter {
            change_filter(self.texture, &filter);
            self.filter = filter;
        }
    }

    pub fn attach_to_framebuffer(&self, attachment: TextureAttachment) {
        self.bind();
        unsafe {
            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                attachment.to_gl_internal(),
                gl::TEXTURE_2D,
                self.texture,
                0,
            );
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &self.texture) }
    }
}
