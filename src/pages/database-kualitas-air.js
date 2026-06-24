// Database Kualitas Air Module
// Fase 3: Real data handling
// Refactored: Reporting logic moved to cetak-kualitas-air.js

const { invoke } = window.__TAURI__.core;

// Import modul cetak yang baru
import { initPrintSystem, populatePrintTemplate, executePrint } from './cetak-kualitas-air.js';

// State untuk record yang sedang dibuka di modal
let currentPrintRecordId = null;

// ============================================
// MAIN INITIALIZATION
// ============================================

export function initDatabaseKualitasAir() {
    console.log("🚀 Database Kualitas Air module loaded.");
    
    setupBasicEventListeners();
    
    // Inisialisasi sistem cetak (load template HTML ke dalam modal)
    initPrintSystem('modalBodyContent'); 
    
    // Load data tabel utama
    loadData();
}

function setupBasicEventListeners() {
    // 1. Refresh Button
    const btnRefresh = document.getElementById('btnRefresh');
    if (btnRefresh) btnRefresh.addEventListener('click', loadData);
    
    // 2. Export All (Bulk CSV)
    const btnExportAll = document.getElementById('btnExportAll');
    if (btnExportAll) {
        btnExportAll.addEventListener('click', async (e) => {
            e.stopPropagation();
            await handleBulkExport();
        });
    }
    
    
    // 3. Navigation Buttons
    const navToInput = () => {
        const navKualitasAir = document.getElementById('nav-kualitas-air');
        if (navKualitasAir) navKualitasAir.click();
    };

    // Navigate to Laporan page
    const btnBuatLaporan = document.getElementById('btnBuatLaporan');
    if (btnBuatLaporan) {
        btnBuatLaporan.addEventListener('click', () => {
            if (typeof window.loadPage === 'function') {
                window.loadPage('laporan-uji-sampel');
            }
        });
    }
    
    const btnGoToInput = document.getElementById('btnGoToInput');
    if (btnGoToInput) btnGoToInput.addEventListener('click', navToInput);
    
    const btnGoToInputFromError = document.getElementById('btnGoToInputFromError');
    if (btnGoToInputFromError) btnGoToInputFromError.addEventListener('click', navToInput);
    
    const btnRetry = document.getElementById('btnRetry');
    if (btnRetry) btnRetry.addEventListener('click', loadData);
    
    // 4. Modal Controls (Close)
    const btnCloseModal = document.getElementById('btnCloseModal');
    const btnCloseModal2 = document.getElementById('btnCloseModal2');
    
    if (btnCloseModal) btnCloseModal.addEventListener('click', hideModal);
    if (btnCloseModal2) btnCloseModal2.addEventListener('click', hideModal);
    
    // 5. PRINT PDF BUTTON (Action di dalam Modal)
    const btnPrintPDF = document.getElementById('btnPrintPDF');
    if (btnPrintPDF) {
        btnPrintPDF.addEventListener('click', () => {
            console.log("🖨️ User requested print...");
            executePrint();
        });
    }
    
    // 6. EXPORT EXCEL SIHKA (Action di dalam Modal, per record)
    const btnExportExcelSIHKA = document.getElementById('btnExportExcelSIHKA');
    if (btnExportExcelSIHKA) {
        btnExportExcelSIHKA.addEventListener('click', async (e) => {
            e.stopPropagation();
            await handleExportExcelSIHKA();
        });
    }
}

// ============================================
// DATA LOADING & RENDERING
// ============================================

async function loadData() {
    try {
        showLoading();
        const rawData = await invoke('get_all_kualitas_air');
        const transformedData = transformData(rawData);
        renderCards(transformedData);
    } catch (error) {
        console.error("❌ Error loading data:", error);
        showErrorState(error);
    }
}

function renderCards(data) {
    hideLoading();
    
    const recordCount = document.getElementById('recordCount');
    if (recordCount) recordCount.textContent = data.length;
    
    if (!data || data.length === 0) {
        showEmptyState();
        return;
    }
    
    const cardsGrid = document.getElementById('cardsGrid');
    if (cardsGrid) {
        cardsGrid.innerHTML = '';
        cardsGrid.style.display = 'grid';
        
        document.getElementById('emptyState').style.display = 'none';
        document.getElementById('errorState').style.display = 'none';
        
        data.forEach((record, index) => {
            cardsGrid.appendChild(createCardElement(record, index));
        });
    }
    
    updateStats(data);
}

function createCardElement(record, index) {
    const card = document.createElement('div');
    card.className = 'db-card';
    card.dataset.id = record.id;
    
    // PERBAIKAN DI SINI:
    // Gunakan field yang sudah diproses di transformData
    // Jangan menghitung ulang manual di sini karena rawan salah nama key
    
    const info = record.status_info; // Gunakan info status yang sudah dihitung
    
    card.innerHTML = `
        <div class="db-card-header">
            <div class="db-card-title">
                <span>${record.display_nama_pos}</span>
                <span class="db-card-badge ${info.colorClass}">${info.text}</span>
            </div>
            <div class="db-card-subtitle">
                <small><i class="fas fa-calendar"></i> ${record.display_tanggal}</small>
            </div>
        </div>
        <div class="db-card-body">
            <div class="db-card-row">
                <span class="db-card-label">Lokasi:</span>
                <span class="db-card-value">${record.display_lokasi}</span>
            </div>
            <div class="db-card-row">
                <span class="db-card-label">IP Score:</span>
                <span class="db-card-value" style="font-weight: 700; color: ${info.color}">${record.display_nilai_ip}</span>
            </div>
            <div class="db-card-row">
                <span class="db-card-label">Pelaksana:</span>
                <span class="db-card-value">${record.display_pelaksana}</span>
            </div>
        </div>
        <div class="db-card-actions">
            <button class="db-card-btn db-card-btn-export" style="background-color: #2c3e50; border-color: #2c3e50;">
                <i class="fas fa-file-alt"></i> Laporan
            </button>
            <button class="db-card-btn db-card-btn-delete">
                <i class="fas fa-trash"></i> Hapus
            </button>
        </div>
    `;
    
    // Listener Tombol Laporan (Preview)
    const btnReport = card.querySelector('.db-card-btn-export');
    if (btnReport) {
        btnReport.addEventListener('click', (e) => {
            e.stopPropagation();
            openPrintPreview(record);
        });
    }

    // Listener Tombol Hapus
    const btnDelete = card.querySelector('.db-card-btn-delete');
    if (btnDelete) {
        btnDelete.addEventListener('click', async (e) => {
            e.stopPropagation();
            if (await window.confirm("Yakin ingin menghapus data ini?")) {
                await deleteRecord(record.id, card);
            }
        });
    }
    
    return card;
}

// Logic untuk membuka Modal Preview
function openPrintPreview(record) {
    const modal = document.getElementById('detailModal');
    
    // Simpan ID record yang sedang dilihat untuk export
    currentPrintRecordId = record.id;
    
    // Panggil fungsi dari modul cetak untuk mengisi data
    populatePrintTemplate(record);

    // Tampilkan Modal
    if (modal) modal.style.display = 'flex';
}

// ============================================
// UTILS & HELPERS (Local)
// ============================================

function transformData(data) {
    if (!Array.isArray(data)) return [];
    
    return data.map(record => {
        // Copy semua field asli
        const transformed = { ...record };
        
        // Handle field naming inconsistency (Rust snake_case vs JS camelCase)
        // Ini kuncinya: kita cek dua-duanya
        const statusIp = record.status_ip || record.statusIp;
        const nilaiIp = record.nilai_ip || record.nilaiIp;
        
        transformed.display_nama_pos = getSafeValue(record.nama_pos, 'Data Tanpa Nama');
        transformed.display_lokasi = getLokasiDisplay(record);
        transformed.display_pelaksana = getPelaksanaDisplay(record);
        transformed.display_tanggal = formatDate(getSafeValue(record.tanggal_sampling, ''));
        
        // Format nilai IP di sini sekali saja
        transformed.display_nilai_ip = formatNumber(nilaiIp, 2);
        
        // Hitung status info di sini sekali saja
        const info = getStatusInfo(statusIp, nilaiIp);
        transformed.status_info = info; 
        
        return transformed;
    });
}

function getLokasiDisplay(record) {
    const locations = [
        record.das, 
        record.sungai, 
        record.provinsi, 
        record.kabupaten
    ];
    
    for (const loc of locations) {
        if (loc && loc !== '-' && loc !== '') return loc;
    }
    return '-';
}

function getPelaksanaDisplay(record) {
    return record.pelaksana || record.laboratorium || '-';
}

function getStatusInfo(statusIp, nilaiIp) {
    let status = (statusIp || '').toLowerCase();
    
    if (status.includes('memenuhi')) return { text: 'Memenuhi', color: '#27ae60', colorClass: 'status-good' };
    if (status.includes('ringan')) return { text: 'Cemar Ringan', color: '#f39c12', colorClass: 'status-light' };
    if (status.includes('sedang')) return { text: 'Cemar Sedang', color: '#e67e22', colorClass: 'status-medium' };
    if (status.includes('berat')) return { text: 'Cemar Berat', color: '#e74c3c', colorClass: 'status-heavy' };
    
    if (nilaiIp !== null && !isNaN(nilaiIp)) {
        const ip = Number(nilaiIp);
        if (ip <= 1.0) return { text: 'Memenuhi', color: '#27ae60', colorClass: 'status-good' };
        if (ip <= 5.0) return { text: 'Cemar Ringan', color: '#f39c12', colorClass: 'status-light' };
        if (ip <= 10.0) return { text: 'Cemar Sedang', color: '#e67e22', colorClass: 'status-medium' };
        return { text: 'Cemar Berat', color: '#e74c3c', colorClass: 'status-heavy' };
    }
    return { text: 'Unknown', color: '#95a5a6', colorClass: 'status-unknown' };
}

function getSafeValue(value, fallback = '-') {
    if (value === null || value === undefined || value === '') {
        return fallback;
    }
    return value;
}

function formatNumber(num, decimals = 2) {
    if (num === null || num === undefined || isNaN(num)) return '-';
    return Number(num).toFixed(decimals);
}

function formatDate(dateString) {
    if (!dateString) return '-';
    try {
        const parts = dateString.split(' ')[0].split('-');
        if (parts.length === 3 && parts[0].length === 4) {
            return `${parts[2]}-${parts[1]}-${parts[0]}`;
        }
    } catch (e) { return dateString; }
    return dateString;
}

function hideModal() {
    const modal = document.getElementById('detailModal');
    if (modal) modal.style.display = 'none';
}

async function handleBulkExport() {
    try {
        document.body.style.cursor = 'wait';
        const result = await invoke('export_kualitas_air_csv');
        alert("✅ Export Berhasil:\n" + result);
    } catch (error) {
        if (!String(error).includes("dibatalkan")) alert("❌ Gagal Export: " + error);
    } finally {
        document.body.style.cursor = 'default';
    }
}

async function handleExportExcelSIHKA() {
    if (!currentPrintRecordId) {
        alert("❌ Tidak ada record yang dipilih untuk diexport.");
        return;
    }
    try {
        document.body.style.cursor = 'wait';
        const result = await invoke('export_excel_sihka', { recordId: currentPrintRecordId });
        alert("✅ " + result);
    } catch (error) {
        if (!String(error).includes("dibatalkan")) alert("❌ Gagal Export Excel SIHKA: " + error);
    } finally {
        document.body.style.cursor = 'default';
    }
}

async function deleteRecord(id, cardElement) {
    try {
        await invoke('delete_kualitas_air', { id });
        cardElement.style.opacity = '0';
        setTimeout(() => {
            cardElement.remove();
            const grid = document.getElementById('cardsGrid');
            if (grid && grid.children.length === 0) showEmptyState();
            const counter = document.getElementById('recordCount');
            if (counter) counter.textContent = grid ? grid.children.length : 0;
        }, 300);
        alert("✅ Data berhasil dihapus.");
    } catch (error) {
        alert("❌ Gagal menghapus: " + error);
    }
}

function showLoading() {
    const lc = document.querySelector('.loading-container');
    if (lc) lc.style.display = 'block';
    const grid = document.getElementById('cardsGrid');
    if (grid) grid.style.display = 'none';
}

function hideLoading() {
    const lc = document.querySelector('.loading-container');
    if (lc) lc.style.display = 'none';
}

function showEmptyState() {
    hideLoading();
    document.getElementById('emptyState').style.display = 'block';
}

function showErrorState(msg) {
    hideLoading();
    const err = document.getElementById('errorState');
    if (err) {
        err.style.display = 'block';
        document.getElementById('errorMessage').textContent = String(msg);
    }
}

function updateStats(data) {
    if (!data) return;
    const counts = { total: data.length, good: 0, light: 0, medium: 0, heavy: 0 };
    data.forEach(r => {
        const s = (r.status_ip || r.statusIp || '').toLowerCase(); // Cek kedua key juga di sini
        if (s.includes('memenuhi')) counts.good++;
        else if (s.includes('ringan')) counts.light++;
        else if (s.includes('sedang')) counts.medium++;
        else if (s.includes('berat')) counts.heavy++;
    });
    const stats = document.querySelectorAll('.stat-value');
    if (stats.length >= 5) {
        stats[0].textContent = counts.total;
        stats[1].textContent = counts.good;
        stats[2].textContent = counts.light;
        stats[3].textContent = counts.medium;
        stats[4].textContent = counts.heavy;
    }
}