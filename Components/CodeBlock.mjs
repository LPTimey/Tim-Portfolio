"use strict";
import { css, extractRangeByLineColumn, html, replaceLast, lightDark } from "../script.mjs";

// @ts-ignore
/** @import * from "../vendor/highlight.js/11.11.1/index" */
import hljs from "../vendor/highlight.js/11.11.1/cdn-release-11-stable/build/es/highlight.js"

/**
 * 
 * @param {string} lang 
 * @param {string} src 
 * @param {boolean} no_pre 
 * @returns 
 */
const template = (lang, src, no_pre) => html`
${no_pre ? "" : html`<pre>`}<code class="language-${lang}">${src}</code>${no_pre ? "" : html`</pre>`}
`;

/**
 * 
 * @param {string} theme 
 * @returns 
 */
const style = (theme) => css`
@import "setup.css";
@import "./vendor/highlight.js/11.11.1/cdn-release-11-stable/build/styles/${theme}.min.css";
:host{
    display: block;
    font-family: "JetBrains Mono", monospace;
    border-radius: var(--border-r);
    overflow: auto;
}
pre,code{
    height: 100%;
    width:  100%;
}
`;

export default class CodeBlock extends HTMLElement {
    static get observedAttributes() {
        return ["lang", "src", "from", "to", "dark-theme", "light-theme", "no-pre", "no-todo"];
    }
    /**
     * 
     * @param {string} unsafe 
     * @returns 
     */
    static escapeHtml(unsafe) {
        return unsafe
            // .replaceAll("&", "&amp;")
            .replaceAll("<", "&lt;")
            .replaceAll(">", "&gt;")
            .replaceAll('"', "&quot;")
            .replaceAll("'", "&#039;");
    };

    get theme() {
        let light = this.getAttribute("light-theme") ?? "atom-one-light";
        let dark = this.getAttribute("dark-theme") ?? "atom-one-dark";

        let el = this;

        return lightDark(el, light, dark);
    }

    emitTheme() {
        this.shadowRoot?.dispatchEvent(new Event('theme-change'));
    }
    /**
    * 
    * @param {string} name name of attribute
    * @param {*} oldValue old value of attribute
    * @param {*} newValue new value of attribute
    */
    attributeChangedCallback(name, oldValue, newValue) {
        if (!this.initialized) {
            return;
        }
        switch (name) {
            case "lang": {
                let code = this.shadowRoot?.querySelector("code");
                code?.classList.replace(`language-${oldValue}`, `language-${newValue}`)
                break;
            }
            case "dark-theme":
            case "light-theme": {
                // @ts-ignore
                this.shadowRoot.querySelector("style").outerHTML = style(this.theme);
            }
            case "no-pre": {
                this.render();
            }
            default: {
                this.render();
                break;
            }
        }
        return;
    }

    /**
     * 
     * @param {string} content 
     * @param {boolean} hasNoTodoAttr 
     * @returns {string}
     */
    removeTodoComments(content, hasNoTodoAttr = false) {
        if (!hasNoTodoAttr) {
            return content; // keine Änderungen wenn "no-todo" nicht gesetzt ist
        }

        // Schritt 1: Entferne HTML-Kommentare mit TODO/FIXME
        content = content.replace(/<!--\s*(TODO|FIXME).*?-->/gsi, '');

        // Schritt 2: Entferne mehrzeilige JS-Kommentare mit TODO/FIXME
        content = content.replace(/\/\*\s*(TODO|FIXME)[\s\S]*?\*\//gsi, '');

        // Schritt 3: Entferne einzeilige JS-Kommentare mit TODO/FIXME
        content = content.replace(/\/\/\s*(TODO|FIXME).*$/gmi, '');

        return content;
    }

    async render() {
        const lang = this.getAttribute("lang") ?? "plaintext";
        const noPre = this.hasAttribute("no-pre");
        let content = "";

        if (this.getAttribute("src")) {
            content = await fetch(this.getAttribute("src") ?? "").then(resp => resp.text());
            content = extractRangeByLineColumn(content, this.getAttribute("from") ?? undefined, this.getAttribute("to") ?? undefined);
            if (content.includes("struct GameState")) console.log(content);
        }

        let inner = this.innerHTML.trimEnd();
        inner = inner.replace("<pre>", "\n");
        inner = replaceLast(inner, "</pre>", "\n");

        if (this.hasAttribute("prefix")) {
            content = "\n" + inner.trimEnd() + "\n" + content;
        } else {
            content += "\n" + inner;
        }

        content = content.replace("<pre>", "\n");
        content = replaceLast(content, "</pre>", "\n");
        content = CodeBlock.escapeHtml(content);

        let lines = this.removeTodoComments(content, this.hasAttribute("no-todo")).split("\n");
        /** @type {number?} */
        let minSpace = null;

        content = lines
            .filter((line, index, arr) => {
                if (index === 0 || index === arr.length - 1) {
                    return line.trim() !== '';
                }
                return true;
            })
            .map(line => {
                let newLine = line.trimStart();
                if (newLine.length !== 0) { // ignore empty lines
                    let delta = line.length - newLine.length;
                    if (minSpace === null) {
                        minSpace = delta;
                    }
                    minSpace = Math.min(minSpace, delta);
                }
                return line.trimEnd();
            })
            .map(line => line.slice(minSpace ?? 0))
            .join("\n")
            .replace(/(\s*\n){2,}/g, '\n\n')// ersetzt 2+ leere Zeilen durch genau eine
            .replace(/\n\n(?=\s*[^a-zA-Z0-9]+\s*$)/gm, '\n') // Entferne gezielt Leerzeilen VOR Sonderzeichenzeilen
            .trim();

        this.shadowRoot.innerHTML = style(this.theme) + template(lang, content, noPre);

        /** @type {HTMLElement|null} */
        const code = /** @type {HTMLElement|null} */ (this.shadowRoot?.querySelector("code"));
        if (code) hljs.highlightElement(code);
    }

    connectedCallback() {
        this._contentObserver.observe(this, {
            childList: true,
            characterData: true,
            subtree: true
        });
        // Eltern beobachten
        /** @type {HTMLElement | null} */
        let current = this;
        while (current) {
            this._themeObserver.observe(current, { attributes: true, attributeFilter: ['class'], childList: true, subtree: false });
            current = current.parentElement;
        }

        this.render();
        this.initialized = true;
    }

    constructor() {
        super();
        this.attachShadow({ mode: "open" });
        /** @type {ShadowRoot} */
        this.shadowRoot;
        this.initialized = false;

        this._contentObserver = new MutationObserver(() => this.render());
        this._themeObserver = new MutationObserver(() => this.emitTheme());

        this.shadowRoot.addEventListener("theme-change", () => {
            // console.log("event");
            this.render()
        });
    }
}
customElements.define("code-block", CodeBlock);