"use strict";
import { css, html } from "../script.mjs";

/**
 * 
 * @param {string?} src 
 * @param {string?} alt 
 * @param {string?} phonePrefix 
 * @returns 
 */
const template = (src, alt, phonePrefix) => html`
${src || alt ? html`<img loading="lazy" class="screen" src="${src}" alt="${alt}">` : html`<slot></slot>`}
<img loading="lazy" class="phone" src="${phonePrefix ?? " ./"}assets/iPhone Template [Konvertiert] noBG.png" alt="">
`;

/**
 * 
 * @param {string?} [bg] 
 * @returns 
 */
const style = (bg) => css`
@import "setup.css";
:host{
    --i-iphone16PM-h: var(--iphone16PM-h, 3000);
    --i-iphone16PM-w: var(--iphone16PM-w, 1420);
    --iphone16PM-aspect: var(--i-iphone16PM-w) / var(--i-iphone16PM-h);
    --iphone16PM-aspect-outer: calc((var(--i-iphone16PM-w) + 75) / var(--i-iphone16PM-h));
    aspect-ratio: var(--iphone16PM-aspect-outer);
    max-height: 100%;
    position: relative;
    display:  block;
    z-index:  0;
}
img,::slotted(*){
    object-fit: fill;
    max-height: 100%;
    max-width: 100%;
    position: absolute;
    display: block;
    height: auto;
    width: auto;
    left: 50%;
    top: 50%;
    transform: translate(-50%, -50%);
}
.phone{
    height: 100%;
    aspect-ratio: var(--iphone16PM-aspect-outer);
    pointer-events: none;
}
.screen, ::slotted(*) {
    border-radius: 10% / 5% ;
    height: 97%;
    aspect-ratio: var(--iphone16PM-aspect);
    ${bg ? `background: ${bg};` : `;`};
}
`;

export class PhoneImage extends HTMLElement {
    static get observedAttributes() {
        return ['src', 'alt', "bg", "phonePrefix"];
    }

    connectedCallback() {
        this.shadowRoot.innerHTML = style(this.getAttribute("bg")) + template(this.getAttribute("src"), this.getAttribute("alt"), this.getAttribute("phonePrefix"));

    }
    constructor() {
        super();
        this.attachShadow({ mode: "open" });
        /** @type {ShadowRoot} */
        this.shadowRoot;
    }
}
customElements.define("phone-img", PhoneImage)