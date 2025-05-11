"use strict";
import { css, html, lightDark } from "../script.mjs";

/**
 * 
 * @param {string} src 
 * @param {string} srcDark 
 * @param {string} alt 
 * @param {string} href 
 * @param {boolean} has_button 
 * @param {boolean} isDark 
 * @returns 
 */
const template = (src, srcDark, alt, href, has_button, isDark) => html`
<picture>
    <img id="DarkImg" src="${srcDark}" alt="${alt}" style="display:${isDark && srcDark ? 'block' : 'none'}" />
    <img src="${src}" alt="${alt}" style="display:${isDark && srcDark ? 'none' : 'block'}" />
</picture>
<div id="Text">
    <slot></slot>
</div>
${!has_button || !href ? "" : html`<a href="${href}" class="button">Details</a>`}
`;

const style = () => css`
@import "setup.css";
@property --translate-y{
    syntax: "<length>";
    inherits: false;
    initial-value: 0rem;
}
:host{
    --x:0.5rem;
    --y:0.5rem;
    --blur:0.5rem;
    position:relative;
    display: grid;
    grid-template-rows: subgrid;
    grid-row: span 4;
    gap: 1rem;
    background-color: color-mix(in srgb, var(--bg), gray 0%);
    border-radius: var(--border-r);
    border: 1px solid var(--fg);
    overflow: hidden;
    filter: drop-shadow(var(--x) var(--y) var(--blur) rgb(from var(--fg) r g b / 0.1));
}
:host(:hover){
    animation: 0.2s ease-in-out 0s 1 forwards hover;
}
:host(:not(:hover)){
    animation: 0.2s ease-in-out 0s 1 forwards unhover;
}
@keyframes hover{
    from{
        --translate-y: 0rem;
        transform: translateY(0px);
        filter: drop-shadow(var(--x) calc(var(--y) - var(--translate-y)) var(--blur) rgb(from var(--fg) r g b / 0.1)); 
    }
    to{
        --translate-y: -0.75rem;
        transform: translateY(var(--translate-y));
        filter: drop-shadow(var(--x) calc(var(--y) - var(--translate-y)) var(--blur) rgb(from var(--fg) r g b / 0.1)); 
    }
}
@keyframes unhover{
    to{
        --translate-y: 0rem;
        transform: translateY(0px);
        filter: drop-shadow(var(--x) calc(var(--y) - var(--translate-y)) var(--blur) rgb(from var(--fg) r g b / 0.1)); 
    }
    from{
        --translate-y: -0.75rem;
        transform: translateY(var(--translate-y));
        filter: drop-shadow(var(--x) calc(var(--y) - var(--translate-y)) var(--blur) rgb(from var(--fg) r g b / 0.1)); 
    }
}
img{
    width: 100%;
    height: 100%;
    object-fit: cover;
}
#Text{
    display:grid;
    grid-template-rows: subgrid;
    grid-row: span 2;
    gap: 0.25rem;
    padding-inline: 2ch;
}
.button{
    margin-inline: 2ch;
    margin-bottom: 1rem;
    margin-left: auto;
}
`;

export default class ProjectCard extends HTMLElement {
    static get observedAttributes() {
        return ["src", "src-dark", "alt", "href", "no-button"];
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
        this.render();
        // Eltern beobachten
        let current = this;
        while (current) {
            this._themeObserver.observe(current, { attributes: true, attributeFilter: ['class'], childList: true, subtree: false });
            current = current.parentElement;
        }
    }
    emitTheme() {
        this.shadowRoot.dispatchEvent(new Event('theme-change'));
    }
    render(isDark){
        this.shadowRoot.innerHTML = style() + template(
            this.getAttribute("src"),
            this.getAttribute("src-dark"),
            this.getAttribute("alt"),
            this.getAttribute("href"),
            !this.hasAttribute("no-button"),
            isDark ?? lightDark(this, false, true)
        );
    }
    constructor() {
        super()

        this.attachShadow({ mode: "open" });

        this._themeObserver = new MutationObserver(() => this.emitTheme());
        this.shadowRoot.addEventListener("theme-change", () => {
            console.log("event");
            this.render()
        });
    }
}
customElements.define("project-card", ProjectCard);