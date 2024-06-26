use std::os::raw::c_void;
use std::{mem, ptr};

use ashen_engine::graphics::gl_wrapper::{BufferObject, Vao, VertexAttribute};
use ashen_engine::graphics::shaders::Shader;
use ashen_engine::graphics::textures::{Texture, TextureFiltering, TextureType, TextureWrapping};
use ashen_engine::logger;
use ashen_engine::window::Window;
use gl::types::{GLfloat, GLsizei};

fn main() {
    logger::init();

    let mut window: Window = Window::new(800, 600, "test");
    let vertices: [f32; 32] = [
        // positions       // colors        // texture coords
         0.5,  0.5, 0.0,   1.0, 0.0, 0.0,   2.0, 2.0, // top right
         0.5, -0.5, 0.0,   0.0, 1.0, 0.0,   2.0, 0.0, // bottom right
        -0.5, -0.5, 0.0,   0.0, 0.0, 1.0,   0.0, 0.0, // bottom left
        -0.5,  0.5, 0.0,   1.0, 1.0, 0.0,   0.0, 2.0  // top left
    ];
    let indices = [
        0, 1, 3,  // first Triangle
        1, 2, 3   // second Triangle
    ];

    window.init_gl();

    let mut shader = Shader::new();
    shader.load_fragment_shader("src/assets/shader.fs");
    shader.load_vertex_shader("src/assets/shader.vs");
    shader.link_program();

    let vao: Vao = Vao::new();
    vao.bind();

    let vbo: BufferObject = BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    vbo.bind();

    vbo.store_f32_data(&vertices);

    let ebo: BufferObject = BufferObject::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);
    ebo.bind();

    ebo.store_i32_data(&indices);

    let stride = 8 * mem::size_of::<GLfloat>() as GLsizei;

    let position_attrib: VertexAttribute = VertexAttribute::new(
        0,
        3,
        gl::FLOAT,
        gl::FALSE,
        stride,
        ptr::null(),
    );

    let color_attrib: VertexAttribute = VertexAttribute::new(
        1,
        3,
        gl::FLOAT,
        gl::FALSE,
        stride,
        (3 * mem::size_of::<GLfloat>()) as *const c_void,
    );

    let tex_cords_attrib: VertexAttribute = VertexAttribute::new(
        2,
        2,
        gl::FLOAT,
        gl::FALSE,
        stride,
        (6 * mem::size_of::<GLfloat>()) as *const c_void
    );

    position_attrib.enable();
    color_attrib.enable();
    tex_cords_attrib.enable();

    let texture: Texture = Texture::new(
        "src/assets/wall.jpg",
        TextureType::Texture2D,
        TextureWrapping::Mirror,
        TextureFiltering::Near,
        TextureFiltering::None,
    );

    let texture_2: Texture = Texture::new(
        "src/assets/awesomeface.png",
        TextureType::Texture2D,
        TextureWrapping::Mirror,
        TextureFiltering::Near,
        TextureFiltering::None,
    );

    let shader = unsafe {
        shader.use_program();

        texture.set_uniform("texture1", shader.id, 0);
        texture_2.set_uniform("texture2", shader.id, 1);

        shader
    };


    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture.id);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, texture_2.id);
            
            // render the triangle
            shader.use_program();
            gl::BindVertexArray(vao.id);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }

        window.update();
    }

    unsafe {
        gl::DeleteBuffers(1, &vao.id);
        gl::DeleteBuffers(1, &vbo.id);
        gl::DeleteBuffers(1, &ebo.id);
    }
}