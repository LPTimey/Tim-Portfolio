import { css, html } from "../script.mjs";

const template = (src, alt, phonePrefix) => html`
${src || alt ? html`<img class="screen" src="${src}" alt="${alt}">` : html`<slot></slot>`}
<img class="phone" src="${phonePrefix ?? "./"}assets/iPhone Template [Konvertiert] noBG.png" alt="">
`;

const style = (bg) => css`
@import "setup.css";
:host{
    --iphone16PM-aspect: 1420 / 2868;
    aspect-ratio: var(--iphone16PM-aspect);
    max-height: 100%;
    position: relative;
    display:  block;
    z-index:  0;
    width: 100%;
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
.screen, ::slotted(*) {
    border-radius: 10% / 5% ;
    height: 97.5%;
    aspect-ratio: var(--iphone16PM-aspect);
    ${bg ? `background: ${bg}` : ""};
}
.phone{
    height: 100%;
    aspect-ratio: var(--iphone16PM-aspect);
    pointer-events: none;
}
`;

export class PhoneImage extends HTMLElement {
    static get observedAttributes() {
        return ['src', 'alt', "bg", "phonePrefix"];
    }

    connectedCallback() {
        this.shadowRoot.innerHTML = style(this.getAttribute("bg")) + template(this.getAttribute("src"), this.getAttribute("alt"), this.getAttribute("phonePrefix"))
    }
    constructor() {
        super();
        this.attachShadow({ mode: "open" });
    }
}
customElements.define("phone-img", PhoneImage)