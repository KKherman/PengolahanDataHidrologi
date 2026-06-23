use crate::models::kualitas_air::KualitasAirRecord;
use regex::Regex;
use std::path::PathBuf;
use std::process::Command;
use std::fs;

fn find_tool(name: &str) -> Option<PathBuf> {
    if let Ok(output) = Command::new("where.exe").arg(name).output() {
        if output.status.success() {
            let path_str = String::from_utf8_lossy(&output.stdout)
                .lines().next().map(|s| s.trim().to_string());
            if let Some(p) = path_str {
                let pb = PathBuf::from(&p);
                if pb.exists() { return Some(pb); }
            }
        }
    }

    let program_files = r"C:\Program Files";
    if let Ok(entries) = fs::read_dir(program_files) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() && path.to_string_lossy().to_lowercase().contains("tesseract") {
                let exe = path.join(format!("{}.exe", name));
                if exe.exists() { return Some(exe); }
            }
        }
    }

    let fallbacks = vec![
        format!("C:\\Program Files\\Tesseract-OCR\\{}.exe", name),
        format!("C:\\Users\\USER\\AppData\\Local\\Microsoft\\WinGet\\Packages\\oschwartz10612.Poppler_Microsoft.Winget.Source_8wekyb3d8bbwe\\poppler-25.07.0\\Library\\bin\\{}.exe", name),
    ];
    for p in &fallbacks {
        let pb = PathBuf::from(p);
        if pb.exists() { return Some(pb); }
    }

    None
}

pub fn ocr_pdf(file_path: &str) -> Result<KualitasAirRecord, String> {
    println!("🔍 [OCR] Memproses PDF: {}", file_path);

    let pdftoppm = find_tool("pdftoppm")
        .ok_or_else(|| "pdftoppm tidak ditemukan. Install poppler-utils.".to_string())?;
    let tesseract = find_tool("tesseract")
        .ok_or_else(|| "tesseract tidak ditemukan. Install Tesseract OCR.".to_string())?;

    let temp_dir = std::env::temp_dir().join("hidrologi_ocr");
    fs::create_dir_all(&temp_dir)
        .map_err(|e| format!("Gagal membuat direktori temp: {}", e))?;

    let img_base = temp_dir.join("page");
    let output = Command::new(&pdftoppm)
        .arg("-png")
        .arg("-r").arg("600")
        .arg(file_path)
        .arg(img_base.to_str().unwrap())
        .output()
        .map_err(|e| format!("Gagal menjalankan pdftoppm: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("pdftoppm error: {}", stderr));
    }

    let mut all_text = String::new();
    let mut page_num = 1;
    loop {
        let img_path = temp_dir.join(format!("page-{}.png", page_num));
        if !img_path.exists() { break; }

        let txt_path = temp_dir.join(format!("page_{}", page_num));
        let _ocr_output = Command::new(&tesseract)
            .arg(img_path.to_str().unwrap())
            .arg(txt_path.to_str().unwrap())
            .arg("-l").arg("ind+eng")
            .arg("--psm").arg("4")
            .output()
            .map_err(|e| format!("Gagal menjalankan tesseract: {}", e))?;

        let txt_file = temp_dir.join(format!("page_{}.txt", page_num));
        if txt_file.exists() {
            let page_text = fs::read_to_string(&txt_file).unwrap_or_default();
            all_text.push_str(&page_text);
            all_text.push('\n');
            fs::remove_file(&txt_file).ok();
        }

        fs::remove_file(&img_path).ok();
        page_num += 1;
    }

    fs::remove_dir_all(&temp_dir).ok();

    if all_text.trim().is_empty() {
        return Err("OCR tidak menghasilkan teks.".to_string());
    }

    let preview_len = all_text.len().min(2000);
    println!("  [OCR] Total teks: {} karakter", all_text.len());
    println!("{}", &all_text[..preview_len]);
    println!("  === PREVIEW END ===");

    extract_parameters(&all_text)
}

pub fn ocr_image(file_path: &str) -> Result<KualitasAirRecord, String> {
    println!("🔍 [OCR] Memproses gambar: {}", file_path);

    let tesseract = find_tool("tesseract")
        .ok_or_else(|| "tesseract tidak ditemukan".to_string())?;

    let temp_dir = std::env::temp_dir().join("hidrologi_ocr");
    fs::create_dir_all(&temp_dir)
        .map_err(|e| format!("Gagal membuat direktori temp: {}", e))?;

    let txt_base = temp_dir.join("img_output");
    let output = Command::new(&tesseract)
        .arg(file_path)
        .arg(txt_base.to_str().unwrap())
        .arg("-l").arg("ind+eng")
        .arg("--psm").arg("4")
        .output()
        .map_err(|e| format!("Gagal menjalankan tesseract: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Tesseract error: {}", stderr));
    }

    let txt_file = temp_dir.join("img_output.txt");
    let content = fs::read_to_string(&txt_file)
        .map_err(|e| format!("Gagal membaca hasil OCR: {}", e))?;

    fs::remove_dir_all(&temp_dir).ok();

    if content.trim().is_empty() {
        return Err("OCR tidak menghasilkan teks.".to_string());
    }

    extract_parameters(&content)
}

fn extract_parameters(text: &str) -> Result<KualitasAirRecord, String> {
    println!("=== EKSTRAKSI PARAMETER DARI OCR ===");

    let mut data = KualitasAirRecord {
        id: None,
        nama_pos: None, das: None, wilayah_sungai: None, provinsi: None, kabupaten: None,
        tahun: None, elevasi_pos: None, pelaksana: None, kecamatan: None, laboratorium: None,
        sungai: None, desa: None, koordinat_geografis: None,
        tanggal_sampling: None, waktu_sampling: None,
        temperatur: None, konduktivitas: None, kekeruhan: None, oksigen: None, ph: None,
        tds: None, tss: None, warna: None, klorida: None, amoniak: None, nitrat: None,
        nitrit: None, fosfat: None, deterjen: None, arsen: None, besi: None, mangan: None,
        tembaga: None, merkuri: None, sianida: None, fluorida: None, belerang: None,
        cod: None, bod: None, minyak_dan_lemak: None, fenol: None, total_coliform: None,
        debit: None,
        nilai_ip: None, status_ip: None, nilai_storet: None, status_storet: None,
        created_at: None,
    };

    let lines: Vec<&str> = text.lines().collect();

    // --- PASS 1: METADATA ---
    for line in &lines {
        let t = line.trim();
        if t.is_empty() { continue; }

        if data.nama_pos.is_none() {
            if let Some(val) = extract_metadata_field(t, &["Lokasi sampel", "Lokasi"], ':') {
                println!("[META] Nama Pos: {}", val);
                data.nama_pos = Some(val);
            }
        }

        if data.tanggal_sampling.is_none() {
            if let Some(date) = extract_date(t, "sampling") {
                data.tanggal_sampling = Some(date);
            }
        }
    }

    // --- PASS 2: PARAMETERS (structured table) ---
    // Format: <no> <param> <unit> <value> [<baku_mutu>] SNI
    let re_num = Regex::new(r"\b(\d+)[,:.](\d+)\b|\b(\d+)\b").unwrap();

    for line in &lines {
        let t = line.trim();
        if t.is_empty() { continue; }

        let sni_pos = t.to_uppercase().find("SNI").unwrap_or(t.len());
        let before_sni = &t[..sni_pos];

        // Helper: find all numbers before SNI, skipping leading line numbers
        let extract_first_value = |after_kw: &str| -> Option<f64> {
            re_num.captures_iter(after_kw)
                .filter_map(|cap| {
                    let raw = cap.get(0)?.as_str();
                    // Handle "2:20" -> "2.20", "048" -> "48" (but keep as-is)
                    let num_str = raw.replace(',', ".").replace(':', ".");
                    // Skip if number is embedded in text like "H2S" or "SNI6989"
                    let start = cap.get(0)?.start();
                    let end = cap.get(0)?.end();
                    let before = after_kw[..start].chars().last();
                    let after = after_kw[end..].chars().next();
                    // Skip numbers preceded/followed by letters (chemical formulas, SNI refs)
                    if before.map(|c| c.is_alphabetic()).unwrap_or(false)
                        || after.map(|c| c.is_alphabetic()).unwrap_or(false)
                    {
                        return None;
                    }
                    // Skip numbers that are part of a known unit prefix (e.g. "100 mL", "1,000 mg/L")
                    let rest = &after_kw[end..].trim_start();
                    let unit_word = rest.split_whitespace().next().unwrap_or("");
                    if ["mL", "mg", "L", "mg/L", "mL", "MPN", "Pt", "Co", "Unit",
                        "NTU", "mg/L", "mg/L)", "(Cl", "(H2S", "(NH3", "(Fe", "(Mn",
                        "(Cu", "(As", "(Hg", "(P", "(Cl"]
                        .contains(&unit_word)
                    {
                        return None;
                    }
                    num_str.parse::<f64>().ok()
                })
                .filter(|&v| v > 0.0 && v < 1_000_000.0)
                .next()
        };

        macro_rules! try_extract {
            ($field:expr, $keywords:expr, $($alt:expr),+ $(,)?) => {
                if $field.is_none() {
                    let matched = [$($alt),+].iter().find(|kw| {
                        Regex::new(&format!("(?i){}", kw)).unwrap().is_match(t)
                    });
                    if let Some(kw) = matched {
                        let re = Regex::new(&format!("(?i){}", kw)).unwrap();
                        let kw_end = re.find(t)
                            .map(|m| m.end())
                            .unwrap_or(0);
                        let search_start = kw_end.min(t.len());
                        let after_kw = &before_sni[search_start.min(before_sni.len())..];

                        if let Some(val) = extract_first_value(after_kw) {
                            $field = Some(val);
                            println!("[OK] {}: {} (dari: '{}')", $keywords, val, t);
                        }
                    }
                }
            };
        }

        try_extract!(data.temperatur, "Temperatur", "Temperatur", "Suhu");
        try_extract!(data.tds, "TDS", "TDS");
        try_extract!(data.tss, "TSS", "TSS");
        try_extract!(data.warna, "Warna", "Warna");
        try_extract!(data.amoniak, "Amoniak", "Amoniak", r"NH3\b");
        try_extract!(data.nitrat, "Nitrat", "Nitrat");
        try_extract!(data.nitrit, "Nitrit", "Nitrit");
        try_extract!(data.cod, "COD", r"\bCOD\b");
        try_extract!(data.bod, "BOD", r"\bBOD\b");
        try_extract!(data.deterjen, "Deterjen", "Detergen", "Deterjen");
        try_extract!(data.minyak_dan_lemak, "Minyak & Lemak", r"Minyak\s*[&/]\s*lemak");
        try_extract!(data.fenol, "Fenol", "Fenol");
        try_extract!(data.sianida, "Sianida", "Sianida");
        try_extract!(data.fluorida, "Fluorida", "Fluorida");
        try_extract!(data.klorida, "Klorida", "Klorida");
        try_extract!(data.besi, "Besi", r"\bBesi\b");
        try_extract!(data.fosfat, "Fosfat", "Fosfat");
        try_extract!(data.tembaga, "Tembaga", "Tembaga");
        try_extract!(data.mangan, "Mangan", "Mangan");
        try_extract!(data.arsen, "Arsen", "Arsen");
        try_extract!(data.merkuri, "Merkuri", "Merkuri");
        try_extract!(data.belerang, "Belerang", r"\bBelerang\b");
        try_extract!(data.total_coliform, "Total Coliform", "Total Coliform", r"Coliform\b", "Koliform");
        try_extract!(data.oksigen, "Oksigen", "Oksigen", r"\bDO\b");
        try_extract!(data.konduktivitas, "Konduktivitas", "Konduktivitas");
        try_extract!(data.kekeruhan, "Kekeruhan", "Kekeruhan");
        try_extract!(data.ph, "pH", r"\bpH\b");
        try_extract!(data.debit, "Debit", "Debit");
    }

    println!("=== SELESAI EKSTRAKSI OCR ===\n");
    Ok(data)
}

fn extract_metadata_field<'a>(line: &'a str, keywords: &[&str], separator: char) -> Option<String> {
    let lower = line.to_lowercase();
    let kw_match = keywords.iter().any(|kw| lower.contains(&kw.to_lowercase()));
    if !kw_match { return None; }

    if let Some(idx) = line.find(separator) {
        let val = line[idx+1..].trim();
        if !val.is_empty() && val != "-" {
            return Some(val.to_string());
        }
    }
    None
}

fn extract_date(line: &str, keyword: &str) -> Option<String> {
    if !line.to_lowercase().contains(keyword) { return None; }

    let re = Regex::new(r"(\d{1,2})\s+([A-Za-z]{3,}[a-z]*)\s+(\d{4})").ok()?;
    if let Some(caps) = re.captures(line) {
        let day = caps.get(1)?.as_str();
        let month_str = caps.get(2)?.as_str().to_lowercase();
        let year = caps.get(3)?.as_str();

        let month_num = match &month_str[..3] {
            "jan" => "01", "feb" => "02", "mar" => "03", "apr" => "04",
            "mei" | "may" => "05", "jun" => "06", "jul" => "07",
            "agu" | "aug" => "08", "sep" => "09", "okt" | "oct" => "10",
            "nov" => "11", "des" | "dec" => "12",
            _ => return None,
        };

        let formatted = format!("{}-{}-{}", day, month_num, year);
        println!("[META] Tanggal {}: {}", keyword, formatted);
        return Some(formatted);
    }
    None
}
