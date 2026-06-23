// Import modul
pub mod commands;
pub mod models;
pub mod services;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let handle = app.handle().clone();
            
            tauri::async_runtime::block_on(async move {
                match services::db_service::init_db(&handle).await {
                    Ok(pool) => {
                        handle.manage(pool);
                        println!("✅ Database berhasil diinisialisasi dan disimpan ke State.");
                    }
                    Err(e) => {
                        panic!("❌ Gagal inisialisasi database: {}", e);
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Daftarkan semua command di sini
            commands::kualitas_air::submit_kualitas_air,
            commands::kualitas_air::calculate_ip_preview,
            commands::kualitas_air::import_pdf,
            commands::kualitas_air::get_all_kualitas_air,
            commands::kualitas_air::delete_kualitas_air,
            commands::kualitas_air::export_kualitas_air_csv,
            commands::kualitas_air::export_excel_sihka
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}