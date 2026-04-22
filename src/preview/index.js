(() => {
    const frame = document.querySelector('.preview-frame');
    const viewport = document.querySelector('.preview-viewport');
    const toolbar = document.querySelector('.preview-toolbar');
    const sizeLabel = toolbar.querySelector('.preview-size');
    const navLinks = Array.from(document.querySelectorAll('.preview-sidebar a[data-path]'));
    const deviceButtons = Array.from(toolbar.querySelectorAll('button[data-device]'));

    const STORAGE_KEY = 'preview-device';

    function showPath(path) {
        if (!path) {
            frame.src = 'about:blank';
            navLinks.forEach(a => a.classList.remove('active'));
            return;
        }
        const target = '/p/' + path.split('/').map(encodeURIComponent).join('/') + '/';
        if (frame.getAttribute('src') !== target) frame.src = target;
        navLinks.forEach(a => a.classList.toggle('active', a.dataset.path === path));
    }

    function setDevice(name) {
        const btn = deviceButtons.find(b => b.dataset.device === name) || deviceButtons[0];
        const width = parseInt(btn.dataset.width, 10) || 0;
        deviceButtons.forEach(b => b.setAttribute('aria-pressed', b === btn ? 'true' : 'false'));
        viewport.classList.toggle('full', width === 0);
        viewport.classList.toggle('constrained', width > 0);
        if (width > 0) {
            viewport.style.setProperty('--device-width', width + 'px');
            sizeLabel.textContent = width + 'px';
        } else {
            viewport.style.removeProperty('--device-width');
            sizeLabel.textContent = '';
        }
        try { localStorage.setItem(STORAGE_KEY, btn.dataset.device); } catch (_) {}
    }

    deviceButtons.forEach(b => b.addEventListener('click', () => setDevice(b.dataset.device)));

    let saved = 'full';
    try { saved = localStorage.getItem(STORAGE_KEY) || 'full'; } catch (_) {}
    setDevice(saved);

    function fromHash() {
        const h = location.hash.slice(1);
        try { return decodeURIComponent(h); } catch (_) { return h; }
    }

    const initial = fromHash() || (navLinks[0] && navLinks[0].dataset.path) || '';
    if (!location.hash && initial) history.replaceState(null, '', '#' + initial);
    showPath(initial);

    addEventListener('hashchange', () => showPath(fromHash()));
})();
