mod pdf;
mod ui;
mod utils;
mod pdf;

fn main() {
    let path = "sample.pdf"; // Cambia el nombre si tu archivo PDF es diferente

    match pdf::loader::render_pdf_first_page(path) {
        Ok(_) => println!("✅ PDF renderizado correctamente como 'render.png'"),
        Err(e) => eprintln!("❌ Error al renderizar el PDF: {:?}", e),
    }
}
