import { css, html } from "../script.mjs";
import ScrollImage from "./ScrollImg.mjs";

const template = (src, alt, cols, rows) => html`
<slot></slot>
<scroll-img src="./assets/Title-img.png" alt="Scrolling Image" id="HeroImage" time="35s" cols="2" rows="3" bg="white"></scroll-img>
`;

const style = (time) => css`
@import "setup.css";
:host{
    width: 100%;
    height: 70vh;
    display: grid;
    overflow: hidden;
    align-items: end;
    * {
        grid-column: 1 / -1;
        grid-row: 1 / -1;
    }
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
        this.shadowRoot.innerHTML = style(this.getAttribute("time")) + template(this.getAttribute("src"), this.getAttribute("alt"), this.getAttribute("cols"), this.getAttribute("rows"));
    }
    constructor() {
        super()
        this.attachShadow({ mode: "open" })
    }
}
customElements.define("hero-img", HeroImg);