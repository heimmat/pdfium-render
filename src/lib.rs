pub mod bindgen {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    include!("bindgen.rs");
}

pub mod bindings;
pub mod bitmap;
pub mod bitmap_config;
pub mod document;
pub mod page;
pub mod pdfium;

use crate::bindgen::{
    FPDF_ERR_FILE, FPDF_ERR_FORMAT, FPDF_ERR_PAGE, FPDF_ERR_PASSWORD, FPDF_ERR_SECURITY,
    FPDF_ERR_UNKNOWN,
};

// Conditional compilation is used to compile different implementations of
// the PdfiumLibraryBindings trait depending on whether we are compiling to a
// WASM module or to a native shared library.

#[cfg(not(target_arch = "wasm32"))]
mod native;

#[cfg(target_arch = "wasm32")]
mod wasm;

pub type PdfPageIndex = u16;
pub type PdfPoints = f32;

#[derive(Debug)]
pub enum PdfiumInternalError {
    Unknown = FPDF_ERR_UNKNOWN as isize,
    FileError = FPDF_ERR_FILE as isize,
    FormatError = FPDF_ERR_FORMAT as isize,
    PasswordError = FPDF_ERR_PASSWORD as isize,
    SecurityError = FPDF_ERR_SECURITY as isize,
    PageError = FPDF_ERR_PAGE as isize,
}

#[derive(Debug)]
pub enum PdfiumError {
    DynamicLibraryLoadingNotSupportedOnWASM,
    #[cfg(not(target_arch = "wasm32"))]
    LoadLibraryError(libloading::Error),
    PageIndexOutOfBounds,
    UnknownBitmapFormat,
    PdfiumLibraryInternalError(PdfiumInternalError),
}

#[cfg(test)]
pub mod tests {
    use crate::bitmap::PdfBitmapRotation;
    use crate::bitmap_config::PdfBitmapConfig;
    use crate::pdfium::Pdfium;
    use image::ImageFormat;

    #[test]
    fn test() {
        let bitmap_settings = PdfBitmapConfig::new()
            .set_target_width(2000)
            .set_maximum_height(2000)
            .rotate_if_landscape(PdfBitmapRotation::Degrees90, true);

        Pdfium::new(Pdfium::bind_to_library("./libpdfium.so").unwrap())
            .load_pdf_from_file("./test/test.pdf", None)
            .unwrap()
            .pages()
            .for_each(|page| {
                let result = page
                    .get_bitmap_with_config(&bitmap_settings)
                    .unwrap()
                    .as_image()
                    .as_bgra8()
                    .unwrap()
                    .save_with_format(format!("test-page-{}.jpg", page.index()), ImageFormat::Jpeg);

                assert!(result.is_ok());
            });
    }
}
