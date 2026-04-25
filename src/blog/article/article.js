(function () {
    const cover = document.querySelector('.post-cover');
    const wrap = document.querySelector('.post-wrap');
    if (!cover || !wrap) return;
    const io = new IntersectionObserver(([entry]) => {
        wrap.classList.toggle('is-past-cover', !entry.isIntersecting);
    });
    io.observe(cover);
})();
