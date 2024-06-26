use std::{os::raw::c_void, path::Path};

extern crate alloc;
use alloc::ffi::CString;


pub struct Texture {
    pub id: u32,
    target: TextureType,
}

impl Texture {
    pub fn new(source: &str, target: TextureType, wrapping: TextureWrapping, filtering: TextureFiltering, mipmap_filtering: TextureFiltering) -> Texture {
        let image = image::open(source).unwrap_or_else(|e| {
            panic!("{}", e)
        });
        let image = image.flipv();
        let (width, height) = (image.width() as i32, image.height() as i32);
        let data = image.into_bytes();
        let wrap: i32;
        let filter: i32;
        let mipmap_filter: i32;
        let mut texture = Texture { id: 0, target: target.clone() };

        let file_path = Path::new(source);
        let format = match file_path.extension() {
            Some(os_str) => match os_str.to_str() {
                Some("png") => gl::RGBA,
                Some("jpg") => gl::RGB,
                Some(_) => gl::RGB,
                None => panic!("File not recognized as an image"),
            },
            None => panic!("File not recognized as an image"),
        };

        unsafe {
            match wrapping {
                TextureWrapping::Repeat => wrap = gl::REPEAT as i32,
                TextureWrapping::Mirror => wrap = gl::MIRRORED_REPEAT as i32,
                TextureWrapping::ClampToEdge => wrap = gl::CLAMP_TO_EDGE as i32,
                TextureWrapping::ClampToBorder => wrap = gl::CLAMP_TO_BORDER as i32,
            }

            match filtering {
                TextureFiltering::Near => {
                    filter = gl::NEAREST as i32;

                    match mipmap_filtering {
                        TextureFiltering::Near => mipmap_filter = gl::NEAREST_MIPMAP_NEAREST as i32,
                        TextureFiltering::Linear => mipmap_filter = gl::NEAREST_MIPMAP_LINEAR as i32,
                        TextureFiltering::None => mipmap_filter = gl::NEAREST as i32,
                    }
                },
                TextureFiltering::Linear => {
                    filter = gl::LINEAR as i32;

                    match mipmap_filtering {
                        TextureFiltering::Near => mipmap_filter = gl::LINEAR_MIPMAP_NEAREST as i32,
                        TextureFiltering::Linear => mipmap_filter = gl::LINEAR_MIPMAP_LINEAR as i32,
                        TextureFiltering::None => mipmap_filter = gl::LINEAR as i32,
                    }
                },
                TextureFiltering::None => panic!("It's obligatory to define a filter for images"),
            }

            gl::GenTextures(1, &mut texture.id);
            match target {
                TextureType::Texture2D => {
                    gl::BindTexture(gl::TEXTURE_2D, texture.id);

                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, wrap);
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, wrap);

                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, filter);
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, filter);

                    gl::TexImage2D(
                        gl::TEXTURE_2D,
                        0,
                        gl::RGB as i32,
                        width,
                        height,
                        0,
                        format,
                        gl::UNSIGNED_BYTE,
                        &data[0] as *const u8 as *const c_void,
                    );
                    gl::GenerateMipmap(gl::TEXTURE_2D);
                },
                TextureType::Texture3D => {
                    gl::BindTexture(gl::TEXTURE_3D, texture.id);

                    gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_WRAP_S, wrap);
                    gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_WRAP_T, wrap);
                    gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_WRAP_R, wrap);

                    gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_MIN_FILTER, mipmap_filter);
                    gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_MAG_FILTER, filter);

                    // TODO: add generation of 3D textures
                },
            }
        }

        texture
    }

    pub unsafe fn bind(&self) {
        match self.target {
            TextureType::Texture2D => gl::BindTexture(gl::TEXTURE_2D, self.id),
            TextureType::Texture3D => gl::BindTexture(gl::TEXTURE_3D, self.id),
        }
    }

    pub fn set_uniform(&self, target: &str, shader: u32, value: i32) {
        let c_str = CString::new(target).expect("Failed to create c_string");
        unsafe {
            gl::Uniform1i(gl::GetUniformLocation(shader, c_str.as_ptr()), value);
        }
    }
}

#[derive(Clone, Copy)]
pub enum TextureType {
    Texture2D,
    Texture3D,
}

pub enum TextureWrapping {
    Repeat,
    Mirror,
    ClampToEdge,
    ClampToBorder,
}

pub enum TextureFiltering {
    Near,
    Linear,
    None,
}