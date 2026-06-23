use crate::models::kualitas_air::KualitasAirRecord;
use rust_xlsxwriter::*;

macro_rules! w {
    ($result:expr) => {
        $result.map_err(|e| format!("Excel error: {}", e))?;
    };
}

const SIHKA_PARAMS: &[(&str, &str, &str)] = &[
    ("1",  "Temperature (°C)",                         "°C"),
    ("2",  "Daya Hantar Listrik (DHL)",                "µS/cm"),
    ("3",  "Total Dissolved Solids (TDS)",             "mg/L"),
    ("4",  "Dissolved Oxygen (DO)",                    "mg/L"),
    ("5",  "Acidity (pH)",                             "-"),
    ("6",  "Salinity (ppt / %)",                       "ppt"),
    ("7",  "Turbidity (NTU)",                          "NTU"),
    ("8",  "Seawater Specific Gravity (ssg)",          ""),
    ("9",  "Ammonia (NH4) / Amoniak",                  "mg/L-N"),
    ("10", "Nitrate (NO3) / Nitrat",                   "mg/L-N"),
    ("11", "Oxidation Reduction Potential (ORP)",      "mV"),
    ("12", "Chemical Oxygen Demand (COD)",             "mg/L"),
    ("13", "Water Pollution Index (IP)",               ""),
    ("14", "Total Suspended Solids (TSS)",             "mg/L"),
    ("15", "Warna",                                     "Pt-Co Unit"),
    ("16", "Biochemical Oxygen Demand (BOD)",           "mg/L"),
    ("17", "Sulfat (SO42-)",                            "mg/L"),
    ("18", "Klorida (Cl-)",                             "mg/L"),
    ("19", "Nitrit (Sebagai N)",                        "mg/L-N"),
    ("20", "Total Nitrogen",                            "mg/L-N"),
    ("21", "Total Fosfat (sebagai P)",                  "mg/L-P"),
    ("22", "Flourida (F-)",                             "mg/L"),
    ("23", "Sianida (CN-)",                             "mg/L"),
    ("24", "Barium (Ba) terlarut",                      "mg/L"),
    ("25", "Boron (B) terlarut",                        "mg/L"),
    ("26", "Besi (Fe) terlarut",                        "mg/L"),
    ("27", "Kadmium (Cd) terlarut",                     "mg/L"),
    ("28", "Mangan (Mn) terlarut",                      "mg/L"),
    ("29", "Nikel (Ni) terlarut",                       "mg/L"),
    ("30", "Seng (Zn) terlarut",                        "mg/L"),
    ("31", "Tembaga (Cu) terlarut",                     "mg/L"),
    ("32", "Timbal (Pb) terlarut",                      "mg/L"),
    ("33", "Kromium heksavalen (Cr (VI))",              "mg/L"),
    ("34", "Minyak dan Lemak",                          "mg/L"),
    ("35", "Deterjen total",                            "mg/L"),
    ("36", "Fenol",                                     "mg/L"),
    ("37", "Aldrin/Dieldrin",                           "µg/L"),
    ("38", "BHC",                                       "µg/L"),
    ("39", "Chlordane",                                 "µg/L"),
    ("40", "Heptacolor",                                "µg/L"),
    ("41", "Lindane",                                   "µg/L"),
    ("42", "Methoxychlor",                              "µg/L"),
    ("43", "Toxapan",                                   "µg/L"),
    ("44", "Fecal Coliform",                            "MPN/100 mL"),
    ("45", "Total Coliform",                            "MPN/100 mL"),
    ("46", "Radioaktivitas (Gross A & Gross B)",        "Bq/L"),
    ("47", "Transparansi",                              "m"),
    ("48", "Klorofil-a",                                "mg/m3"),
];

fn record_to_sihka_rows(record: &KualitasAirRecord) -> Vec<(f64, Option<f64>)> {
    let mut rows = Vec::new();
    macro_rules! push {
        ($sihka_no:expr, $val:expr) => {
            if let Some(v) = $val {
                rows.push(($sihka_no as f64, Some(v)));
            }
        };
    }
    push!(1, record.temperatur);
    push!(2, record.konduktivitas);
    push!(3, record.tds);
    push!(4, record.oksigen);
    push!(5, record.ph);
    push!(7, record.kekeruhan);
    push!(9, record.amoniak);
    push!(10, record.nitrat);
    push!(12, record.cod);
    push!(13, record.nilai_ip);
    push!(14, record.tss);
    push!(15, record.warna);
    push!(16, record.bod);
    push!(18, record.klorida);
    push!(19, record.nitrit);
    push!(21, record.fosfat);
    push!(22, record.fluorida);
    push!(23, record.sianida);
    push!(26, record.besi);
    push!(28, record.mangan);
    push!(31, record.tembaga);
    push!(34, record.minyak_dan_lemak);
    push!(35, record.deterjen);
    push!(36, record.fenol);
    push!(45, record.total_coliform);
    rows
}

fn parse_date(date_str: &str) -> String {
    let parts: Vec<&str> = date_str.split(|c| c == '-' || c == '/').collect();
    if parts.len() == 3 {
        if parts[0].len() == 2 && parts[1].len() == 2 && parts[2].len() == 4 {
            return format!("{}-{}-{}", parts[2], parts[1], parts[0]);
        }
        if parts[0].len() == 4 {
            return date_str.to_string();
        }
    }
    date_str.to_string()
}

pub fn export_sihka_one(record: &KualitasAirRecord, file_path: &str) -> Result<(), String> {
    let nama_pos = record.nama_pos.as_deref().unwrap_or("-");
    let tanggal = record.tanggal_sampling.as_deref().unwrap_or("-");
    let param_count = record_to_sihka_rows(record).len();

    println!("📤 [SIHKA] Export 1 record: Pos '{}' ({})", nama_pos, tanggal);

    let mut workbook = Workbook::new();

    let header_fmt = Format::new()
        .set_bold()
        .set_background_color(Color::RGB(0x2C3E50))
        .set_font_color(Color::White)
        .set_border(FormatBorder::Thin)
        .set_font_size(11);

    let cell_fmt = Format::new()
        .set_border(FormatBorder::Thin)
        .set_font_size(10);

    let number_fmt = Format::new()
        .set_border(FormatBorder::Thin)
        .set_font_size(10)
        .set_num_format("0.######");

    // ── Sheet 1: Parameter Reference ──
    let sheet1 = workbook.add_worksheet();
    w!(sheet1.set_name("Parameter"));
    w!(sheet1.set_column_width(0, 6));
    w!(sheet1.set_column_width(1, 45));
    w!(sheet1.set_column_width(2, 16));

    w!(sheet1.write_string_with_format(0, 0, "NO", &header_fmt));
    w!(sheet1.write_string_with_format(0, 1, "PARAMETER", &header_fmt));
    w!(sheet1.write_string_with_format(0, 2, "UNIT", &header_fmt));

    for (i, (no, name, unit)) in SIHKA_PARAMS.iter().enumerate() {
        let row = (i + 1) as u32;
        w!(sheet1.write_string_with_format(row, 0, *no, &cell_fmt));
        w!(sheet1.write_string_with_format(row, 1, *name, &cell_fmt));
        w!(sheet1.write_string_with_format(row, 2, *unit, &cell_fmt));
    }

    // ── Sheet 2: Data Entry ──
    let sheet2 = workbook.add_worksheet();
    w!(sheet2.set_name("Data"));
    w!(sheet2.set_column_width(0, 14));
    w!(sheet2.set_column_width(1, 10));
    w!(sheet2.set_column_width(2, 12));
    w!(sheet2.set_column_width(3, 8));
    w!(sheet2.set_column_width(4, 16));

    w!(sheet2.write_string_with_format(0, 0, "date", &header_fmt));
    w!(sheet2.write_string_with_format(0, 1, "time", &header_fmt));
    w!(sheet2.write_string_with_format(0, 2, "parameter", &header_fmt));
    w!(sheet2.write_string_with_format(0, 3, "math", &header_fmt));
    w!(sheet2.write_string_with_format(0, 4, "value", &header_fmt));

    let date_str = record.tanggal_sampling.as_deref()
        .map(parse_date)
        .unwrap_or_default();
    let time_str = record.waktu_sampling.as_deref()
        .unwrap_or("00:00:00");

    let sihka_rows = record_to_sihka_rows(record);
    let mut row_idx: u32 = 1;
    for (sihka_no, value) in &sihka_rows {
        w!(sheet2.write_string_with_format(row_idx, 0, &date_str, &cell_fmt));
        w!(sheet2.write_string_with_format(row_idx, 1, time_str, &cell_fmt));
        w!(sheet2.write_number_with_format(row_idx, 2, *sihka_no, &cell_fmt));
        w!(sheet2.write_string_with_format(row_idx, 3, "", &cell_fmt));
        if let Some(v) = value {
            w!(sheet2.write_number_with_format(row_idx, 4, *v, &number_fmt));
        }
        row_idx += 1;
    }

    workbook.save(file_path)
        .map_err(|e| format!("Gagal menyimpan Excel: {}", e))?;

    println!("✅ [SIHKA] Excel berhasil disimpan: '{}' ({}) — {} parameter",
        nama_pos, tanggal, param_count);
    println!("   File: {}", file_path);

    Ok(())
}
