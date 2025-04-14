import { css, html } from "../script.mjs";
import ScrollImage from "./ScrollImg.mjs";

const template = (src, alt, time, cols, rows, bg) => html`
${time && cols && rows ?
        html`<scroll-img src="${src}" alt="${alt ?? ""}" id="HeroImage" time="${time}" cols="${cols}" rows="${rows}" bg="${bg ?? "white"}"></scroll-img>`
        : src || alt ? html`<img src="${src}" alt="${alt ?? ""}"/>` : html``}
<slot></slot>
`;

const style = (bg)=>css`
@import "setup.css";
:host{
    --height: 70vh;
    width: 100%;
    height: var(--height);
    display: grid;
    overflow: hidden;
    align-items: end;
    ${bg? `background: ${bg};`:""}
    * {
        grid-column: 1 / -1;
        grid-row: 1 / -1;
    }
}
img{
    height: var(--height);
    width: 100%;
}
::slotted(*) {
    grid-column: 1 / -1;
    grid-row: 1 / -1;
    max-height: var(--height);
}
`;

export default class HeroImg extends HTMLElement {
    static get observedAttributes() {
        return ["src", "alt", "bg", "time", "cols", "rows"];
    }
    /**
     * 
     * @param {string} name name of attribute
     * @param {*} oldValue old value of attribute
     * @param {*} newValue new value of attribute
     */
    attributeChangedCallback(name, oldValue, newValue) {
        this.shadowRoot.innerHTML = style(this.getAttribute("bg")) + template(this.getAttribute("src"), this.getAttribute("alt"), this.getAttribute("time"), this.getAttribute("cols"), this.getAttribute("rows"), this.getAttribute("bg"));
        return;
    }
    connectedCallback() {
        this.shadowRoot.innerHTML = style(this.getAttribute("bg")) + template(this.getAttribute("src"), this.getAttribute("alt"), this.getAttribute("time"), this.getAttribute("cols"), this.getAttribute("rows"), this.getAttribute("bg"));
    }
    constructor() {
        super()
        this.attachShadow({ mode: "open" })
    }
}
customElements.define("hero-img", HeroImg);