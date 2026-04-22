(() => {
    let last = null;
    async function poll() {
        try {
            const r = await fetch('/_reload');
            const v = await r.text();
            if (last !== null && v !== last) {
                location.reload();
                return;
            }
            last = v;
        } catch (e) { }
        setTimeout(poll, 500);
    }
    poll();
})();
