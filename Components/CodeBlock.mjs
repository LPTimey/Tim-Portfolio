"use strict";
import { css, html, replaceLast } from "../script.mjs";

/** @import * from "../vendor/highlight.js/11.11.1/index" */
import hljs from "../vendor/highlight.js/11.11.1/cdn-release-11-stable/build/es/highlight.js"

const template = (lang, src, no_pre) => html`
${no_pre ? "" : html`<pre>`}<code class="language-${lang}">${src}</code>${no_pre ? "" : html`</pre>`}
`;

const style = (theme) => css`
@import "setup.css";
@import "./vendor/highlight.js/11.11.1/cdn-release-11-stable/build/styles/${theme}.min.css";
:host{
    display: block;
    font-family: "JetBrains Mono", monospace;
}
`;

export default class CodeBlock extends HTMLElement {
    static get observedAttributes() {
        return ["lang", "src", "from", "to", "dark-theme", "light-theme", "no-pre"];
    }
    static escapeHtml(unsafe) {
        return unsafe
            // .replaceAll("&", "&amp;")
            .replaceAll("<", "&lt;")
            .replaceAll(">", "&gt;")
            .replaceAll('"', "&quot;")
            .replaceAll("'", "&#039;");
    };

    get theme() {
        const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;

        let light = this.getAttribute("light-theme") ?? "atom-one-light";
        let dark = this.getAttribute("dark-theme") ?? "atom-one-dark";

        let closestThemeElement = null;
        let el = this;

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

    emitTheme() {
        this.shadowRoot.dispatchEvent(new Event('theme-change'));
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
                let code = this.shadowRoot.querySelector("code");
                code.classList.replace(`language-${oldValue}`, `language-${newValue}`)
                break;
            }
            case "dark-theme":
            case "light-theme": {
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

    async render() {
        const lang = this.getAttribute("lang") ?? "plaintext";
        const noPre = this.hasAttribute("no-pre");
        let content = "";
        if (this.getAttribute("src")) {
            content = await fetch(this.getAttribute("src")).then(resp => resp.text());
            //TODO: limit from "from" to "to";
        } else {
            content = this.innerHTML;
            content = content.replace("<pre>", "\n");
            content = replaceLast(content, "</pre>", "\n");
        }
        content = CodeBlock.escapeHtml(content);
        console.log(content)

        let lines = content.split("\n");
        let minSpace = null;
        //TODO: remove TODO lines
        content = lines.filter(line => line.trim()).map(line => {
            let newLine = line.trimStart();
            let delta = line.length - newLine.length;
            if (!minSpace) {
                minSpace = delta;
            }
            minSpace = Math.min(minSpace, delta);
            return line.trimEnd();
        }).map(line => line.slice(minSpace)).join("\n");

        this.shadowRoot.innerHTML = style(this.theme) + template(lang, content, noPre);

        const code = this.shadowRoot.querySelector("code");
        hljs.highlightElement(code);
    }

    connectedCallback() {
        this._contentObserver.observe(this, {
            childList: true,
            characterData: true,
            subtree: true
        });
        // Eltern beobachten
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
        this.initialized = false;

        this._contentObserver = new MutationObserver(() => this.render());
        this._themeObserver = new MutationObserver(() => this.emitTheme());
        this.shadowRoot.addEventListener("theme-change", () => {
            console.log("event");
            this.render()
        });
    }
}
customElements.define("code-block", CodeBlock);