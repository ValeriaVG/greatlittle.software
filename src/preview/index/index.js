(() => {
    const frame = document.querySelector('.preview-frame');
    const viewport = document.querySelector('.preview-viewport');
    const toolbar = document.querySelector('.preview-toolbar');
    const sizeLabel = toolbar.querySelector('.preview-size');
    const navLinks = Array.from(document.querySelectorAll('.preview-sidebar a[data-path]'));
    const deviceButtons = Array.from(toolbar.querySelectorAll('button[data-device]'));

    const STORAGE_KEY = 'preview-device';
    const COLLAPSED_KEY = 'preview-collapsed-groups';

    const sidebarList = document.querySelector('.preview-sidebar .preview-list');
    const groupNodes = Array.from(sidebarList.querySelectorAll('.preview-group'));

    function groupKey(groupEl) {
        const parts = [groupEl.querySelector('span').textContent];
        const depth = parseInt(groupEl.dataset.depth, 10) || 0;
        let node = groupEl.previousElementSibling;
        let d = depth - 1;
        while (node && d >= 0) {
            if (node.classList.contains('preview-group')) {
                const nd = parseInt(node.dataset.depth, 10) || 0;
                if (nd === d) {
                    parts.unshift(node.querySelector('span').textContent);
                    d -= 1;
                }
            }
            node = node.previousElementSibling;
        }
        return parts.join('/');
    }

    function loadCollapsed() {
        try { return new Set(JSON.parse(localStorage.getItem(COLLAPSED_KEY) || '[]')); }
        catch (_) { return new Set(); }
    }

    function saveCollapsed(set) {
        try { localStorage.setItem(COLLAPSED_KEY, JSON.stringify(Array.from(set))); }
        catch (_) {}
    }

    const collapsedGroups = loadCollapsed();

    function applyCollapse() {
        const hideStack = [];
        let node = sidebarList.firstElementChild;
        while (node) {
            const depth = parseInt(node.dataset.depth, 10) || 0;
            while (hideStack.length && hideStack[hideStack.length - 1] >= depth) hideStack.pop();
            const hidden = hideStack.length > 0;
            node.classList.toggle('collapsed', hidden);
            if (node.classList.contains('preview-group')) {
                const key = groupKey(node);
                const isCollapsed = collapsedGroups.has(key);
                const toggle = node.querySelector('.preview-group-toggle');
                if (toggle) toggle.setAttribute('aria-expanded', isCollapsed ? 'false' : 'true');
                if (isCollapsed && !hidden) hideStack.push(depth);
            }
            node = node.nextElementSibling;
        }
    }

    groupNodes.forEach(g => {
        const toggle = g.querySelector('.preview-group-toggle');
        if (!toggle) return;
        toggle.addEventListener('click', () => {
            const key = groupKey(g);
            if (collapsedGroups.has(key)) collapsedGroups.delete(key);
            else collapsedGroups.add(key);
            saveCollapsed(collapsedGroups);
            applyCollapse();
        });
    });

    applyCollapse();

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
