import { css, html } from "../script.mjs";

const template = html`
<select id="ThemeSelect">
    <option id="ThemeLight" value="ThemeLight">Licht-Thema</option>
    <option id="ThemeSystem" value="ThemeSystem" selected>System-Thema</option>
    <option id="ThemeDark" value="ThemeDark">Dunkel-Thema</option>
</select>
`;

const style = css`
@import "setup.css";
`;

export default class ThemeSelect extends HTMLElement {
    get theme() {
        let value = this.shadowRoot.querySelector("select").value
        return value;
    }
    set theme(theme) {
        console.log(theme);
        let test = [...this.shadowRoot.querySelector("select").options].map(op => op.value);
        if (!test.includes(theme)){
            return;
        }
        this.shadowRoot.querySelector("select").value = theme;
    }
    static getLocalStorageTheme() {
        return window.localStorage.getItem("theme");
    }
    static setLocalStorageTheme(theme) {
        window.localStorage.setItem("theme", theme);
    }
    static setDocTheme(theme) {
        switch (theme) {
            case "ThemeLight":
                document.body.className = "light"
                break;
            case "ThemeSystem":
                document.body.className = "system"
                break;
            case "ThemeDark":
                document.body.className = "dark"
                break;
            default:
                break;
        }
    }
    connectedCallback() {
        this.shadowRoot.innerHTML = style + template;
        this.shadowRoot.querySelector("select").addEventListener("change", () => {
            let t = this.theme;
            ThemeSelect.setDocTheme(t);
            ThemeSelect.setLocalStorageTheme(t);
        });
        this.theme = ThemeSelect.getLocalStorageTheme();
        ThemeSelect.setDocTheme(this.theme);
    }
    constructor() {
        super()
        this.attachShadow({ mode: "open" })
    }
}
customElements.define("theme-select", ThemeSelect);