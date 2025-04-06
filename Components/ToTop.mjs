import { css, html, to_top_icon } from "../script.mjs";

const template = html`
<a href="#">${to_top_icon}</a>
`;

const style = css`
@import "setup.css";
a {
    position: fixed;
    bottom: 3rem;
    right: calc(var(--outer-pad-inline) / 2);

    padding: 1.5ch;

    border-radius: 50%;
    border: 1px solid var(--fg);

    background-color: rgba(from var(--bg) r g b / var(--transparency));
    -webkit-backdrop-filter: blur(var(--blur-r));
    backdrop-filter: blur(var(--blur-r));
}


`;

export default class ToTop extends HTMLElement {
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
customElements.define("to-top", ToTop);