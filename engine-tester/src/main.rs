use std::{mem, ptr};

use ashen_engine::graphics::gl_wrapper::{BufferObject, Vao, VertexAttribute};
use ashen_engine::logger;
use ashen_engine::window::Window;
use gl::types::{GLfloat, GLsizei};

fn main() {
    logger::init();

    let mut window: Window = Window::new(1080, 720, "test");
    let vertices: [f32; 18] = [
        // positions     // colors
         0.5, -0.5, 0.0,  1.0, 0.0, 0.0,  // bottom right
        -0.5, -0.5, 0.0,  0.0, 1.0, 0.0,  // bottom left
         0.0,  0.5, 0.0,  0.0, 0.0, 1.0   // top
    ];

    window.init_gl();

    let vao: Vao = Vao::new();
    vao.bind();

    let vbo: BufferObject = BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    vbo.bind();

    vbo.store_f32_data(&vertices);

    let position_attrib: VertexAttribute = VertexAttribute::new(
        0,
        3,
        gl::FLOAT,
        gl::FALSE,
        6 * mem::size_of::<GLfloat>() as GLsizei,
        ptr::null(),
    );

    position_attrib.enable();

    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            
            gl::BindVertexArray(vao.id);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        window.update();
    }
}