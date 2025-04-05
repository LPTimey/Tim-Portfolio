import { css, html } from "../script.mjs";

const template = (src, alt, cols, rows) => html`
${html`<picture><img src="${src}" alt="${alt}" class="scrolling-image"/></picture>`.repeat(cols * rows)}
`;

const style = (time, cols, bg) => css`
@import "setup.css";
:host{
    display:  grid;
    grid-template-columns: repeat(${cols},1fr);
    position: relative;
    width:    calc(100% * ${cols});
    height:   100%;
    overflow: hidden;
    z-index:  -1;
    left:     -100%;
    background: ${bg};

    animation:  scrollGrid ${time} linear infinite;
}
.scrolling-image {
    display:    block;
    object-fit: cover;
    margin:   0;
    padding:  0;
    width:    100%;
    height:   auto;
    position: relative;
    top: 0;
    animation: scrollImage ${time} linear infinite;
    ${bg ? `background: ${bg}` : ""};
}

@keyframes scrollImage {
    0% {
        translate: 0 0;
    }

    100% {
        translate: 0 -100%;
        /* Zweites Bild kommt an die Stelle des ersten Bildes */
    }
}

@keyframes scrollGrid {
    0% {
        translate: 0 0;
    }

    100% {
        translate: calc(100% / ${cols}) 0;
    }
}
`;

export default class ScrollImage extends HTMLElement {
    static get observedAttributes() {
        return ["src", "alt", "time", "cols", "rows", "bg"];
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
        this.shadowRoot.innerHTML = style(this.getAttribute("time"), this.getAttribute("cols"), this.getAttribute("bg"))
            + template(this.getAttribute("src"), this.getAttribute("alt"), this.getAttribute("cols"), this.getAttribute("rows"));
    }
    constructor() {
        super()
        this.attachShadow({ mode: "open" })
    }
}
customElements.define("scroll-img", ScrollImage);