import { css, html, changeEvent } from "../script.mjs";
const template = (id) => html`
<div hidden>
    <input type="checkbox" name="${id}" id="${id}" />
</div>
<label for="${id}"><slot name="activated"></slot></label>
<label for="${id}"><slot name="deactivated"></slot></label>
`;

const style = css`
@import "setup.css";
:host{
    display: grid;
    place-items: center;
     * {
        width: 100%;
        height: 100%;
        grid-column: 1 / -1;
        grid-row: 1 / -1;
    }
}
::slotted(*){
    width: 100%;
    height: 100%;
    object-fit: cover;
}
:host([checked]){
    [name="deactivated"]{
        visibility: hidden;
    }
    [name="activated"]{
        visibility: visible;
    }
}
:host(:not([checked])){
    [name="deactivated"]{
        visibility: visible
    }
    [name="activated"]{
        visibility: hidden
    }
}
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
        const slots = this.shadowRoot.querySelectorAll('slot');
        if (this.shadowRoot.querySelector("input").checked) {
            this.setAttribute("checked", "");
        } else {
            this.removeAttribute("checked");
        }

        this.shadowRoot.querySelector("input").addEventListener("change", (ev) => {
            // this.setAttribute("checked", ev.target.checked);
            // this.checked = !!ev.target.checked
            if (ev.target.checked) {
                this.setAttribute("checked", "");
            } else {
                this.removeAttribute("checked");
            }
            this.dispatchEvent(changeEvent)
        });

    }
    constructor() {
        super()
        this.attachShadow({ mode: "open" })
    }
}
customElements.define("icon-button", IconButton);