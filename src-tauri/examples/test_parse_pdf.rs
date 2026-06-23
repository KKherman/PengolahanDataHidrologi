fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run --example test_parse_pdf -- <pdf_path>");
        return;
    }
    let path = args[1].clone();
    match data_hidrologi_lib::services::pdf_engine::parse_pdf(path) {
        Ok(record) => {
            println!("nama_pos: {:?}", record.nama_pos);
            println!("tanggal: {:?}", record.tanggal_sampling);
            println!("amoniak: {:?}", record.amoniak);
            println!("nitrat: {:?}", record.nitrat);
            println!("nitrit: {:?}", record.nitrit);
            println!("tds: {:?}", record.tds);
            println!("tss: {:?}", record.tss);
            println!("cod: {:?}", record.cod);
            println!("bod: {:?}", record.bod);
            println!("warna: {:?}", record.warna);
            println!("deterjen: {:?}", record.deterjen);
            println!("fenol: {:?}", record.fenol);
            println!("sianida: {:?}", record.sianida);
            println!("fluorida: {:?}", record.fluorida);
            println!("klorida: {:?}", record.klorida);
            println!("besi: {:?}", record.besi);
            println!("fosfat: {:?}", record.fosfat);
            println!("tembaga: {:?}", record.tembaga);
            println!("mangan: {:?}", record.mangan);
            println!("arsen: {:?}", record.arsen);
            println!("merkuri: {:?}", record.merkuri);
            println!("belerang: {:?}", record.belerang);
            println!("total_coliform: {:?}", record.total_coliform);
            println!("oksigen: {:?}", record.oksigen);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
