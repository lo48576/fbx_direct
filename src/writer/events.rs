//! Contains `FbxEvent` for writer.

use std::borrow::Cow;

use crate::common::{FbxFormatType, Property};

pub enum FbxEvent<'a> {
    /// Denotes start of FBX data.
    ///
    /// For Binary FBX, this item corresponds to magic binary.
    StartFbx(FbxFormatType),
    /// Denotes end of FBX data.
    ///
    /// This event should always be written before any other event. If it is not written at all, a
    /// default format and version (which may be different according to this crate's version) is
    /// used.
    ///
    /// NOTE: Current implementation of Binary FBX parser does not read to the last byte of the FBX stream.
    EndFbx,
    /// Denotes beginning of a node.
    StartNode {
        /// Node name.
        name: &'a str,
        /// Node properties.
        properties: Cow<'a, [Property<'a>]>,
    },
    /// Denotes end of a node.
    EndNode,
    /// Comment.
    ///
    /// Comment only appears in ASCII FBX.
    Comment(&'a str),
}
