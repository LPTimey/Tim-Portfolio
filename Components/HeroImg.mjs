"use strict";
import { css, html } from "../script.mjs";
import ScrollImage from "./ScrollImg.mjs";

/**
 * 
 * @param {string?} [src] - Bildquelle (URL oder Pfad).
 * @param {string?} [alt] - Alternativtext für das Bild.
 * @param {(string | number)?} [time] - Zeitwert, der ggf. als Attribut übergeben wird.
 * @param {(string | number)?} [cols] - Anzahl der Spalten für das Grid-Layout.
 * @param {(string | number)?} [rows] - Anzahl der Zeilen für das Grid-Layout.
 * @param {string?} [bg] - Hintergrund CSS des Containers (Standard: "white").
 * @param {string?} [imgStyle] - Inline-Stil für das `<img>`-Tag oder `img-style`-Attribut bei `<scroll-img>`.
 * @returns 
 */
const template = (src, alt, time, cols, rows, bg, imgStyle) => html`
${(src || alt) && (cols || rows) ?
        html`<scroll-img src="${src}" alt="${alt ?? ""}" id="HeroImage" ${!!time ? html`time="${time}"` : ""} cols="${cols ?? "1"}" rows="${rows ?? "1"}" bg="${bg ?? "white"}" img-style="${imgStyle ?? ""}"></scroll-img>`
        : src || alt ? html`<img id="HeroImage" src="${src ?? ""}" alt="${alt ?? ""}" style="${imgStyle ?? ""}" width="${window.innerWidth}" height="${window.innerHeight}" /> ` : html``}
<slot></slot>
`;


/**
 * 
 * @param {string?} bg - Hintergrund CSS des Containers.
 * @returns 
 */
const style = (bg) => css`
@import "setup.css";
:host{
    --height: 70vh;
    width: 100%;
    height: var(--height);
    display: grid;
    overflow: hidden;
    align-items: end;
    ${bg ? `background: ${bg};background-size: cover;` : ""}
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
        return ["src", "alt", "bg", "time", "cols", "rows", "img-style"];
    }
    /**
     * 
     * @param {string} name name of attribute
     * @param {*} oldValue old value of attribute
     * @param {*} newValue new value of attribute
     */
    attributeChangedCallback(name, oldValue, newValue) {
        // this.shadowRoot.innerHTML = style(this.getAttribute("bg")) + template(this.getAttribute("src"), this.getAttribute("alt"), this.getAttribute("time"), this.getAttribute("cols"), this.getAttribute("rows"), this.getAttribute("bg"), this.getAttribute("img-style"));
        // console.log(name, oldValue, newValue);
        switch (name) {
            case "src": {
                let hero = this.shadowRoot?.querySelector("#HeroImage");
                hero?.setAttribute("src", newValue);
                break;
            }
            case "alt": {
                let hero = this.shadowRoot?.querySelector("#HeroImage");
                hero?.setAttribute("alt", newValue);
                break;
            }
            case "bg": {
                let hero = this.shadowRoot?.querySelector("scroll-img#HeroImage");
                hero?.setAttribute("bg", newValue);
                break;
            }
            case "time": {
                let hero = this.shadowRoot?.querySelector("scroll-img#HeroImage");
                hero?.setAttribute("time", newValue);
                break;
            }
            case "cols": {
                let hero = this.shadowRoot?.querySelector("scroll-img#HeroImage");
                hero?.setAttribute("cols", newValue);
                break;
            }
            case "rows": {
                let hero = this.shadowRoot?.querySelector("scroll-img#HeroImage");
                hero?.setAttribute("rows", newValue);
                break;
            }
            case "img-style": {
                let hero = this.shadowRoot?.querySelector("scroll-img#HeroImage");
                hero?.setAttribute("img-style", newValue);
                if (!hero) {
                    let hero = this.shadowRoot?.querySelector("img#HeroImage");
                    hero?.setAttribute("style", newValue);
                }
                break;
            }
            default: {
                break;
            }
        }
        return;
    }
    connectedCallback() {
        // @ts-ignore
        this.shadowRoot.innerHTML = style(this.getAttribute("bg")) + template(this.getAttribute("src"), this.getAttribute("alt"), this.getAttribute("time"), this.getAttribute("cols"), this.getAttribute("rows"), this.getAttribute("bg"), this.getAttribute("img-style"));
    }
    constructor() {
        super()
        this.attachShadow({ mode: "open" })
    }
}
customElements.define("hero-img", HeroImg);