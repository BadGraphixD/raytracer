use std::os::raw::c_void;

fn gen_buffer() -> u32 {
    let mut id: u32 = 0;
    unsafe { gl::GenBuffers(1, &mut id) }
    id
}

fn buffer_data<T>(id: u32, target: u32, data: &[T], usage: u32) {
    unsafe {
        gl::BindBuffer(target, id);
        gl::BufferData(
            target,
            (data.len() * std::mem::size_of::<T>()) as isize,
            &data[0] as *const T as *const c_void,
            usage
        )
    }
}

pub struct IndexBuffer {
    ibo: u32,
    size: i32,
}

impl IndexBuffer {
    pub fn new() -> Self {
        Self { ibo: gen_buffer(), size: 0 }
    }

    pub fn bind(&self) {
        unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ibo) }
    }
    pub fn unbind(&self) {
        unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0) }
    }

    pub fn buffer_data<T>(&mut self, data: &[T]) {
        buffer_data(self.ibo, gl::ELEMENT_ARRAY_BUFFER, data, gl::STATIC_DRAW);
        self.size = data.len() as i32;
    }

    pub fn size(&self) -> i32 { self.size }
}

impl Drop for IndexBuffer {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.ibo) }
    }
}

pub struct VertexBuffer {
    vbo: u32,
}

impl VertexBuffer {
    pub fn new() -> Self {
        Self { vbo: gen_buffer() }
    }

    pub fn bind(&self) {
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo) }
    }
    pub fn unbind(&self) {
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, 0) }
    }

    pub fn buffer_data<T>(&mut self, data: &[T]) {
        buffer_data(self.vbo, gl::ARRAY_BUFFER, data, gl::STATIC_DRAW);
    }
}

impl Drop for VertexBuffer {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.vbo) }
    }
}

