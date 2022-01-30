use std::io;

#[derive(Debug, Clone, thiserror::Error)]
pub enum ParrotError {
    #[error("Suitable graphics adapter was not found")]
    NoAdaptersFound,
    #[error("Device creation error")]
    RequestDeviceError(#[from] wgpu::RequestDeviceError),
}

impl From<Error> for io::Error {
    fn from(err: Error) -> io::Error {
        io::Error::new(io::ErrorKind::Other, err)
    }
}
