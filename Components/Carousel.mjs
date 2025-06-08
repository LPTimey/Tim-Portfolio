"use strict";
import { css, html } from "../script.mjs";

const template = html`
<div class="ramp left"></div>
<div class="scroll">
    <slot></slot>
</div>
<div class="ramp right"></div>
`;

/**
 * 
 * @param {number?} [width] 
 * @returns 
 */
const style = (width) => css`
@import "setup.css";

:host{
    display: flex;
    position: relative;
    overflow: hidden;
}

.ramp{
    position: absolute;
    z-index: 1;
    height: auto;
    top: 0;
    bottom: 0;
    pointer-events: none;
    width: ${width ? width + `ch` : `10ch`};

    &.left{
        left: 0;
        background: linear-gradient(to right, var(--bg), rgba(0,0,0,0))
    }

    &.right{
        right: 0;
        background: linear-gradient(to left, var(--bg), rgba(0,0,0,0))
    }
}

.scroll{
    display: flex;
    gap: 1ch;
    overflow: auto;
    padding-block: 1rem;
    scroll-snap-type: x mandatory;
    padding-inline: ${width ? width + `ch` : `10ch`};
}
::slotted(*){
    scroll-snap-align: center;
}
::slotted(img) {
    user-drag: none;
    -webkit-user-drag: none;
    pointer-events: auto;
}
`;

export default class Carousel extends HTMLElement {
    static get observedAttributes() {
        return [];
    }
    /**
         * 
         * @param {string} name name of attribute
         * @param {*} oldValue old value of attribute
         * @param {*} newValue new value of attribute
         */
    attributeChangedCallback(name, oldValue, newValue) {
        if (!this.initialized) {
            return;
        }
        switch (name) {
            case "X": {
                let hero = this.shadowRoot?.querySelector("Y");
                hero?.setAttribute("X", newValue);
                break;
            }
            default: {
                break;
            }
        }
        return;
    }
    connectedCallback() {
        this.shadowRoot.innerHTML = style(2) + template;

        const scroll = /** @type {HTMLDivElement} */(this.shadowRoot.querySelector(".scroll"));

        let isDown = false;
        /** @type {number} */
        let startX;
        /** @type {number} */
        let scrollLeft;

        scroll.addEventListener("mousedown", (e) => {
            isDown = true;
            scroll.classList.add("dragging");
            startX = e.pageX - scroll.offsetLeft;
            scrollLeft = scroll.scrollLeft;
        });

        scroll.addEventListener("mouseleave", () => {
            isDown = false;
            scroll.classList.remove("dragging");
        });

        scroll.addEventListener("mouseup", () => {
            isDown = false;
            scroll.classList.remove("dragging");
        });

        scroll.addEventListener("mousemove", (e) => {
            if (!isDown) return;
            e.preventDefault();
            const x = e.pageX - scroll.offsetLeft;
            const walk = (x - startX) * 1.5; // Geschwindigkeit skalieren
            scroll.scrollLeft = scrollLeft - walk;
        });

        this.shadowRoot.querySelectorAll("img").forEach(img => {
            img.setAttribute("draggable", "false");
        });
        const slot = /** @type {HTMLSlotElement} */ (this.shadowRoot.querySelector("slot"));
        slot.addEventListener("slotchange", () => {
            const nodes = slot.assignedElements({ flatten: true });
            nodes.forEach(el => {
                if (el.tagName === "IMG") {
                    el.setAttribute("draggable", "false");
                }
            });
        });

        this.initialized = true;
    }
    constructor() {
        super()
        this.attachShadow({ mode: "open" });
        /** @type {ShadowRoot} */
        this.shadowRoot;
        this.initialized = false;
    }
}
customElements.define("x-carousel", Carousel);