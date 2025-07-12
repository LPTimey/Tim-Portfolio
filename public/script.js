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

const noDrags = document.querySelectorAll('[draggable="false"]');

noDrags.forEach(el => {
    let isDragging = false;
    el.addEventListener("mousedown", () => {
        isDragging = false;
    });
    el.addEventListener("mousemove", () => {
        isDragging = true;
    });
    el.addEventListener("click", (event) => {
        if (isDragging) {
            event.preventDefault()
        }
    })
});

//#endregion noDrag

//#region DodgeHeader

const dodgers = document.querySelectorAll(".dodge-header");
const headerBar = /** @type {HTMLElement} */(document.querySelector("#SiteHeader"));

window.addEventListener("DOMContentLoaded", () => {
    if (!dodgers || dodgers.length === 0 || !headerBar) {
        return;
    }
    const fn = () => {
        dodgers.forEach(el => {
            if (!(el instanceof HTMLElement)) { return; }
            if (headerBar.clientHeight >= window.innerHeight) {
                el.style.height = "0";
                return;
            }
            el.style.height = String(headerBar.clientHeight) + "px";
        });
    };
    fn()
    window.setInterval(fn, 500);
})

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