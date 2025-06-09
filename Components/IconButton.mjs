"use strict";
import { css, html, changeEvent } from "../script.mjs";

const style = css`
@import "setup.css";
:host{
    display: grid;
    place-items: center;
    cursor: pointer;
    border-radius: var(--border-r);
}
div{
    border-radius: var(--border-r);
    width: 100%;
    height: 100%;
    object-fit: cover;
    *{
        border-radius: var(--border-r);
        width: 100%;
        height: 100%;
        object-fit: cover;
    }
}
    `;

export default class IconButton extends HTMLElement {
    get value() {
        return this.shadowRoot.querySelector("input")?.value;
    }

    connectedCallback() {
        this.shadowRoot.innerHTML = style;
        this.shadowRoot.appendChild(this.content);

        const change_content = () => {
            this.checked = !this.checked;
            if (this.checked) {
                this.setAttribute("checked", "")
                this.content.innerHTML = this.checkedIcon.outerHTML;
            } else {
                this.removeAttribute("checked");
                this.content.innerHTML = this.uncheckedIcon.outerHTML;
            }
        }
        change_content()
        this.addEventListener("click", change_content);

    }
    constructor() {
        super()
        this.checked = true;
        this.checkedIcon = this.querySelector(`[slot="activated"]`) ?? document.createElement("div");
        this.uncheckedIcon = this.querySelector(`[slot="deactivated"]`) ?? document.createElement("div");
        this.content = document.createElement("div");
        this.attachShadow({ mode: "open" })
        /** @type {ShadowRoot} */
        this.shadowRoot;
    }
}
customElements.define("icon-button", IconButton);