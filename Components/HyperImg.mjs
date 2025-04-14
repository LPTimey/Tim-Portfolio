import { css, html } from "../script.mjs";

/* TODO: Impl This: A way to have multiple images wich link to each other through 
 * hidden interaction areas, emulating a Prototype
 */
const template = html`
`;

const style = css`
@import "setup.css";
`;

export default class HyperImg extends HTMLElement {
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
        // this.shadowRoot.innerHTML = style + template;
        // console.log(name, oldValue, newValue);
        switch (name) {
            case "X": {
                let hero = this.shadowRoot.querySelector("Y");
                hero?.setAttribute("X", newValue);
                break;
            }
            default: {
                break;
            }
        }
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
customElements.define("hyper-img", HyperImg);