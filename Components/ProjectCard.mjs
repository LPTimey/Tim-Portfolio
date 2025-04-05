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
:host{
    position:relative;
    display: grid;
    grid-template-rows: subgrid;
    grid-row: span 5;
    background-color: color-mix(in srgb, var(--bg), gray 10%);
    border-radius: var(--border-r);
    border: 1px solid var(--fg);
    overflow: hidden;
    filter: drop-shadow(0.25rem 0.25rem 0.25rem rgb(from var(--fg) r g b / 0.2));
}
:host(:hover){
    transform: translateY(-0.75rem);
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
    padding-inline: 2ch;
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