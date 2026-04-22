(() => {
    try {
        const stored = localStorage.getItem('preview-theme');
        if (stored === 'light' || stored === 'dark') {
            document.documentElement.setAttribute('data-theme', stored);
        }
    } catch (_) {}
})();
