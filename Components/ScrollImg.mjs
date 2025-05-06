"use strict";
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
    width:    ${100.0 * cols}%;
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
    from {
        translate: 0 0;
    }

    to {
        translate: 0 -100%;
        /* Zweites Bild kommt an die Stelle des ersten Bildes */
    }
}

@keyframes scrollGrid {
    from {
        translate: 0 0;
    }

    to {
        translate:  ${100.0 / cols}% 0;
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
        if (!this._initialized){
            return;
        }
        console.log(name, oldValue, newValue);
        switch (name) {
            case "src": {
                let heroes = this.shadowRoot.querySelectorAll(".scrolling-image");
                heroes.forEach((el) => el.setAttribute("src", newValue));
                break;
            }
            case "alt": {
                let heroes = this.shadowRoot.querySelectorAll("img");
                heroes.forEach((el) => el.setAttribute("alt", newValue));
                break;
            }
            case "time": {
                let styleEl = this.shadowRoot.querySelector("style") ?? {};
                styleEl.outerHTML = style(newValue, this.getAttribute("cols"), this.getAttribute("bg"));
                break;
            }
            case "cols": {
                let styleEl = this.shadowRoot.querySelector("style") ?? {};
                styleEl.outerHTML = style(this.getAttribute("time"), newValue, this.getAttribute("bg"));
                let imgs = [... this.shadowRoot.querySelectorAll(".scrolling-image")];
                let delta = imgs.length - Number(newValue) * Number(this.getAttribute("rows"));
                if (delta < 0) {
                    for (let i = 0; i < delta; i++) {
                        let img = document.createElement("picture");
                        img.innerHTML = html`
                        <img src="${this.getAttribute("src") ?? ""}" alt="${this.getAttribute("alt") ?? ""}" class="scrolling-image"/>
                        `
                        document.appendChild(img);
                    }
                }
                break;
            }
            case "rows": {
                let imgs = [... this.shadowRoot.querySelectorAll(".scrolling-image")];
                let delta = imgs.length - Number(newValue) * Number(this.getAttribute("cols"));
                if (delta < 0) {
                    for (let i = 0; i < delta; i++) {
                        let img = document.createElement("picture");
                        img.innerHTML = html`
                        <img src="${this.getAttribute("src") ?? ""}" alt="${this.getAttribute("alt") ?? ""}" class="scrolling-image"/>
                        `
                        document.appendChild(img);
                    }
                }
                break;
            }
            case "bg": {
                let styleEl = this.shadowRoot.querySelector("style") ?? {};
                styleEl.outerHTML = style(this.getAttribute("time"), this.getAttribute("cols"), newValue);
                break;
            }
            default: {
                break;
            }
        }
        return;
    }
    connectedCallback() {
        this.shadowRoot.innerHTML = style(this.getAttribute("time"), this.getAttribute("cols"), this.getAttribute("bg"))
            + template(this.getAttribute("src"), this.getAttribute("alt"), this.getAttribute("cols"), this.getAttribute("rows"));

        this._initialized = true;
    }
    constructor() {
        super()
        this._initialized = false;
        this.attachShadow({ mode: "open" })
    }
}
customElements.define("scroll-img", ScrollImage);