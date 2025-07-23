const hyperImages = /** @type {NodeListOf<HTMLElement>} */ (
    document.querySelectorAll(".hyper-img")
);

for (const hyperImg of hyperImages) {
    /** @type {string[]} */
    let history = [];

    let initial = getCurrent(hyperImg);
    if (initial) { activatePage(hyperImg,history, initial); }
    console.log(history)

    /** @type {HTMLButtonElement[]} */
    const buttons = /***/([...hyperImg.querySelectorAll("button")]);

    for (const button of buttons) {
        let directive = button.getAttribute("href");
        if (!directive) {
            continue;
        }
        button.addEventListener("click", (event) => {
            if (directive === '#Back') {
                deactivatePage(hyperImg, history);
            } else if (directive.startsWith('#Back:')) {
                deactivatePage(hyperImg, history);
                // @ts-ignore
                activatePage(hyperImg, history, hyperImg.querySelector(`.hi-page-wrapper[for="${directive.replace("#Back:","")}"]`));
            } else {
                // @ts-ignore
                activatePage(hyperImg, history, hyperImg.querySelector(`.hi-page-wrapper[for="${directive}"]`))
            }
            console.info(history)
        })
    }
}

/**
 * 
 * @param {HTMLElement} hyperImg 
 * @returns {HTMLElement?}
 */
function getCurrent(hyperImg) {
    return hyperImg.querySelector('.hi-page-wrapper:has(.hi-page[data-active="true"])')
}

/**
 * @param {HTMLElement} hyperImg 
 * @param {String[]} history 
 * @param {HTMLElement} pageWrapper 
 */
function activatePage(hyperImg, history, pageWrapper) {
    const id = pageWrapper.getAttribute("for");
    if (id && history[history.length - 1] !== id) {
        history.push(id);
    }
    pageWrapper.style.setProperty(`--z-i`, `${history.length - 1}`);
    for (const page of /** @type {NodeListOf<HTMLElement>} */(hyperImg.querySelectorAll(".hi-page"))){
        page.style.setProperty(`--z-i`, `-1`);
    }
    pageWrapper.querySelector(".hi-page")?.setAttribute("data-active", `${true}`);
}

/**
 * @param {HTMLElement} hyperImg 
 * @param {String[]} history 
 */
function deactivatePage(hyperImg, history) {
    const page = hyperImg.querySelector(`#${history.pop()}`);
    page?.parentElement?.style.setProperty(`--z-i`, `-1`);
}