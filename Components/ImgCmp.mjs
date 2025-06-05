"use strict";
import { css, html, double_arrow } from "../script.mjs";

/**
 * 
 * @param {string} src1 
 * @param {string} alt1 
 * @param {string} src2 
 * @param {string} alt2 
 * @returns 
 */
const template = function (src1, alt1, src2, alt2) {
    return html`
<div>
    <img src="${src1}" alt="${alt1}">
</div>
<div class="img-comp-overlay">
    <img src="${src2}" alt="${alt2}">
</div>
<div>

    <div class="arrow left">${double_arrow}</div>
    <input type="range" name="" id="" min="0" max="1000" value="500" class="slider" aria-label="Slider der Sichtbarkeit vom vergleichsbild steuert">

    <div class="arrow right">${double_arrow}</div>
</div>
`;
}

// FIXME: Fix width when no-clip
/**
 * 
 * @param {boolean} noClip 
 * @returns 
 */
const style = (noClip) => css`
@import "setup.css";

*{
    transition: width 0s;
}

:host{
    position: relative;
    display:  grid;
    width: 100%;
    >*{
        grid-column: 1 / -1;
        grid-row: 1 / -1;
    }
    overflow:hidden;
}

img{
    display: block;
    width: 100%;
    height:100%;
    object-fit: ${noClip ? "contain" : "cover"};
    pointer-events: none;
}

div{
    overflow: hidden;
}

@keyframes pulse{
    from {
        opacity: 0.25;
    }
    to {
        opacity: 0.75;
    }
}
.arrow{
    position: absolute;
    z-index: 3;
    top:50%;
    right:50%;
    translate: 50% -50%;
    fill: var(--accent);
    aspect-ratio: 1 / 1;
    width: 5ch;
    display: grid;
    place-items: center;
    animation: pulse 1s infinite alternate ease-in-out;
    pointer-events: none;

    svg{
        fill: var(--fg);
        stroke: var(--bg);
        stroke-width: 1.25em;
        width: 100%;
        height: 100%;
    }

    &.left{
        -webkit-transform: scale(-1, -1);
        -moz-transform: scale(-1, -1);
        -o-transform: scale(-1, -1);
        transform: scale(-1, -1);
    }
}



.img-comp-overlay {
    position: relative;
    overflow: hidden;
    img{
        position: absolute;
        left: 0;
        height: 100%;
        z-index: 1;
        /* object-position: left; */
    }
}

.slider {
    position: relative;
    -webkit-appearance: none;
    appearance: none;
    background: transparent;
    cursor: pointer;
    width: 100%;
    height: 100%;
    z-index: 2;
}
`;

//TODO: Remove, make delay bigger or make the arrows almost gone after fist interaction
export default class ImgCmp extends HTMLElement {
    static get observedAttributes() {
        return ["src1", "alt1", "src2", "alt2", "no-clip" /*, "angle" */];
    }
    /**
         * 
         * @param {string} name name of attribute
         * @param {*} oldValue old value of attribute
         * @param {*} newValue new value of attribute
         */
    // @ts-ignore
    attributeChangedCallback(name, oldValue, newValue) {
        return;
    }
    initComparisons() {
        const input = /** @type {HTMLInputElement} */(this.shadowRoot?.querySelector("input"));
        /**
         * 
         * @param {HTMLInputElement} target 
         */
        const clip = (target) => {
            //TODO: add a way to make cut-of angled like for example / or \ instead of just |
            // @ts-ignore
            this.shadowRoot.querySelector(".img-comp-overlay > img")?.style.clipPath = `inset(0 ${100 - Number(target.value) / 10}% 0 0)`
        }
        clip(input);
        /**
         * 
         * @param {HTMLInputElement} target 
         */
        const affordance = (target) => {
            // @ts-ignore
            this.shadowRoot.querySelectorAll(".arrow").forEach((el) => {
                let delta = 2;
                if (el.classList.contains("right")) {
                    delta = (-delta);
                }
                // @ts-ignore
                el.style.right = `calc(${100 - target.value / 10}% + ${delta}em)`;
            })
        }
        affordance(input);

        // @ts-ignore
        this.shadowRoot.querySelector("input").addEventListener("input", (ev) => {
            // @ts-ignore
            clip(ev.target);
            // @ts-ignore
            affordance(ev.target)
        })
    }
    connectedCallback() {
        // @ts-ignore
        this.shadowRoot.innerHTML =
            style(this.hasAttribute("no-clip"))
            // @ts-ignore
            + template(this.getAttribute("src1"), this.getAttribute("alt1"),
                this.getAttribute("src2"), this.getAttribute("alt2"));

        this.initComparisons();
    }
    constructor() {
        super()
        this.attachShadow({ mode: "open" })
    }
}
customElements.define("img-cmp", ImgCmp);