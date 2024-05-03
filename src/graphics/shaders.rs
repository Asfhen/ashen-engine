use std::{
    ffi::{CStr, CString},
    fs::File,
    io::Read,
    ptr,
    str
};

use gl::types::{GLchar, GLint};
use glm::{Matrix4, Vec3};

use crate::logger;

pub struct Shader {
    id: u32,

    fragment_shader: u32,
    geometry_shader: u32,
    vertex_shader: u32,
}

impl Shader {
    pub fn new() -> Shader {
        let mut shader: Shader = Shader {
            id: 0,
            
            fragment_shader: 0,
            geometry_shader: 0,
            vertex_shader: 0,
        };

        unsafe {
            shader.id = gl::CreateProgram();
        }

        shader
    }

    pub fn load_fragment_shader(&mut self, source: &str) {
        let mut shader_file = File::open(source)
            .unwrap_or_else(|e| {
                logger::error!("{}", e);
                panic!("Failed to open {}", source);
            });
        
        let mut shader_code = String::new();
        shader_file
            .read_to_string(&mut shader_code)
            .expect("Failed to read fragment shader code");

        let f_shader_code = CString::new(shader_code.as_bytes()).unwrap();

        unsafe {
            self.fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(
                self.fragment_shader,
                1,
                &f_shader_code.as_ptr(),
                ptr::null(),
            );
            gl::CompileShader(self.fragment_shader);
            self.check_compile_errors(self.fragment_shader, "FRAGMENT");

            gl::AttachShader(self.id, self.fragment_shader);
        }
    }

    pub fn load_vertex_shader(&mut self, source: &str) {
        let mut shader_file = File::open(source)
            .unwrap_or_else(|e| {
                logger::error!("{}", e);
                panic!("Failed to open {}", source);
            });
        
        let mut shader_code = String::new();
        shader_file
            .read_to_string(&mut shader_code)
            .expect("Failed to read fragment shader code");

        let v_shader_code = CString::new(shader_code.as_bytes()).unwrap();

        unsafe {
            self.vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(
                self.vertex_shader,
                1,
                &v_shader_code.as_ptr(),
                ptr::null(),
            );
            gl::CompileShader(self.vertex_shader);
            self.check_compile_errors(self.vertex_shader, "VERTEX");

            gl::AttachShader(self.id, self.vertex_shader);
        }
    }

    pub fn load_geometry_shader(&mut self, source: &str) {
        let mut shader_file = File::open(source)
            .unwrap_or_else(|e| {
                logger::error!("{}", e);
                panic!("Failed to open {}", source);
            });
        
        let mut shader_code = String::new();
        shader_file
            .read_to_string(&mut shader_code)
            .expect("Failed to read fragment shader code");

        let v_shader_code = CString::new(shader_code.as_bytes()).unwrap();

        unsafe {
            self.geometry_shader = gl::CreateShader(gl::GEOMETRY_SHADER);
            gl::ShaderSource(
                self.geometry_shader,
                1,
                &v_shader_code.as_ptr(),
                ptr::null(),
            );
            gl::CompileShader(self.geometry_shader);
            self.check_compile_errors(self.geometry_shader, "GEOMETRY");

            gl::AttachShader(self.id, self.geometry_shader);
        }
    }

    pub fn link_program(&self) {
        unsafe {
            gl::LinkProgram(self.id);
            self.check_compile_errors(self.id, "PROGRAM");

            if self.fragment_shader != 0 { gl::DeleteShader(self.fragment_shader); }
            if self.vertex_shader != 0 { gl::DeleteShader(self.vertex_shader); }
            if self.geometry_shader != 0 { gl::DeleteShader(self.geometry_shader); }
        }
    }

    pub unsafe fn use_program(&self) {
        gl::UseProgram(self.id);
    }

    pub unsafe fn set_bool(&self, name: &CStr, value: bool) {
        gl::Uniform1i(gl::GetUniformLocation(self.id, name.as_ptr()), value as i32);
    }

    pub unsafe fn set_int(&self, name: &CStr, value: i32) {
        gl::Uniform1i(gl::GetUniformLocation(self.id, name.as_ptr()), value);
    }

    pub unsafe fn set_float(&self, name: &CStr, value: f32) {
        gl::Uniform1f(gl::GetUniformLocation(self.id, name.as_ptr()), value);
    }

    pub unsafe fn set_vector_3(&self, name: &CStr, value: &Vec3) {
        gl::Uniform1fv(gl::GetUniformLocation(self.id, name.as_ptr()), 1, value.as_array().as_ptr());
    }

    pub unsafe fn set_vec_3(&self, name: &CStr, x: f32, y: f32, z: f32) {
        gl::Uniform3f(gl::GetUniformLocation(self.id, name.as_ptr()), x, y, z);
    }

    pub unsafe fn set_mat_4(&self, name: &CStr, value: &Matrix4<f32>) {
        gl::UniformMatrix4fv(
            gl::GetUniformLocation(self.id, name.as_ptr()),
            1,
            gl::FALSE,
            value.as_array().as_ptr() as *const f32
        );
    }

    unsafe fn check_compile_errors(&self, shader: u32, type_: &str) {
        let mut success = gl::FALSE as GLint;
        let mut info_log = Vec::with_capacity(1024);

        info_log.set_len(1024 - 1);

        if type_ != "PROGRAM" {
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(shader, 1024, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                println!("ERROR::SHADER_COMPILATION_ERROR of type: {}\n{}\n \
                          -- --------------------------------------------------- -- ",
                         type_,
                         str::from_utf8(&info_log).unwrap());
            }

        } else {
            gl::GetProgramiv(shader, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetProgramInfoLog(shader, 1024, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                println!("ERROR::PROGRAM_LINKING_ERROR of type: {}\n{}\n \
                          -- --------------------------------------------------- -- ",
                         type_,
                         str::from_utf8(&info_log).unwrap());
            }
        }
    }
}