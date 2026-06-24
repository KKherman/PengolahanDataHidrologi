use crate::models::kualitas_air::KualitasAirRecord;
use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportSections {
    pub cover: String,
    pub kata_pengantar: String,
    pub bab1_1: String,
    pub bab1_2: String,
    pub bab2_1: String,
    pub bab2_2: String,
    pub bab2_3: String,
    pub bab3_1: String,
    pub bab3_2: String,
}

fn safe_val(v: &Option<f64>) -> String {
    match v {
        Some(val) => format!("{:.2}", val),
        None => "-".to_string(),
    }
}

fn safe_str(v: &Option<String>) -> String {
    match v {
        Some(s) if !s.is_empty() && s != "-" => s.clone(),
        _ => "-".to_string(),
    }
}

fn build_data_tables(records: &[KualitasAirRecord]) -> String {
    let mut tables = String::new();

    for (i, r) in records.iter().enumerate() {
        let pos_name = safe_str(&r.nama_pos);
        let tgl = safe_str(&r.tanggal_sampling);
        let ip = safe_val(&r.nilai_ip);
        let status = safe_str(&r.status_ip);

        tables.push_str(&format!(
            r#"<h3>Tabel {num}: Hasil Uji Kualitas Air - {nama}</h3>
<table class="data-table">
<tr><th>No</th><th>Parameter</th><th>Satuan</th><th>Hasil</th><th>Baku Mutu</th><th>Keterangan</th></tr>
<tr><td>1</td><td>Temperatur</td><td>°C</td><td>{temp}</td><td>deviasi 3</td><td></td></tr>
<tr><td>2</td><td>Konduktivitas</td><td>µS/cm</td><td>{konduk}</td><td>-</td><td></td></tr>
<tr><td>3</td><td>Kekeruhan</td><td>NTU</td><td>{kekeruhan}</td><td>-</td><td></td></tr>
<tr><td>4</td><td>Oksigen (DO)</td><td>mg/L</td><td>{oksigen}</td><td>≥ 4</td><td></td></tr>
<tr><td>5</td><td>pH</td><td>-</td><td>{ph}</td><td>6-9</td><td></td></tr>
<tr><td>6</td><td>TDS</td><td>mg/L</td><td>{tds}</td><td>1000</td><td></td></tr>
<tr><td>7</td><td>TSS</td><td>mg/L</td><td>{tss}</td><td>50</td><td></td></tr>
<tr><td>8</td><td>Warna</td><td>PtCo</td><td>{warna}</td><td>-</td><td></td></tr>
<tr><td>9</td><td>Klorida</td><td>mg/L</td><td>{klorida}</td><td>-</td><td></td></tr>
<tr><td>10</td><td>Amoniak</td><td>mg/L</td><td>{amoniak}</td><td>-</td><td></td></tr>
<tr><td>11</td><td>Nitrat</td><td>mg/L</td><td>{nitrat}</td><td>10</td><td></td></tr>
<tr><td>12</td><td>Nitrit</td><td>mg/L</td><td>{nitrit}</td><td>0.06</td><td></td></tr>
<tr><td>13</td><td>Fosfat</td><td>mg/L</td><td>{fosfat}</td><td>-</td><td></td></tr>
<tr><td>14</td><td>Deterjen</td><td>mg/L</td><td>{deterjen}</td><td>0.2</td><td></td></tr>
<tr><td>15</td><td>Arsen</td><td>mg/L</td><td>{arsen}</td><td>0.05</td><td></td></tr>
<tr><td>16</td><td>Besi</td><td>mg/L</td><td>{besi}</td><td>-</td><td></td></tr>
<tr><td>17</td><td>Mangan</td><td>mg/L</td><td>{mangan}</td><td>-</td><td></td></tr>
<tr><td>18</td><td>Tembaga</td><td>mg/L</td><td>{tembaga}</td><td>0.02</td><td></td></tr>
<tr><td>19</td><td>Merkuri</td><td>mg/L</td><td>{merkuri}</td><td>0.002</td><td></td></tr>
<tr><td>20</td><td>Sianida</td><td>mg/L</td><td>{sianida}</td><td>0.02</td><td></td></tr>
<tr><td>21</td><td>Fluorida</td><td>mg/L</td><td>{fluorida}</td><td>-</td><td></td></tr>
<tr><td>22</td><td>Belerang</td><td>mg/L</td><td>{belerang}</td><td>-</td><td></td></tr>
<tr><td>23</td><td>COD</td><td>mg/L</td><td>{cod}</td><td>25</td><td></td></tr>
<tr><td>24</td><td>BOD</td><td>mg/L</td><td>{bod}</td><td>3</td><td></td></tr>
<tr><td>25</td><td>Minyak & Lemak</td><td>mg/L</td><td>{minyak}</td><td>1</td><td></td></tr>
<tr><td>26</td><td>Fenol</td><td>mg/L</td><td>{fenol}</td><td>0.001</td><td></td></tr>
<tr><td>27</td><td>Total Coliform</td><td>MPN/100mL</td><td>{coliform}</td><td>5000</td><td></td></tr>
<tr><td>28</td><td>Debit</td><td>m³/s</td><td>{debit}</td><td>-</td><td></td></tr>
</table>
<p><strong>Nilai IP: {ip} | Status: {status}</strong></p>
<p><em>Tanggal Sampling: {tgl}</em></p>
"#,
            num = i + 1,
            nama = pos_name,
            temp = safe_val(&r.temperatur),
            konduk = safe_val(&r.konduktivitas),
            kekeruhan = safe_val(&r.kekeruhan),
            oksigen = safe_val(&r.oksigen),
            ph = safe_val(&r.ph),
            tds = safe_val(&r.tds),
            tss = safe_val(&r.tss),
            warna = safe_val(&r.warna),
            klorida = safe_val(&r.klorida),
            amoniak = safe_val(&r.amoniak),
            nitrat = safe_val(&r.nitrat),
            nitrit = safe_val(&r.nitrit),
            fosfat = safe_val(&r.fosfat),
            deterjen = safe_val(&r.deterjen),
            arsen = safe_val(&r.arsen),
            besi = safe_val(&r.besi),
            mangan = safe_val(&r.mangan),
            tembaga = safe_val(&r.tembaga),
            merkuri = safe_val(&r.merkuri),
            sianida = safe_val(&r.sianida),
            fluorida = safe_val(&r.fluorida),
            belerang = safe_val(&r.belerang),
            cod = safe_val(&r.cod),
            bod = safe_val(&r.bod),
            minyak = safe_val(&r.minyak_dan_lemak),
            fenol = safe_val(&r.fenol),
            coliform = safe_val(&r.total_coliform),
            debit = safe_val(&r.debit),
            ip = ip,
            status = status,
            tgl = tgl,
        ));
    }

    tables
}

fn md_to_html(s: &str) -> String {
    let mut html = String::new();
    for para in s.split("\n\n") {
        let p = para.trim();
        if p.is_empty() {
            continue;
        }
        html.push_str("<p>");
        html.push_str(&md_inline(p));
        html.push_str("</p>\n");
    }
    html
}

fn md_inline(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut out = String::new();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '*' {
            let mut n = 0;
            while i + n < chars.len() && chars[i + n] == '*' {
                n += 1;
            }
            if n > 3 {
                n = 3;
            }

            let mut j = i + n;
            let mut close = None;
            while j + n <= chars.len() {
                if (0..n).all(|k| chars[j + k] == '*') {
                    close = Some(j);
                    break;
                }
                j += 1;
            }

            if let Some(c) = close {
                let inner = md_inline(&s[i + n..c]);
                match n {
                    3 => out.push_str(&format!("<strong><em>{}</em></strong>", inner)),
                    2 => out.push_str(&format!("<strong>{}</strong>", inner)),
                    _ => out.push_str(&format!("<em>{}</em>", inner)),
                }
                i = c + n;
                continue;
            }
        }
        match chars[i] {
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '&' => out.push_str("&amp;"),
            '"' => out.push_str("&quot;"),
            c => out.push(c),
        }
        i += 1;
    }
    out
}

fn build_report_html(
    sections: &ReportSections,
    tahun: i32,
    bulan: &str,
    putaran: i32,
    records: &[KualitasAirRecord],
) -> String {
    let data_tables = build_data_tables(records);

    format!(
        r#"<!DOCTYPE html>
<html>
<head>
<meta charset="UTF-8">
<meta http-equiv="Content-Type" content="text/html; charset=UTF-8">
<style>
    @page {{ size: A4; margin: 2.5cm 3cm 2.5cm 3cm; }}
    body {{ font-family: 'Times New Roman', Times, serif; font-size: 12pt; line-height: 1.5; color: #000; }}
    .page {{ margin-bottom: 30px; }}
    .page-break {{ page-break-after: always; }}
    h1 {{ text-align: center; font-size: 18pt; font-weight: bold; margin-top: 60px; }}
    h2 {{ text-align: center; font-size: 14pt; font-weight: bold; margin-top: 30px; }}
    h3 {{ font-size: 12pt; font-weight: bold; margin-top: 20px; }}
    .cover-page {{ text-align: center; padding-top: 120px; }}
    .cover-page h1 {{ font-size: 20pt; margin-bottom: 10px; }}
    .cover-page h2 {{ font-size: 16pt; margin-bottom: 30px; }}
    .cover-page p {{ font-size: 12pt; margin: 5px 0; }}
    table.data-table {{ width: 100%; border-collapse: collapse; margin: 10px 0; font-size: 10pt; }}
    table.data-table th {{ background-color: #d9e1f2; border: 1px solid #000; padding: 4px 6px; text-align: center; font-weight: bold; }}
    table.data-table td {{ border: 1px solid #000; padding: 3px 6px; }}
    p {{ text-align: justify; text-indent: 1.5cm; margin: 6px 0; }}
    .no-indent {{ text-indent: 0; }}
    .kata-pengantar {{ margin: 0 40px; }}
</style>
</head>
<body>

<!-- COVER -->
<div class="page cover-page">
    <h1>LAPORAN UJI SAMPEL KUALITAS AIR</h1>
    <h2>TAHUN {tahun}</h2>
    <p>Periode: {bulan} - Putaran ke-{putaran}</p>
    <br><br>
    <p style="font-weight:bold;">BALAI WILAYAH SUNGAI SULAWESI IV</p>
    <p>Kendari</p>
    <br><br>
    <div style="text-align:left; margin-top:40px;">
        {cover}
    </div>
</div>

<div class="page-break"></div>

<!-- KATA PENGANTAR -->
<div class="page">
    <h2>KATA PENGANTAR</h2>
    <div class="kata-pengantar">
        {kata_pengantar}
    </div>
</div>

<div class="page-break"></div>

<!-- BAB I -->
<div class="page">
    <h2>BAB I<br>PENDAHULUAN</h2>
    <h3>1.1 Latar Belakang</h3>
    {bab1_1}
    <h3>1.2 Maksud dan Tujuan</h3>
    {bab1_2}
</div>

<div class="page-break"></div>

<!-- BAB II -->
<div class="page">
    <h2>BAB II<br>METODOLOGI</h2>
    <h3>2.1 Waktu dan Tempat</h3>
    {bab2_1}
    <h3>2.2 Parameter yang Diuji</h3>
    {bab2_2}
    <h3>2.3 Alat dan Bahan</h3>
    {bab2_3}
</div>

<div class="page-break"></div>

<!-- BAB III -->
<div class="page">
    <h2>BAB III<br>PENUTUP</h2>
    <h3>3.1 Kesimpulan</h3>
    {bab3_1}
    <h3>3.2 Saran</h3>
    {bab3_2}
</div>

<div class="page-break"></div>

<!-- LAMPIRAN -->
<div class="page">
    <h2>LAMPIRAN<br>DATA HASIL UJI KUALITAS AIR</h2>
    {data_tables}
</div>

</body>
</html>"#,
        tahun = tahun,
        bulan = bulan,
        putaran = putaran,
        cover = md_to_html(&sections.cover),
        kata_pengantar = md_to_html(&sections.kata_pengantar),
        bab1_1 = md_to_html(&sections.bab1_1),
        bab1_2 = md_to_html(&sections.bab1_2),
        bab2_1 = md_to_html(&sections.bab2_1),
        bab2_2 = md_to_html(&sections.bab2_2),
        bab2_3 = md_to_html(&sections.bab2_3),
        bab3_1 = md_to_html(&sections.bab3_1),
        bab3_2 = md_to_html(&sections.bab3_2),
        data_tables = data_tables,
    )
}

pub fn export_to_word(
    file_path: &str,
    sections: &ReportSections,
    tahun: i32,
    bulan: &str,
    putaran: i32,
    records: &[KualitasAirRecord],
) -> Result<String, String> {
    let html = build_report_html(sections, tahun, bulan, putaran, records);
    fs::write(file_path, html).map_err(|e| format!("Gagal menyimpan file: {}", e))?;
    Ok(format!("Laporan berhasil diexport ke: {}", file_path))
}
