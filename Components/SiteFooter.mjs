import { css, html } from "../script.mjs";
import ToTop from "./ToTop.mjs"

const template = html`
<to-top></to-top>
<footer>
    Created with HTML, CSS & JS by Tim Ruland © 2025
</footer>
`;

const style = css`
@import "setup.css";
footer {
    padding-block: 3em;
    padding-inline: var(--outer-pad-inline);
    background-color: color-mix(in srgb, var(--bg), var(--fg) 25%);
    margin: auto;
}

`;

export default class SiteFooter extends HTMLElement {
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
        super();
        this.attachShadow({ mode: "open" });
    }
}
customElements.define("site-footer", SiteFooter);