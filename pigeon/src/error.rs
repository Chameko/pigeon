#[derive(Debug, Clone, thiserror::Error)]
pub enum EguiBackendError {
    #[error("Unknown texture ID: {0}")]
    UnknownTextureId(String),
}