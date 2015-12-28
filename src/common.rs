//! Contains common types for reader and writer.

/// Format of FBX data
#[derive(Debug, Clone, Copy)]
pub enum FbxFormatType {
    /// Binary FBX, with version (for example, `7400` for FBX 7.4).
    Binary(u32),
    /// ASCII FBX.
    Ascii,
}
