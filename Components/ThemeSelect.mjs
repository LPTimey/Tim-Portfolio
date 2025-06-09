"use strict";
import { css, html } from "../script.mjs";

const template = html`
<select id="ThemeSelect" aria-label="Wählt Farbthema der Seite aus">
    <option id="ThemeLight" value="ThemeLight">Licht-Thema</option>
    <!-- <option id="ThemeSystem" value="ThemeSystem" selected>System-Thema</option> -->
    <option id="ThemeDark" value="ThemeDark">Dunkel-Thema</option>
</select>
`;

const style = css`
@import "setup.css";
`;

/**
 * @typedef {"ThemeLight"|"ThemeDark"} Theme
 */

export default class ThemeSelect extends HTMLElement {
    /**
     * @returns {Theme}
     */
    get theme() {
        let value = /** @type {Theme|undefined} */ (this.shadowRoot?.querySelector("select")?.value)
        return value ?? "ThemeLight";
    }
    /**
     * 
     * @param {Theme} theme 
     */
    set theme(theme) {
        console.log(theme);
        const select = /** @type {HTMLSelectElement} */(this.shadowRoot.querySelector("select"));
        let test = [...select.options].map(op => op.value);
        if (!test.includes(theme)) {
            return;
        }
        select.value = theme;
    }
    static getLocalStorageTheme() {
        return window.localStorage.getItem("theme");
    }
    /**
     * 
     * @param {Theme} theme 
     */
    static setLocalStorageTheme(theme) {
        window.localStorage.setItem("theme", theme);
    }
    /**
     * 
     * @param {Theme} theme 
     */
    static setDocTheme(theme) {
        switch (theme) {
            case "ThemeLight":
                document.body.className = "light"
                break;
            // case "ThemeSystem":
            //     document.body.className = "system"
            //     break;
            case "ThemeDark":
                document.body.className = "dark"
                break;
            default:
                break;
        }
    }
    connectedCallback() {
        this.shadowRoot.innerHTML = style + template;
        this.shadowRoot.querySelector("select")?.addEventListener("change", () => {
            let t = this.theme;
            ThemeSelect.setDocTheme(t);
            ThemeSelect.setLocalStorageTheme(t);
        });
        this.theme = /** @type {Theme?} */(ThemeSelect.getLocalStorageTheme()) ?? "ThemeLight";
        ThemeSelect.setDocTheme(this.theme);
    }
    constructor() {
        super()
        this.attachShadow({ mode: "open" })
        /** @type {ShadowRoot} */
        this.shadowRoot;
    }
}
customElements.define("theme-select", ThemeSelect);