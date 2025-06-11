"use strict";
import { css, html } from "../script.mjs";

/**
 * 
 * @param {string} src 
 * @param {string} alt 
 * @param {number} cols 
 * @param {number} rows 
 * @param {(string | number)?} [imgWidth]
 * @param {(string | number)?} [imgHeight]
 * @returns 
 */
const template = (src, alt, cols, rows, imgWidth, imgHeight) => html`
${html`<picture><img src="${src}" alt="${alt}" class="scrolling-image" width="${imgWidth || window.innerWidth}" height="${imgHeight || window.innerHeight}" /></picture>`.repeat(cols * rows)}
`;

/**
 * 
 * @param {string} time 
 * @param {number} cols 
 * @param {string?} bg 
 * @returns 
 */
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
        return ["src", "alt", "time", "cols", "rows", "bg", "img-width", "img-height"];
    }
    /**
         * 
         * @param {string} name name of attribute
         * @param {*} oldValue old value of attribute
         * @param {*} newValue new value of attribute
         */
    attributeChangedCallback(name, oldValue, newValue) {
        if (!this._initialized) {
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
                let styleEl = this.shadowRoot?.querySelector("style") ?? { outerHTML: "" };
                styleEl.outerHTML = style(newValue, Number(this.getAttribute("cols")), this.getAttribute("bg"));
                break;
            }
            case "cols": {
                let styleEl = this.shadowRoot?.querySelector("style") ?? { outerHTML: "" };
                styleEl.outerHTML = style(this.getAttribute("time") ?? "0s", newValue, this.getAttribute("bg"));
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
                let styleEl = this.shadowRoot.querySelector("style") ?? { outerHTML: "" };
                styleEl.outerHTML = style(this.getAttribute("time") ?? "0s", Number(this.getAttribute("cols")), newValue);
                break;
            }
            default: {
                break;
            }
        }
        return;
    }
    connectedCallback() {
        this.shadowRoot.innerHTML =
            style(
                this.getAttribute("time") ?? "0s",
                Number(this.getAttribute("cols")),
                this.getAttribute("bg")
            )
            + template(
                this.getAttribute("src") ?? "",
                this.getAttribute("alt") ?? "",
                Number(this.getAttribute("cols")),
                Number(this.getAttribute("rows")),
                this.getAttribute("imgWidth"),
                this.getAttribute("imgHeight")
            );

        this._initialized = true;
    }
    constructor() {
        super()
        this._initialized = false;
        this.attachShadow({ mode: "open" })
        /** @type {ShadowRoot} */
        this.shadowRoot;
    }
}
customElements.define("scroll-img", ScrollImage);