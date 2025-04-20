"use strict";
import { css, html } from "../script.mjs";

const template = (src, alt, href) => html`
<picture><img src="${src}" alt="${alt}"/></picture>
<div id="Text">
    <slot></slot>
</div>
<a href="${href}" class="button">Details</a>
`;

const style = css`
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
        return ["src", "alt", "href"];
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
        this.shadowRoot.innerHTML = style + template(this.getAttribute("src"), this.getAttribute("alt"), this.getAttribute("href"));
    }
    constructor() {
        super()
        this.attachShadow({ mode: "open" })
    }
}
customElements.define("project-card", ProjectCard);