use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run --example test_ocr -- <pdf_or_image_path>");
        return;
    }
    let path = &args[1];
    eprintln!("Testing OCR on: {}", path);
    match data_hidrologi_lib::services::ocr_engine::ocr_pdf(path) {
        Ok(record) => {
            println!("✅ OCR succeeded!");
            println!("  nama_pos: {:?}", record.nama_pos);
            println!("  tanggal_sampling: {:?}", record.tanggal_sampling);
            println!("  temperatur: {:?}", record.temperatur);
            println!("  tds: {:?}", record.tds);
            println!("  tss: {:?}", record.tss);
            println!("  amoniak: {:?}", record.amoniak);
            println!("  nitrat: {:?}", record.nitrat);
            println!("  nitrit: {:?}", record.nitrit);
            println!("  cod: {:?}", record.cod);
            println!("  bod: {:?}", record.bod);
            println!("  warna: {:?}", record.warna);
            println!("  deterjen: {:?}", record.deterjen);
            println!("  minyak_dan_lemak: {:?}", record.minyak_dan_lemak);
            println!("  fenol: {:?}", record.fenol);
            println!("  sianida: {:?}", record.sianida);
            println!("  fluorida: {:?}", record.fluorida);
            println!("  klorida: {:?}", record.klorida);
            println!("  besi: {:?}", record.besi);
            println!("  fosfat: {:?}", record.fosfat);
            println!("  tembaga: {:?}", record.tembaga);
            println!("  mangan: {:?}", record.mangan);
            println!("  arsen: {:?}", record.arsen);
            println!("  merkuri: {:?}", record.merkuri);
            println!("  belerang: {:?}", record.belerang);
            println!("  total_coliform: {:?}", record.total_coliform);
            println!("  oksigen: {:?}", record.oksigen);
            println!("  konduktivitas: {:?}", record.konduktivitas);
            println!("  kekeruhan: {:?}", record.kekeruhan);
            println!("  ph: {:?}", record.ph);
            println!("  debit: {:?}", record.debit);
        }
        Err(e) => eprintln!("❌ OCR failed: {}", e),
    }
}
