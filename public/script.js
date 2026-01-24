"use strict";

//#region Dismiss
/**
 * Event listener für Klicks auf das Dokument.
 * Schließt alle geöffneten <details class="dismiss">-Elemente,
 * wenn außerhalb des Elements geklickt wird.
 */
document.addEventListener("click", (event) => {
    /** @type {NodeListOf<HTMLDetailsElement>} */
    const openDetails = document.querySelectorAll("details.dismiss[open]");

    openDetails.forEach((details) => {
        const target = event.target;
        if (!(target instanceof Node)) return;

        // Prüfen, ob der Klick außerhalb des <details> war
        if (!details.contains(target)) {
            details.removeAttribute("open");
        }
    });
});

/**
 * Event listener für Tastatur-Events (z. B. Escape-Taste).
 * Schließt alle geöffneten <details class="dismiss">-Elemente,
 * wenn die Escape-Taste gedrückt wird.
 */
document.addEventListener("keydown", (event) => {
    if (event.key === "Escape") {
        /** @type {NodeListOf<HTMLDetailsElement>} */
        const openDetails = document.querySelectorAll("details.dismiss[open]");

        openDetails.forEach((details) => {
            details.removeAttribute("open");

            // Optional: Fokus zurück auf <summary> setzen
            const summary = details.querySelector("summary");
            if (summary instanceof HTMLElement) {
                summary.focus();
            }
        });
    }
});

//#endregion Dismiss

//#region ToTop

/** min scroll in px bis sichtbar */
const toTopButtonDelta = 100;
const toTop = /** @type {HTMLAnchorElement} */(document.getElementById("ReturnToTop"));
toTop.style.visibility = "hidden";

window.addEventListener("scroll", (ev) => {
    if (window.pageYOffset < toTopButtonDelta) {
        toTop.style.visibility = "hidden";
    } else {
        toTop.style.visibility = "visible";
    }
})

//#endregion ToTop

//#region noDrag

const noDrags = /** @type {NodeListOf<HTMLElement>} */(document.querySelectorAll('[draggable="false"]'));


noDrags.forEach(el => {
    let isDragging = false;
    let startX = 0;
    let startY = 0;
    const dragThresholdPixel = 5; // Mindestbewegung in Pixeln

    el.addEventListener("mousedown", (e) => {
        e = (e);
        isDragging = false;
        startX = e.clientX;
        startY = e.clientY;
    });

    el.addEventListener("mousemove", (e) => {
        const dx = Math.abs(e.clientX - startX);
        const dy = Math.abs(e.clientY - startY);
        if (dx > dragThresholdPixel || dy > dragThresholdPixel) {
            isDragging = true;
        }
    });

    el.addEventListener("click", (event) => {
        if (isDragging) {
            event.preventDefault();
        }
    });
});

//#endregion noDrag

//#region DodgeHeader

const dodgers = /** @type {NodeListOf<HTMLElement>} */(document.querySelectorAll(".dodge-header"));
const headerBar = /** @type {HTMLElement} */(document.querySelector("#SiteHeader"));
const headerDetails = /** @type {NodeListOf<HTMLDetailsElement>} */(headerBar.querySelectorAll("nav details"));

/**
 * 
 * @param {HTMLElement} dodger 
 */
function setDodgerHeight(dodger) {
    if (headerBar.clientHeight >= window.innerHeight) {
        dodger.style.height = "0";
        return;
    }
    console.log(headerBar)
    console.log(headerBar.clientHeight)
    dodger.style.height = String(headerBar.clientHeight) + "px";
}

headerDetails.forEach((details) => {
    details.addEventListener("toggle", function (e) {
        // if (details.open) {
        //     console.log("open: ", e)
        // } else {
        //     console.log("close: ", e)
        // }
        dodgers.forEach(el => {
            const transitionDuration = window.getComputedStyle(el).transitionDuration;
            const durationInMilliseconds = parseFloat(transitionDuration) * (transitionDuration.includes("s") ? 1000 : 1);
            setDodgerHeight(el)
            let interval = window.setInterval( setDodgerHeight, 50, el)
            window.setTimeout(() => {
                window.clearInterval(interval)
            }, durationInMilliseconds+10);
        });
    })
})
dodgers.forEach(setDodgerHeight)

//#endregion DodgeHeader

//#region Parsing
/**
 * 
 * @param {string} jsonc 
 * @returns 
 */
export function parseJSONC(jsonc) {
    // Entfernt einzeilige Kommentare: //
    jsonc = jsonc.replace(/\/\/.*$/gm, '');

    // Entfernt mehrzeilige Kommentare: /* ... */
    jsonc = jsonc.replace(/\/\*[\s\S]*?\*\//g, '');

    // JSON parsen
    return JSON.parse(jsonc);
}
//#endregion Parsing

//#region Truncating

/**
 * 
 * @param {string} str 
 * @param {string} search 
 * @param {string} replace 
 * @returns {string}
 */
export function replaceLast(str, search, replace) {
    const index = str.lastIndexOf(search);
    if (index === -1) return str; // nichts gefunden
    return str.substring(0, index) + replace + str.substring(index + search.length);
}

/**
 * 
 * @param {string} text 
 * @param {string} [from] im Format "Zeile:Spalte" (0-indiziert)
 * @param {string} [to] im Format "Zeile:Spalte" (0-indiziert)
 * @returns {string}
 */
export function extractRangeByLineColumn(text, from, to) {
    const lines = text.split('\n');

    let fromLine = 0, fromCol = 0;
    let toLine = lines.length - 1, toCol = lines[lines.length - 1].length;

    if (from) {
        const [lineStr, colStr] = from.split(':');
        const line = parseInt(lineStr?.trim(), 10);
        const col = parseInt(colStr?.trim(), 10);

        if (isNaN(line) || line < 0 || line >= lines.length) {
            console.error(`Ungültiger 'from'-Zeilenindex: "${from}"`);
        } else {
            fromLine = line;
        }

        if (!isNaN(col) && col >= 0) {
            fromCol = col;
        }
    }

    if (to) {
        const [lineStr, colStr] = to.split(':');
        const line = parseInt(lineStr?.trim(), 10);
        const col = parseInt(colStr?.trim(), 10);

        if (isNaN(line) || line < 0 || line >= lines.length) {
            console.error(`Ungültiger 'to'-Zeilenindex: "${to}"`);
        } else {
            toLine = line;
        }

        if (!isNaN(col) && col >= 0) {
            toCol = col;
        }
    }

    if (toLine < fromLine || (toLine === fromLine && toCol < fromCol)) {
        console.error(`'to' (${to}) liegt vor 'from' (${from}) - leerer Ausschnitt zurückgegeben.`);
        return '';
    }

    const extracted = lines.slice(fromLine, toLine + 1);

    // Erste und Letzte Zeile zuschneiden
    if (extracted.length > 0) {
        extracted[0] = extracted[0].substring(fromCol);
        if (extracted.length > 1 || toLine === fromLine) {
            extracted[extracted.length - 1] = extracted[extracted.length - 1].substring(0, toCol);
        }
    }

    return extracted.join('\n');
}
//#endregion Truncating

//#region ?
/**
 * 
 * @param {HTMLElement} el 
 * @param {string} varname 
 */
export function getCSSVar(el, varname) {
    return getComputedStyle(el).getPropertyValue(varname).trim()
}

export const changeEvent = new Event('change', {
    bubbles: true,
    cancelable: true,
    composed: true,
});
//#endregion ?

//#region HoverBorder
const [...hoverBorderTargets] = document.querySelectorAll("[hover-border-target]");
hoverBorderTargets
    .map(el =>
        /** @type {[HTMLElement,HTMLElement|null]} */
        ([el, document.getElementById(el.getAttribute("hover-border-target") ?? "")])
    )
    .forEach(
        /** @param {[HTMLElement,HTMLElement|null]} param0 */
        ([el, target]) => {
            el.addEventListener("mouseenter", () => {
                target?.classList.add("accent-border");
            });
            el.addEventListener("mouseleave", () => {
                target?.classList.remove("accent-border");
            })
        });

const [...hoverBorderMultiTargets] = document.querySelectorAll("[hover-border-targets]");
hoverBorderMultiTargets
    .map(el =>
        /** @type {[HTMLElement,[HTMLElement|null]]} */
        ([el, el.getAttribute("hover-border-targets")?.split(",").map(str => str.trim()).map(str => document.getElementById(str))])
    )
    .forEach(
        /** @param {[HTMLElement,[HTMLElement|null]]} param0 */
        ([el, targets]) => {
            console.log(targets)
            el.addEventListener("mouseenter", () => {
                targets?.forEach(target => { target?.classList.add("accent-border"); })
            });
            el.addEventListener("mouseleave", () => {
                targets?.forEach(target => { target?.classList.remove("accent-border"); })
            })
        });
//#endregion HoverBorder

//#region linkToScroll
/** 
 * @type {NodeListOf<HTMLAnchorElement>} 
 */
const links = document.querySelectorAll('#Languages a.link');

links.forEach(link => {
    link.addEventListener('click', function (e) {
        const scrollY = window.scrollY || document.documentElement.scrollTop;
        // Verhindern, dass der Link sofort lädt
        e.preventDefault();

        // Aktuelle URL mit Scroll-Position erweitern (als Query)
        const url = new URL(this.href, window.location.origin);
        url.searchParams.set('scroll', scrollY.toString());

        // Jetzt navigieren
        window.location.href = url.toString();
    });
});
window.addEventListener('DOMContentLoaded', () => {
    const params = new URLSearchParams(window.location.search);
    const scroll = params.get('scroll');
    if (scroll) {
        window.scrollTo({ top: parseInt(scroll, 10), behavior: 'instant' });
    }
});
//#endregion linkToScroll

//#region webGPU?
export function supportsWebGPU() {
    return typeof navigator !== "undefined" && !!navigator.gpu
}
//#endregion webGPU?
