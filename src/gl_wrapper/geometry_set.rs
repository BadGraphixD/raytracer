use crate::gl_wrapper::buffer::{IndexBuffer, VertexBuffer};
use crate::gl_wrapper::types::{AttributeType, Primitive};
use crate::rendering::model::Model;

fn gen_vertex_array() -> u32 {
    let mut id: u32 = 0;
    unsafe { gl::GenVertexArrays(1, &mut id) }
    id
}

pub struct GeometrySet {
    vao: u32,
    draw_count: i32,
    primitives: u32,
}

impl GeometrySet {
    fn new(draw_count: i32, primitives: u32) -> Self {
        Self {
            vao: gen_vertex_array(),
            draw_count,
            primitives,
        }
    }

    pub fn bind(&self) {
        unsafe { gl::BindVertexArray(self.vao) }
    }
    pub fn unbind(&self) {
        unsafe { gl::BindVertexArray(0) }
    }

    pub fn draw(&self) {
        self.bind();
        unsafe {
            gl::DrawElements(
                self.primitives,
                self.draw_count,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            )
        }
    }
}

impl Drop for GeometrySet {
    fn drop(&mut self) {
        unsafe { gl::DeleteVertexArrays(1, &self.vao) }
    }
}

struct Attribute {
    dimension: i32,
    r#type: AttributeType,
}

impl Attribute {
    fn size(&self) -> i32 {
        self.dimension * self.r#type.size()
    }
}

struct Buffer<'a> {
    buffer: &'a VertexBuffer,
    attributes: Vec<Attribute>,
}

pub struct GeometrySetBuilder<'a> {
    buffers: Vec<Buffer<'a>>,
}

impl<'a> GeometrySetBuilder<'a> {
    pub fn new() -> Self {
        Self { buffers: vec![] }
    }

    pub fn add_buffer(mut self, buffer: &'a VertexBuffer) -> Self {
        self.buffers.push(Buffer {
            buffer,
            attributes: vec![],
        });
        self
    }

    pub fn add_attribute(mut self, dimension: i32, r#type: AttributeType) -> Self {
        if let Some(buffer) = self.buffers.last_mut() {
            buffer.attributes.push(Attribute { dimension, r#type })
        }
        self
    }

    pub fn build(self, indices: &IndexBuffer, primitives: Primitive) -> GeometrySet {
        let mesh = GeometrySet::new(indices.size(), primitives.to_gl_internal());
        mesh.bind();
        indices.bind();

        let mut idx = 0;
        self.buffers.iter().for_each(|buffer| {
            if let Some(stride) = buffer
                .attributes
                .iter()
                .map(|attrib| attrib.size())
                .reduce(|acc, s| acc + s)
            {
                buffer.buffer.bind();
                let mut offset = 0;
                buffer.attributes.iter().for_each(|attrib| {
                    unsafe {
                        gl::VertexAttribPointer(
                            idx,
                            attrib.dimension,
                            attrib.r#type.to_gl_internal(),
                            gl::FALSE,
                            stride,
                            offset as *const _,
                        );
                        gl::EnableVertexAttribArray(idx); // fuck this line
                    }
                    offset += attrib.size();
                    idx += 1;
                });
            }
        });

        mesh
    }

    pub fn create_square_geometry() -> (GeometrySet, IndexBuffer, VertexBuffer) {
        const VERTICES: [f32; 8] = [-1.0, -1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0];
        const INDICES: [i32; 6] = [0, 2, 1, 0, 2, 3];

        let mut vbo = VertexBuffer::new();
        let mut ibo = IndexBuffer::new();
        vbo.buffer_data(&VERTICES);
        ibo.buffer_data(&INDICES);

        let gs = GeometrySetBuilder::new()
            .add_buffer(&vbo)
            .add_attribute(2, AttributeType::Float)
            .build(&ibo, Primitive::Triangles);

        (gs, ibo, vbo)
    }

    pub fn from_model(model: &Model) -> (GeometrySet, IndexBuffer, Vec<VertexBuffer>) {
        let mut ibo = IndexBuffer::new();
        let mut pos_vbo = VertexBuffer::new();
        let mut tex_vbo = None;
        let mut nor_vbo = None;

        ibo.buffer_data(model.triangles());
        pos_vbo.buffer_data(model.positions());

        let mut gsb = GeometrySetBuilder::new()
            .add_buffer(&pos_vbo)
            .add_attribute(3, AttributeType::Float);

        if let Some(tex_coords) = model.tex_coords() {
            tex_vbo = Some(VertexBuffer::new());
            tex_vbo.as_mut().unwrap().buffer_data(tex_coords);
            gsb = gsb.add_buffer(tex_vbo.as_ref().unwrap()).add_attribute(2, AttributeType::Float);
        }
        if let Some(normals) = model.normals() {
            nor_vbo = Some(VertexBuffer::new());
            nor_vbo.as_mut().unwrap().buffer_data(normals);
            gsb = gsb.add_buffer(nor_vbo.as_ref().unwrap()).add_attribute(3, AttributeType::Float);
        }

        let gs = gsb.build(&ibo, Primitive::Triangles);
        let mut vbos = vec![pos_vbo];
        if let Some(tex_vbo) = tex_vbo { vbos.push(tex_vbo) }
        if let Some(nor_vbo) = nor_vbo { vbos.push(nor_vbo) }

        (gs, ibo, vbos)
    }
}
