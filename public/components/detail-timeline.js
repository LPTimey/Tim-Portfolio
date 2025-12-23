const moreButtons = /** @type {NodeListOf<HTMLButtonElement>} */(document.querySelectorAll(".detail-timeline .dt-content .dt-short-content > button"))
const lessButtons = /** @type {NodeListOf<HTMLButtonElement>} */(document.querySelectorAll(".detail-timeline .dt-content .dt-long-content > button"))

moreButtons.forEach(btn => {
    btn.addEventListener("click", () => {
        const dtContent = btn.closest('.dt-content');
        if (!dtContent) { return }
        dtContent.setAttribute('data-open', "true");
    });
});
lessButtons.forEach(btn => {
    btn.addEventListener("click", () => {
        const dtContent = btn.closest('.dt-content');
        if (!dtContent) { return }
        dtContent.setAttribute('data-open', "false");
    });
});
