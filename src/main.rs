use pdfium_render::prelude::{PdfPageRenderRotation, PdfRenderConfig, Pdfium};

fn main() {
    let pdfium = Pdfium::new(
        Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./lib"))
            .or_else(|_| Pdfium::bind_to_system_library())
            .unwrap(),
    );
    let render_config = PdfRenderConfig::new()
        .set_target_width(2000)
        .set_maximum_height(2000)
        .rotate_if_landscape(PdfPageRenderRotation::Degrees90, true);
    let document = pdfium
        .load_pdf_from_file("./ec06d154fb3ec32eaea65cd1d1fe5618.pdf", None)
        .unwrap();
    let document = document
        .pages()
        .get(0)
        .unwrap()
        .render_with_config(&render_config)
        .unwrap()
        .as_image()
        .into_rgb8();
    document
        .save_with_format(format!("{}.jpg", "test"), image::ImageFormat::Jpeg)
        .unwrap();
}
