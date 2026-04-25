(function () {
    var root = document.documentElement;
    var KEY = 'theme';
    var stored = null;
    try { stored = localStorage.getItem(KEY); } catch (e) {}
    if (stored === 'dark' || stored === 'light') {
        root.setAttribute('data-theme', stored);
    }
    var btn = document.querySelector('.site-header-theme');
    if (!btn) return;
    function current() {
        var attr = root.getAttribute('data-theme');
        if (attr === 'dark' || attr === 'light') return attr;
        return matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
    }
    function sync() { btn.setAttribute('aria-pressed', current() === 'dark' ? 'true' : 'false'); }
    sync();
    btn.addEventListener('click', function () {
        var next = current() === 'dark' ? 'light' : 'dark';
        root.setAttribute('data-theme', next);
        try { localStorage.setItem(KEY, next); } catch (e) {}
        sync();
    });
})();
