import { css, html } from "../script.mjs";
const template = (id) => html`
<div hidden>
    <input type="checkbox" name="${id}" id="${id}" />
</div>
<label for="${id}"><slot></slot></label>
<label for="${id}"><slot></slot></label>
`;

const style = css`
@import url("setup.css");
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
        return;
    }
    connectedCallback() {
        this.attachShadow({ mode: "open" })
        let temp = template(this.id + "box");
        debugger
        this.shadowRoot.innerHTML = style + temp;

    }
    constructor() {
        super()
    }
}
customElements.define("icon-button", IconButton);