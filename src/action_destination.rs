//! Defines the [PdfActionDestination] struct, exposing functionality related to the
//! destination of a single `PdfAction`.

pub enum PdfActionDestination {
    LocalDestination(PdfActionLocalDestination),
    RemoteDestination(PdfActionRemoteDestination),
    EmbeddedDestination(PdfActionEmbeddedDestination),
    Launch(PdfActionLaunchDestination),
    URI(PdfActionURIDestination),
}

pub struct PdfActionLocalDestination {}

pub struct PdfActionRemoteDestination {}

pub struct PdfActionEmbeddedDestination {}

pub struct PdfActionLaunchDestination {}

pub struct PdfActionURIDestination {}
