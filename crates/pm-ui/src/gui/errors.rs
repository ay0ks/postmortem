use thiserror::Error;

#[derive(Error, Debug)]
pub enum X11Error {
    #[error("could not open X display {0}")]
    CouldNotOpen(String),
    #[error("could not create window")]
    CouldNotCreateWindow,
    #[error("could not create OpenGL context")]
    CouldNotCreateGLContext,
}
