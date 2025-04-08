import { css, html } from "../script.mjs";
import ScrollImage from "./ScrollImg.mjs";

const template = (src, alt, time, cols, rows) => html`
<slot></slot>
${time && cols && rows ?
        html`<scroll-img src="${src}" alt="${alt}" id="HeroImage" time="${time}" cols="${cols}" rows="${rows}" bg="white"></scroll-img>`
    : html`<img src="${src}" alt="${alt}"/>`}
`;

const style = css`
@import "setup.css";
:host{
    --height: 70vh;
    width: 100%;
    height: var(--height);
    display: grid;
    overflow: hidden;
    align-items: end;
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
}
`;

export default class HeroImg extends HTMLElement {
    static get observedAttributes() {
        return ["src", "alt", "time", "cols", "rows"];
    }
    connectedCallback() {
        this.shadowRoot.innerHTML = style + template(this.getAttribute("src"), this.getAttribute("alt"), this.getAttribute("time"), this.getAttribute("cols"), this.getAttribute("rows"));
    }
    constructor() {
        super()
        this.attachShadow({ mode: "open" })
    }
}
customElements.define("hero-img", HeroImg);