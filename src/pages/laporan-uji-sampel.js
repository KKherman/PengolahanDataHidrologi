const { invoke } = window.__TAURI__.core;

let laporanState = {
    tahun: null,
    bulan: null,
    putaran: 1,
    records: [],
    generatedSections: new Set(),
};

export function initLaporanUjiSampel() {
    setupEventListeners();
    checkApiKeyAndProceed();
}

async function checkApiKeyAndProceed() {
    const banner = document.getElementById('apiKeyBanner');
    const content = document.getElementById('laporanContent');
    try {
        const hasKey = await invoke('check_api_key');
        if (hasKey) {
            banner.style.display = 'none';
            if (content) content.style.display = 'block';
            loadTahunOptions();
        } else {
            banner.style.display = 'flex';
            if (content) content.style.display = 'none';
        }
    } catch (err) {
        banner.style.display = 'flex';
        if (content) content.style.display = 'none';
        console.error('Gagal cek API key:', err);
    }
}

function setupEventListeners() {
    document.getElementById('btnLoadLaporanData')?.addEventListener('click', handleLoadData);

    document.getElementById('btnSaveApiKey')?.addEventListener('click', handleSaveApiKey);
    document.getElementById('apiKeyInput')?.addEventListener('keydown', (e) => {
        if (e.key === 'Enter') handleSaveApiKey();
    });

    document.querySelectorAll('.btn-ai-generate').forEach(btn => {
        btn.addEventListener('click', (e) => {
            const section = e.currentTarget.dataset.section;
            handleGenerate(section);
        });
    });

    document.getElementById('btnExportWord')?.addEventListener('click', handleExportWord);
}

async function loadTahunOptions() {
    const select = document.getElementById('laporanTahun');
    try {
        const years = await invoke('get_tahun_options');
        if (Array.isArray(years) && years.length > 0) {
            select.innerHTML = '<option value="">-- Pilih Tahun --</option>';
            years.forEach(y => {
                const opt = document.createElement('option');
                opt.value = y;
                opt.textContent = y;
                select.appendChild(opt);
            });
        }
    } catch (err) {
        console.error('Gagal load tahun:', err);
    }
}

async function handleSaveApiKey() {
    const input = document.getElementById('apiKeyInput');
    const statusEl = document.getElementById('apiKeyStatus');
    const key = input?.value?.trim();

    if (!key) {
        if (statusEl) {
            statusEl.textContent = '❌ Masukkan API Key terlebih dahulu';
            statusEl.className = 'api-key-status error';
        }
        return;
    }

    try {
        await invoke('save_api_key', { key });
        if (statusEl) {
            statusEl.textContent = '✅ API Key berhasil disimpan!';
            statusEl.className = 'api-key-status success';
        }
        setTimeout(() => {
            const banner = document.getElementById('apiKeyBanner');
            const content = document.getElementById('laporanContent');
            if (banner) banner.style.display = 'none';
            if (content) content.style.display = 'block';
            loadTahunOptions();
        }, 1000);
    } catch (err) {
        if (statusEl) {
            statusEl.textContent = '❌ Gagal menyimpan: ' + err;
            statusEl.className = 'api-key-status error';
        }
    }
}

async function handleLoadData() {
    const tahun = document.getElementById('laporanTahun').value;
    const bulan = document.getElementById('laporanBulan').value;
    const putaran = parseInt(document.getElementById('laporanPutaran').value) || 1;

    if (!tahun) return showToast('Pilih tahun terlebih dahulu', 'error');
    if (!bulan) return showToast('Pilih bulan terlebih dahulu', 'error');

    try {
        const records = await invoke('get_data_for_laporan', {
            tahun: parseInt(tahun),
            bulan: bulan,
            putaran: putaran,
        });

        laporanState.tahun = parseInt(tahun);
        laporanState.bulan = bulan;
        laporanState.putaran = putaran;
        laporanState.records = records || [];
        laporanState.generatedSections.clear();

        const infoBar = document.getElementById('laporanInfoBar');
        const infoText = document.getElementById('laporanInfoText');
        const cards = document.getElementById('laporanCards');
        const exportBar = document.getElementById('laporanExportBar');

        if (records && records.length > 0) {
            infoText.textContent = `${records.length} pos sampling terdeteksi untuk Tahun ${tahun} - ${bulan} Putaran ke-${putaran}`;
            infoBar.style.display = 'flex';
            cards.style.display = 'block';
            exportBar.style.display = 'flex';

            // Reset textareas
            document.querySelectorAll('.card-textarea').forEach(ta => ta.value = '');

            showToast(`Data ditemukan: ${records.length} pos`, 'success');
        } else {
            infoText.textContent = `Tidak ada data untuk Tahun ${tahun} - ${bulan} Putaran ke-${putaran}`;
            infoBar.style.display = 'flex';
            cards.style.display = 'none';
            exportBar.style.display = 'none';
            showToast('Tidak ada data untuk kombinasi tersebut', 'warning');
        }
    } catch (err) {
        console.error('Gagal load data:', err);
        showToast('Gagal memuat data: ' + err, 'error');
    }
}

async function handleGenerate(section) {
    const btn = document.querySelector(`.btn-ai-generate[data-section="${section}"]`);
    const card = btn?.closest('.laporan-card');
    const loading = card?.querySelector('.card-loading');
    const textarea = card?.querySelector('.card-textarea');

    if (!btn || !loading || !textarea) return;

    if (!laporanState.records || laporanState.records.length === 0) {
        return showToast('Muat data terlebih dahulu', 'error');
    }

    btn.disabled = true;
    loading.style.display = 'flex';

    try {
        const result = await invoke('generate_ai_content', {
            section: section,
            tahun: laporanState.tahun,
            bulan: laporanState.bulan,
            putaran: laporanState.putaran,
            dataRecords: laporanState.records,
        });

        textarea.value = stripMarkdown(result);
        laporanState.generatedSections.add(section);
        showToast(`✅ ${getSectionLabel(section)} berhasil digenerate`, 'success');
    } catch (err) {
        console.error('Gagal generate:', err);
        showToast('❌ Gagal generate: ' + err, 'error');
    } finally {
        btn.disabled = false;
        loading.style.display = 'none';
    }
}

async function handleExportWord() {
    const sections = getSectionsFromTextareas();

    // Validate
    const emptySections = Object.entries(sections)
        .filter(([_, v]) => !v || v.trim() === '')
        .map(([k]) => getSectionLabel(k));

    if (emptySections.length > 0) {
        return showToast(`Generate konten terlebih dahulu: ${emptySections.join(', ')}`, 'error');
    }

    try {
        const result = await invoke('export_laporan_word', {
            sections: sections,
            tahun: laporanState.tahun,
            bulan: laporanState.bulan,
            putaran: laporanState.putaran,
            dataRecords: laporanState.records,
        });
        showToast('✅ ' + result, 'success');
    } catch (err) {
        if (!String(err).includes('dibatalkan')) {
            showToast('❌ Gagal export: ' + err, 'error');
        }
    }
}

function getSectionsFromTextareas() {
    const sections = {};
    document.querySelectorAll('.card-textarea').forEach(ta => {
        sections[ta.dataset.section] = ta.value;
    });
    return sections;
}

function stripMarkdown(text) {
    return text
        .replace(/\*\*\*(.*?)\*\*\*/gs, '$1')
        .replace(/\*\*(.*?)\*\*/gs, '$1')
        .replace(/\*(.*?)\*/gs, '$1')
        .replace(/__(.*?)__/gs, '$1')
        .replace(/^#{1,6}\s+/gm, '')
        .replace(/`(.*?)`/gs, '$1')
        .replace(/^[\s]*[-*+]\s+/gm, '')
        .replace(/^\s*\d+\.\s+/gm, '');
}

function getSectionLabel(section) {
    const labels = {
        cover: 'Cover',
        kata_pengantar: 'Kata Pengantar',
        bab1_1: 'BAB I.1 (Latar Belakang)',
        bab1_2: 'BAB I.2 (Maksud dan Tujuan)',
        bab2_1: 'BAB II.1 (Waktu dan Tempat)',
        bab2_2: 'BAB II.2 (Parameter yang Diuji)',
        bab2_3: 'BAB II.3 (Alat dan Bahan)',
        bab3_1: 'BAB III.1 (Kesimpulan)',
        bab3_2: 'BAB III.2 (Saran)',
    };
    return labels[section] || section;
}

let toastTimer = null;

function showToast(message, type = 'info') {
    const toast = document.getElementById('laporanToast');
    if (!toast) return;

    toast.textContent = message;
    toast.className = 'laporan-toast';
    toast.classList.add(`toast-${type}`);
    toast.style.display = 'block';

    if (toastTimer) clearTimeout(toastTimer);
    toastTimer = setTimeout(() => {
        toast.style.display = 'none';
    }, 5000);
}
