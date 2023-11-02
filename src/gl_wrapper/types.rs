pub enum Primitive {
    Points,
    LineStrip,
    LineLoop,
    Lines,
    LineStripAdjacency,
    LinesAdjacency,
    TriangleStrip,
    TriangleFan,
    Triangles,
    TriangleStripAdjacency,
    TrianglesAdjacency,
    Patches,
}

impl Primitive {
    pub fn to_gl_internal(&self) -> u32 {
        match self {
            Primitive::Points => gl::POINTS,
            Primitive::LineStrip => gl::LINE_STRIP,
            Primitive::LineLoop => gl::LINE_LOOP,
            Primitive::Lines => gl::LINES,
            Primitive::LineStripAdjacency => gl::LINE_STRIP_ADJACENCY,
            Primitive::LinesAdjacency => gl::LINES_ADJACENCY,
            Primitive::TriangleStrip => gl::TRIANGLE_STRIP,
            Primitive::TriangleFan => gl::TRIANGLE_FAN,
            Primitive::Triangles => gl::TRIANGLES,
            Primitive::TriangleStripAdjacency => gl::TRIANGLE_STRIP_ADJACENCY,
            Primitive::TrianglesAdjacency => gl::TRIANGLES_ADJACENCY,
            Primitive::Patches => gl::PATCHES,
        }
    }
}

pub enum AttributeType {
    Byte,
    UByte,
    Short,
    UShort,
    Int,
    UInt,
    Half,
    Float,
    Double,
}

impl AttributeType {
    pub fn to_gl_internal(&self) -> u32 {
        match self {
            AttributeType::Byte => gl::BYTE,
            AttributeType::UByte => gl::UNSIGNED_BYTE,
            AttributeType::Short => gl::SHORT,
            AttributeType::UShort => gl::UNSIGNED_SHORT,
            AttributeType::Int => gl::INT,
            AttributeType::UInt => gl::UNSIGNED_INT,
            AttributeType::Half => gl::HALF_FLOAT,
            AttributeType::Float => gl::FLOAT,
            AttributeType::Double => gl::DOUBLE,
        }
    }

    pub fn size(&self) -> i32 {
        match self {
            AttributeType::Byte | AttributeType::UByte => 1,
            AttributeType::Short | AttributeType::UShort => 2,
            AttributeType::Int | AttributeType::UInt => 4,
            AttributeType::Half => 2,
            AttributeType::Float => 4,
            AttributeType::Double => 8,
        }
    }
}

pub enum ShaderType {
    ComputeShader,
    VertexShader,
    TessCtrlShader,
    TessEvalShader,
    GeometryShader,
    FragmentShader,
}

impl ShaderType {
    pub fn to_gl_internal(&self) -> u32 {
        match self {
            ShaderType::ComputeShader => gl::COMPUTE_SHADER,
            ShaderType::VertexShader => gl::VERTEX_SHADER,
            ShaderType::TessCtrlShader => gl::TESS_CONTROL_SHADER,
            ShaderType::TessEvalShader => gl::TESS_EVALUATION_SHADER,
            ShaderType::GeometryShader => gl::GEOMETRY_SHADER,
            ShaderType::FragmentShader => gl::FRAGMENT_SHADER,
        }
    }
}

#[derive(PartialEq)]
pub enum TextureFilter {
    Nearest,
    Linear,
}

impl TextureFilter {
    pub fn to_gl_internal(&self) -> u32 {
        match self {
            TextureFilter::Nearest => gl::NEAREST,
            TextureFilter::Linear => gl::LINEAR,
        }
    }
}

#[derive(PartialEq)]
pub enum TextureFormat {
    Depth,
    DepthStencil,

    R8,
    R8I,
    R8UI,
    R16,
    R16I,
    R16UI,
    R16F,
    R32I,
    R32UI,
    R32F,

    RG8,
    RG8I,
    RG8UI,
    RG16,
    RG16I,
    RG16UI,
    RG16F,
    RG32I,
    RG32UI,
    RG32F,

    RGB8,
    RGB8I,
    RGB8UI,
    RGB16,
    RGB16I,
    RGB16UI,
    RGB16F,
    RGB32I,
    RGB32UI,
    RGB32F,

    RGBA8,
    RGBA8I,
    RGBA8UI,
    RGBA16,
    RGBA16I,
    RGBA16UI,
    RGBA16F,
    RGBA32I,
    RGBA32UI,
    RGBA32F,
}

impl TextureFormat {
    pub fn to_gl_internal(&self) -> u32 {
        match self {
            TextureFormat::Depth => gl::DEPTH_COMPONENT,
            TextureFormat::DepthStencil => gl::DEPTH_STENCIL,
            TextureFormat::R8 => gl::R8,
            TextureFormat::R8I => gl::R8I,
            TextureFormat::R8UI => gl::R8UI,
            TextureFormat::R16 => gl::R16,
            TextureFormat::R16I => gl::R16I,
            TextureFormat::R16UI => gl::R16UI,
            TextureFormat::R16F => gl::R16F,
            TextureFormat::R32I => gl::R32I,
            TextureFormat::R32UI => gl::R32UI,
            TextureFormat::R32F => gl::R32F,
            TextureFormat::RG8 => gl::RG8,
            TextureFormat::RG8I => gl::RG8I,
            TextureFormat::RG8UI => gl::RG8UI,
            TextureFormat::RG16 => gl::RG16,
            TextureFormat::RG16I => gl::RG16I,
            TextureFormat::RG16UI => gl::RG16UI,
            TextureFormat::RG16F => gl::RG16F,
            TextureFormat::RG32I => gl::RG32I,
            TextureFormat::RG32UI => gl::RG32UI,
            TextureFormat::RG32F => gl::RG32F,
            TextureFormat::RGB8 => gl::RGB8,
            TextureFormat::RGB8I => gl::RGB8I,
            TextureFormat::RGB8UI => gl::RGB8UI,
            TextureFormat::RGB16 => gl::RGB16,
            TextureFormat::RGB16I => gl::RGB16I,
            TextureFormat::RGB16UI => gl::RGB16UI,
            TextureFormat::RGB16F => gl::RGB16F,
            TextureFormat::RGB32I => gl::RGB32I,
            TextureFormat::RGB32UI => gl::RGB32UI,
            TextureFormat::RGB32F => gl::RGB32F,
            TextureFormat::RGBA8 => gl::RGBA8,
            TextureFormat::RGBA8I => gl::RGBA8I,
            TextureFormat::RGBA8UI => gl::RGBA8UI,
            TextureFormat::RGBA16 => gl::RGBA16,
            TextureFormat::RGBA16I => gl::RGBA16I,
            TextureFormat::RGBA16UI => gl::RGBA16UI,
            TextureFormat::RGBA16F => gl::RGBA16F,
            TextureFormat::RGBA32I => gl::RGBA32I,
            TextureFormat::RGBA32UI => gl::RGBA32UI,
            TextureFormat::RGBA32F => gl::RGBA32F,
        }
    }
}

pub enum TextureAttachment {
    Color(u32),
    Depth,
    Stencil,
    DepthStencil,
}

impl TextureAttachment {
    pub fn to_gl_internal(&self) -> u32 {
        match self {
            TextureAttachment::Color(idx) => gl::COLOR_ATTACHMENT0 + idx,
            TextureAttachment::Depth => gl::DEPTH_ATTACHMENT,
            TextureAttachment::Stencil => gl::STENCIL_ATTACHMENT,
            TextureAttachment::DepthStencil => gl::DEPTH_STENCIL_ATTACHMENT,
        }
    }
}
