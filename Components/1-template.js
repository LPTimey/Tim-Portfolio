"use strict";
import { css, html } from "../script.mjs";

const template = html`
`;

const style = css`
@import "setup.css";
`;

export default class x extends HTMLElement {
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
customElements.define("x-x", x);