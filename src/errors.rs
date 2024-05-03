use thiserror::Error;

#[derive(Error, Debug)]
pub enum Errors {
    #[error("This function was not implemented")]
    NotImplemented,
    #[error("")]
    ShadersError(ShaderErrors)
}

#[derive(Error, Debug)]
pub enum ShaderErrors {
    #[error("")]
    GlError(gl::types::GLenum)
}