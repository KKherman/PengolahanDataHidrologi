use std::env;
use std::process::Command;
use std::path::PathBuf;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run --example ocr_debug -- <pdf_path> [dpi]");
        return;
    }
    let path = &args[1];
    let dpi: u32 = args.get(2).and_then(|d| d.parse().ok()).unwrap_or(600);
    
    println!("Testing OCR on: {} at {} DPI", path, dpi);
    
    // Use pdftoppm at specified DPI
    let pdftoppm = r"C:\Users\USER\AppData\Local\Microsoft\WinGet\Packages\oschwartz10612.Poppler_Microsoft.Winget.Source_8wekyb3d8bbwe\poppler-25.07.0\Library\bin\pdftoppm.exe";
    let tesseract = r"C:\Program Files\Tesseract-OCR\tesseract.exe";
    
    let temp_dir = std::env::temp_dir().join("hidrologi_ocr_debug");
    fs::create_dir_all(&temp_dir).unwrap();
    
    let img_base = temp_dir.join("page");
    println!("Running: {} -png -r {} \"{}\" \"{}\"", pdftoppm, dpi, path, img_base.display());
    let output = Command::new(pdftoppm)
        .arg("-png")
        .arg("-r").arg(dpi.to_string())
        .arg(path)
        .arg(img_base.to_str().unwrap())
        .output()
        .expect("Failed to run pdftoppm");
    
    println!("pdftoppm status: {}", output.status);
    if !output.status.success() {
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        return;
    }
    
    let mut page_num = 1;
    loop {
        let img_path = temp_dir.join(format!("page-{}.png", page_num));
        if !img_path.exists() { 
            // Try alternative naming
            let alt_path = temp_dir.join(format!("page-{}.png", page_num));
            if !alt_path.exists() { break; }
        }
        
        println!("\n=== PAGE {} ===", page_num);
        
        // Try PSM 4 (single column)
        for psm in &[3, 4, 6, 11] {
            let txt_path = temp_dir.join(format!("page_{}_psm{}", page_num, psm));
            let output = Command::new(tesseract)
                .arg(img_path.to_str().unwrap())
                .arg(txt_path.to_str().unwrap())
                .arg("-l").arg("ind+eng")
                .arg("--psm").arg(psm.to_string())
                .output()
                .expect("Failed to run tesseract");
            
            let txt_file = temp_dir.join(format!("page_{}_psm{}.txt", page_num, psm));
            if txt_file.exists() {
                let text = fs::read_to_string(&txt_file).unwrap_or_default();
                let lines: Vec<&str> = text.lines().filter(|l| !l.trim().is_empty()).collect();
                println!("\n  [PSM {}] {} lines:", psm, lines.len());
                for line in lines.iter().take(30) {
                    println!("  | {}", line);
                }
                fs::remove_file(&txt_file).ok();
            }
        }
        
        fs::remove_file(&img_path).ok();
        page_num += 1;
    }
    
    fs::remove_dir_all(&temp_dir).ok();
}
