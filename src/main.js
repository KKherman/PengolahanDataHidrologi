// Import fungsi inisialisasi dari modul halaman
import { initKualitasAir } from './pages/kualitas-air.js';
import { initDatabaseKualitasAir } from './pages/database-kualitas-air.js';
import { initLaporanUjiSampel } from './pages/laporan-uji-sampel.js';

// Mapping: ID Halaman -> Fungsi Init JS-nya
const pageScripts = {
    'kualitas-air': initKualitasAir,
    'database-kualitas-air': initDatabaseKualitasAir,
    'laporan-uji-sampel': initLaporanUjiSampel,
    // 'dashboard': initDashboard, // Nanti jika sudah ada
};

/**
 * Fungsi Utama Router
 * @param {string} pageName - Nama file di folder pages/ (tanpa .html)
 */
async function loadPage(pageName) {
    const contentDiv = document.getElementById('app-content');
    
    // 1. Tampilkan Loading (opsional)
    contentDiv.innerHTML = `<div class="loading"><i class="fas fa-spinner fa-spin"></i> Memuat ${pageName}...</div>`;

    try {
        // 2. Fetch File HTML
        const response = await fetch(`./pages/${pageName}.html`);
        
        if (!response.ok) {
            throw new Error(`Halaman ${pageName} tidak ditemukan (${response.status})`);
        }

        // 3. Masukkan HTML ke DOM
        const html = await response.text();
        contentDiv.innerHTML = html;

        // 4. Jalankan Script Logika (jika ada di mapping)
        if (pageScripts[pageName]) {
            pageScripts[pageName]();
        }

        // 5. Update Active State di Sidebar
        document.querySelectorAll('.nav-link').forEach(btn => btn.classList.remove('active'));
        const activeBtn = document.getElementById(`nav-${pageName}`);
        if (activeBtn) activeBtn.classList.add('active');

    } catch (error) {
        console.error("Router Error:", error);
        contentDiv.innerHTML = `<div class="error-msg"><h3>Gagal memuat halaman</h3><p>${error.message}</p></div>`;
    }
}

// Export loadPage untuk digunakan dari modul lain
window.loadPage = loadPage;

// Event Listener saat aplikasi siap
window.addEventListener("DOMContentLoaded", () => {
    // Listener Menu Kualitas Air
    const btnKualitasAir = document.getElementById("nav-kualitas-air");
    if (btnKualitasAir) {
        btnKualitasAir.addEventListener("click", () => loadPage("kualitas-air"));
    }

    // Listener Menu Dashboard (Placeholder)
    document.getElementById("nav-dashboard")?.addEventListener("click", () => {
        document.getElementById('app-content').innerHTML = `
            <div style="text-align:center; margin-top:50px;">
                <h2>Dashboard</h2>
                <p>Fitur ini belum tersedia di MVP Fase 1.</p>
            </div>`;
        
        // Update active class
        document.querySelectorAll('.nav-link').forEach(btn => btn.classList.remove('active'));
        document.getElementById('nav-dashboard').classList.add('active');
    });

    // Listener Menu Database Kualitas Air
    document.getElementById("nav-ruang-data")?.addEventListener("click", () => {
        console.log("📊 Navigasi ke Database Kualitas Air");
        loadPage("database-kualitas-air");
    });

    // Default Load: Buka Dashboard atau Kualitas Air saat pertama buka
    // Kita arahkan langsung ke Kualitas Air untuk testing
    loadPage("kualitas-air");
});