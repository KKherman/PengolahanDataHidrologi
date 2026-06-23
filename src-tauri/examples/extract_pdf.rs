fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run --example extract_pdf -- <pdf_path>");
        return;
    }
    let path = &args[1];
    eprintln!("Extracting text from: {}", path);
    match pdf_extract::extract_text(path) {
        Ok(text) => {
            eprintln!("Extracted {} characters", text.len());
            println!("{}", text);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
