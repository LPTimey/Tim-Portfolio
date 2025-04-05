import { css, html, changeEvent } from "../script.mjs";
const template = (id) => html`
<div hidden>
    <input type="checkbox" name="${id}" id="${id}" />
</div>
<label for="${id}"><slot></slot></label>
<label for="${id}"><slot></slot></label>
`;

const style = css`
@import "setup.css";
    `;

export default class IconButton extends HTMLElement {
    get value() {
        return this.shadowRoot.querySelector("input").value;
    }
    static get observedAttributes() {
        return ["id"];
    }
    /**
         * 
         * @param {string} name name of attribute
         * @param {*} oldValue old value of attribute
         * @param {*} newValue new value of attribute
         */
    attributeChangedCallback(name, oldValue, newValue) {
        switch (name) {
            case "id":
                this.shadowRoot.innerHTML = style + template(newValue + "box");
                break;
        }
        return;
    }

    connectedCallback() {
        this.shadowRoot.innerHTML = style + template(this.id + "box");
        this.shadowRoot.querySelector("input").addEventListener("change", (ev) => { this.dispatchEvent(changeEvent) });

    }
    constructor() {
        super()
        this.attachShadow({ mode: "open" })
    }
}
customElements.define("icon-button", IconButton);