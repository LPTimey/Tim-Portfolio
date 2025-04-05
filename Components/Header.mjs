import { burger_icon, close_icon, css, html } from "../script.mjs"

const template = html`
<icon-button id="Theme">${burger_icon} ${close_icon}</icon-button>
<header>
    <nav>
        <ul>
            <li><a href="/#" class="underline">Home</a></li>
            <li><a href="/about.html" class="underline"></a></li>
            <li></li>
            
            <li></li>
            <li><a href="" class="underline"></a></li>
        </ul>
    </nav>
</header>
`;

const style = css`
@import url("setup.css");
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
        this.attachShadow({ mode: "open" })
        this.shadowRoot.innerHTML = style + template;
    }
    constructor() {
        super()
    }
}
customElements.define("site-header", SiteHeader);