"use strict";

// @ts-ignore
/** @import * from "../vendor/highlight.js/11.11.1/index" */
import hljs from "../vendor/highlight.js/11.11.1/cdn-release-11-stable/build/es/highlight.js"


// Optionaler globaler Pfad, falls nötig
// @ts-ignore
const pathToRoot = window.pathToRoot || "";

const mediaQuery = window.matchMedia("(prefers-color-scheme: dark)");

// CSS-Dateipfade
const themeLight = `${pathToRoot}/vendor/highlight.js/11.11.1/cdn-release-11-stable/build/styles/atom-one-light.min.css`;
const themeDark = `${pathToRoot}/vendor/highlight.js/11.11.1/cdn-release-11-stable/build/styles/atom-one-dark.min.css`;

// Erstelle <link> für Light Theme
const linkLight = document.createElement("link");
linkLight.rel = "stylesheet";
linkLight.href = themeLight;
linkLight.id = "hljs-theme-light";

// Erstelle <link> für Dark Theme (zunächst deaktiviert)
const linkDark = document.createElement("link");
linkDark.rel = "stylesheet";
linkDark.href = themeDark;
linkDark.id = "hljs-theme-dark";
linkDark.disabled = true;

// Füge beide <link>s in den <head> ein
document.head.appendChild(linkLight);
document.head.appendChild(linkDark);

// Highlight alle Codeblöcke
const blocks = /** @type {NodeListOf<HTMLElement>} */ (document.querySelectorAll(".codeblock"));
for (const block of blocks) {
    hljs.highlightElement(block);
}

function getCurrentHighlightTheme() {
    if (document.querySelector('#Dark:checked')) return "dark";
    if (document.querySelector('#Light:checked')) return "light";
    if (document.querySelector('#System:checked')) {
        return window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
    }
    if (document.querySelector('#Custom:checked')) {
        return "light"; //TODO: calc based on vars
    }

    // Fallback
    return "light";
}

/**
 * Theme setzen
 * @param {*} theme 
 */
function switchHighlightTheme(theme) {
    const light = document.getElementById("hljs-theme-light");
    const dark = document.getElementById("hljs-theme-dark");

    if (!(light instanceof HTMLLinkElement) || !(dark instanceof HTMLLinkElement)) return;

    light.disabled = theme !== "light";
    dark.disabled = theme !== "dark";
}

switchHighlightTheme(getCurrentHighlightTheme());

document.querySelectorAll('input[name="theme"]').forEach((radio) => {
    radio.addEventListener("change", () => {
        switchHighlightTheme(getCurrentHighlightTheme());
    });
});

mediaQuery.addEventListener("change", (event) => {
    if (document.querySelector('#System:checked')) {
        const newTheme = event.matches ? "dark" : "light";
        switchHighlightTheme(newTheme);
    }
});