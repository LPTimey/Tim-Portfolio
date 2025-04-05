import { burger_icon, close_icon, css, gh_logo, html } from "../script.mjs"

const template = html`
<icon-button id="Theme">${burger_icon} ${close_icon}</icon-button>
<header>
    <nav>
        <ul>
            <li><a href="/#" class="underline link">Home</a></li>
            <div class="separator"></div>
            <li><a href="/about.html" class="underline link">Über mich</a></li>
            <div class="separator"></div>
            <li>Projects</li>
            <div class="grow"></div>
            <li><slot></slot></li>
            <div class="separator"></div>
            <li><a href="" class="link"> <span class="underline mobile">Source</span> ${gh_logo}</a></li>
        </ul>
    </nav>
</header>
`;

const style = css`
@import "setup.css";
`;

export default class SiteHeader extends HTMLElement {
    static get observedAttributes() {
        return [];
    }
    /**
         * 
         * @param {string} name name of attribute
         * @param {*} oldValue old value of attribute
         * @param {*} newValue new value of attribute
         */
    attributeChangedCallback(name, oldValue, newValue) {
        return;
    }
    connectedCallback() {
        this.shadowRoot.innerHTML = style + template;
    }
    constructor() {
        super()
        this.attachShadow({ mode: "open" })
    }
}
customElements.define("my-site-header", SiteHeader);
customElements.define("site-header", class extends HTMLElement{
    constructor(){
        super();
        this.outerHTML =`<my-site-header><select id="ThemeSelect">
        <option id="ThemeLight" value="ThemeLight">Licht-Thema</option>
        <option id="ThemeSystem" value="ThemeSystem" selected>System-Thema</option>
        <option id="ThemeDark" value="ThemeDark">Dunkel-Thema</option>
    </select></my-site-header>`;
    }
});