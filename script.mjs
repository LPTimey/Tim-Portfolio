"use strict";
export function noOpTag(strings, ...values) {
    // Füge alle Strings zusammen und setze Werte in den Platzhalter ein
    /** @type {string} */
    let res = strings.reduce((result, string, i) => {
        // Wenn es einen Wert gibt, füge ihn hinzu, andernfalls füge einfach den String hinzu
        let new_res = result + string + (values[i] !== undefined ? values[i] : '');
        return new_res;
    }, '');
    return res;
}

export function css(strings, ...values) {
    return "<style>" + noOpTag(strings, ...values) + "</style>";
}
export function html(strings, ...values) {
    return noOpTag(strings, ...values);
}

export function parseJSONC(jsonc) {
    // Entfernt einzeilige Kommentare: //
    jsonc = jsonc.replace(/\/\/.*$/gm, '');

    // Entfernt mehrzeilige Kommentare: /* ... */
    jsonc = jsonc.replace(/\/\*[\s\S]*?\*\//g, '');

    // JSON parsen
    return JSON.parse(jsonc);
}

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
 * @param {string} [from?] im Format "Zeile:Spalte" 0 indiziert
 * @param {string} [to?] im Format "Zeile:Spalte" 0 indiziert
 * @returns {string}
 */
export function extractRangeByLineColumn(text, from, to) {
    let [fromLine, fromCol] = [null, null];
    let [toLine, toCol] = [null, null];
    if (!from && !to) {
        return text;
    }
    if (from) {
        [fromLine, fromCol] = from.split(':').map(str => str.trim()).map(Number || null);
    }
    if (to) {
        [toLine, toCol] = to.split(':').map(str => str.trim()).map(Number || null);
    }

    // TODO: Fehlerbehandlung: Indexbereich prüfen

    const lines = text.split('\n').map((str, i) => { return { str, i } });

    let result;

    result = lines
        .filter(obj => obj.i >= fromLine ?? 0)
        .filter(obj => toLine ? (obj.i <= toLine) : true)
    //FIXME: indexingdoesn't work because of filter nee obj.i

    let fromLineIndex = result.findIndex(obj => obj.i == fromLine ?? 0);
    let toLineIndex = result.findIndex(obj => toLine ? (obj.i <= toLine) : false);
    result[fromLineIndex].str = result[fromLineIndex].str.substring(fromCol ?? 0);
    if (toCol && toLineIndex >= 0) result[toLineIndex].str = result[toLineIndex].str.substring(0, toCol);

    return result.map(obj => obj.str).join("\n");
}

/**
 * @template T
 * @template K
 * @param {Element} el 
 * @param {T} light 
 * @param {K} dark 
 * @returns {light | dark}
 */
export function lightDark(el, light, dark) {
    const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
    let closestThemeElement = null;

    while (el) {
        if (el.classList?.contains('light') || el.classList?.contains('dark') || el.classList?.contains('system')) {
            closestThemeElement = el;
            break;
        }
        el = el.parentElement;
    }

    if (!closestThemeElement || closestThemeElement.classList.contains('system')) {
        return prefersDark ? dark : light;
    } else if (closestThemeElement.classList.contains('light')) {
        return light;
    } else if (closestThemeElement.classList.contains('dark')) {
        return dark;
    }

    return prefersDark ? dark : light;
}

/**
 * 
 * @param {HTMLElement} el 
 * @param {string} varname 
 */
export function getCSSVar(el, varname) {
    return getComputedStyle(el).getPropertyValue(varname).trim()
}
/**
 * 
 * @param {HTMLElement} el 
 * @param {string} varname 
 * @returns {number} color in hex
 */
export function getCSSLightDarkColor(el, varname) {
    let prop = getCSSVar(el, varname);

    const match = prop.match(/light-dark\(\s*(#[0-9a-fA-F]+)\s*,\s*(#[0-9a-fA-F]+)\s*\)/);
    if (!match) return parseInt(prop.slice(1), 16); // Kein light-dark(), gib original zurück

    const [, darkColor, lightColor] = match;

    return parseInt(lightDark(el, darkColor, lightColor).slice(1), 16)
}


export const changeEvent = new Event('change', {
    bubbles: true,
    cancelable: true,
    composed: true,
});

const tooltips = document.querySelectorAll("[data-tooltip]")
tooltips.forEach((el) => {
    let tooltip = document.createElement("div");
    tooltip.classList.add("tooltip");
    tooltip.innerHTML = el.getAttribute("data-tooltip");
    el.appendChild(tooltip);
});
// console.log(tooltips)

const [...hoverBorderTargets] = document.querySelectorAll("[hover-border-target]");
hoverBorderTargets.map(el => [el, document.getElementById(el.getAttribute("hover-border-target"))]).forEach(
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
    .map(el => [el, el.getAttribute("hover-border-targets").split(",").map(str => str.trim()).map(str => document.getElementById(str))])
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

/******************************\
 *                            *
 *            svg             *
 *                            *
\******************************/
//#region svg's

export const gh_logo = `
<svg class="gh_logo" height="32" aria-hidden="true" viewBox="0 0 24 24" version="1.1" width="32" data-view-component="true">
    <path d="M12.5.75C6.146.75 1 5.896 1 12.25c0 5.089 3.292 9.387 7.863 10.91.575.101.79-.244.79-.546 0-.273-.014-1.178-.014-2.142-2.889.532-3.636-.704-3.866-1.35-.13-.331-.69-1.352-1.18-1.625-.402-.216-.977-.748-.014-.762.906-.014 1.553.834 1.769 1.179 1.035 1.74 2.688 1.25 3.349.948.1-.747.402-1.25.733-1.538-2.559-.287-5.232-1.279-5.232-5.678 0-1.25.445-2.285 1.178-3.09-.115-.288-.517-1.467.115-3.048 0 0 .963-.302 3.163 1.179.92-.259 1.897-.388 2.875-.388.977 0 1.955.13 2.875.388 2.2-1.495 3.162-1.179 3.162-1.179.633 1.581.23 2.76.115 3.048.733.805 1.179 1.825 1.179 3.09 0 4.413-2.688 5.39-5.247 5.678.417.36.776 1.05.776 2.128 0 1.538-.014 2.774-.014 3.162 0 .302.216.662.79.547C20.709 21.637 24 17.324 24 12.25 24 5.896 18.854.75 12.5.75Z"></path>
</svg>`;
export const burger_icon = `
<svg xmlns="http://www.w3.org/2000/svg" height="22px" viewBox="0 -960 960 960" width="24px">
    <path d="M160-240q-17 0-28.5-11.5T120-280q0-17 11.5-28.5T160-320h640q17 0 28.5 11.5T840-280q0 17-11.5 28.5T800-240H160Zm0-200q-17 0-28.5-11.5T120-480q0-17 11.5-28.5T160-520h640q17 0 28.5 11.5T840-480q0 17-11.5 28.5T800-440H160Zm0-200q-17 0-28.5-11.5T120-680q0-17 11.5-28.5T160-720h640q17 0 28.5 11.5T840-680q0 17-11.5 28.5T800-640H160Z"/>
</svg>
`;
export const close_icon = `<svg xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 -960 960 960" width="24px">
    <path d="M480-424 284-228q-11 11-28 11t-28-11q-11-11-11-28t11-28l196-196-196-196q-11-11-11-28t11-28q11-11 28-11t28 11l196 196 196-196q11-11 28-11t28 11q11 11 11 28t-11 28L536-480l196 196q11 11 11 28t-11 28q-11 11-28 11t-28-11L480-424Z"/>
</svg>`;
export const light_mode_icon = `
<svg xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 -960 960 960" width="24px">
    <path d="M480-360q50 0 85-35t35-85q0-50-35-85t-85-35q-50 0-85 35t-35 85q0 50 35 85t85 35Zm0 80q-83 0-141.5-58.5T280-480q0-83 58.5-141.5T480-680q83 0 141.5 58.5T680-480q0 83-58.5 141.5T480-280ZM200-440H40v-80h160v80Zm720 0H760v-80h160v80ZM440-760v-160h80v160h-80Zm0 720v-160h80v160h-80ZM256-650l-101-97 57-59 96 100-52 56Zm492 496-97-101 53-55 101 97-57 59Zm-98-550 97-101 59 57-100 96-56-52ZM154-212l101-97 55 53-97 101-59-57Zm326-268Z"/>
</svg>`;
export const dark_mode_icon = `
<svg xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 -960 960 960" width="24px">
    <path d="M480-120q-150 0-255-105T120-480q0-150 105-255t255-105q14 0 27.5 1t26.5 3q-41 29-65.5 75.5T444-660q0 90 63 153t153 63q55 0 101-24.5t75-65.5q2 13 3 26.5t1 27.5q0 150-105 255T480-120Zm0-80q88 0 158-48.5T740-375q-20 5-40 8t-40 3q-123 0-209.5-86.5T364-660q0-20 3-40t8-40q-78 32-126.5 102T200-480q0 116 82 198t198 82Zm-10-270Z" />
</svg>`;
export const system_mode_icon = `
<svg xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 -960 960 960" width="24px">
    <path d="M80-160v-120h80v-440q0-33 23.5-56.5T240-800h600v80H240v440h240v120H80Zm520 0q-17 0-28.5-11.5T560-200v-400q0-17 11.5-28.5T600-640h240q17 0 28.5 11.5T880-600v400q0 17-11.5 28.5T840-160H600Zm40-120h160v-280H640v280Zm0 0h160-160Z"/>
</svg>`;
export const to_top_icon = `
<svg xmlns="http://www.w3.org/2000/svg" height="24px" width="24px" viewBox="0 -960 960 960" width="24px">
    <path d="M160-760v-80h640v80H160Zm280 640v-408L336-424l-56-56 200-200 200 200-56 56-104-104v408h-80Z"/>
</svg>
`;

export const pause_icon = `
<svg xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 -960 960 960" width="24px">
    <path d="M600-200q-33 0-56.5-23.5T520-280v-400q0-33 23.5-56.5T600-760h80q33 0 56.5 23.5T760-680v400q0 33-23.5 56.5T680-200h-80Zm-320 0q-33 0-56.5-23.5T200-280v-400q0-33 23.5-56.5T280-760h80q33 0 56.5 23.5T440-680v400q0 33-23.5 56.5T360-200h-80Zm320-80h80v-400h-80v400Zm-320 0h80v-400h-80v400Zm0-400v400-400Zm320 0v400-400Z"/>
</svg>
`
export const pause_circle_icon = `
<svg xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 -960 960 960" width="24px">
    <path d="M400-320q17 0 28.5-11.5T440-360v-240q0-17-11.5-28.5T400-640q-17 0-28.5 11.5T360-600v240q0 17 11.5 28.5T400-320Zm160 0q17 0 28.5-11.5T600-360v-240q0-17-11.5-28.5T560-640q-17 0-28.5 11.5T520-600v240q0 17 11.5 28.5T560-320ZM480-80q-83 0-156-31.5T197-197q-54-54-85.5-127T80-480q0-83 31.5-156T197-763q54-54 127-85.5T480-880q83 0 156 31.5T763-763q54 54 85.5 127T880-480q0 83-31.5 156T763-197q-54 54-127 85.5T480-80Zm0-80q134 0 227-93t93-227q0-134-93-227t-227-93q-134 0-227 93t-93 227q0 134 93 227t227 93Zm0-320Z"/>
</svg>
`
export const play_icon = `
<svg xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 -960 960 960" width="24px">
    <path d="M320-273v-414q0-17 12-28.5t28-11.5q5 0 10.5 1.5T381-721l326 207q9 6 13.5 15t4.5 19q0 10-4.5 19T707-446L381-239q-5 3-10.5 4.5T360-233q-16 0-28-11.5T320-273Zm80-207Zm0 134 210-134-210-134v268Z"/>
</svg>
`
export const play_circle_icon = `
<svg xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 -960 960 960" width="24px">
    <path d="m426-330 195-125q14-9 14-25t-14-25L426-630q-15-10-30.5-1.5T380-605v250q0 18 15.5 26.5T426-330Zm54 250q-83 0-156-31.5T197-197q-54-54-85.5-127T80-480q0-83 31.5-156T197-763q54-54 127-85.5T480-880q83 0 156 31.5T763-763q54 54 85.5 127T880-480q0 83-31.5 156T763-197q-54 54-127 85.5T480-80Zm0-80q134 0 227-93t93-227q0-134-93-227t-227-93q-134 0-227 93t-93 227q0 134 93 227t227 93Zm0-320Z"/>
</svg>
`

export const double_arrow = `
<svg xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 -960 960 960" width="24px">
    <path d="M442-480 287-697q-14-20-3.5-41.5T319-760q10 0 19 4.5t14 12.5l188 263-188 263q-5 8-14 12.5t-19 4.5q-24 0-35-21.5t3-41.5l155-217Zm238 0L525-697q-14-20-3.5-41.5T557-760q10 0 19 4.5t14 12.5l188 263-188 263q-5 8-14 12.5t-19 4.5q-24 0-35-21.5t3-41.5l155-217Z"/>
</svg>
`;

//#endregion svg's
