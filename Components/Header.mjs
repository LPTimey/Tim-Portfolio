import { burger_icon, close_icon, css, gh_logo, html } from "../script.mjs"
import IconButton from "./IconButton.mjs";
import ThemeSelect from "./ThemeSelect.mjs";

const template = html`
<header>
    <nav class="content">
        <ul>
            <li><a href="/#" class="underline link">Home</a></li>
            <div class="separator"></div>
            <li><a href="/about.html" class="underline link">Über mich</a></li>
            <div class="separator"></div>
            <li>Projects</li>
            <div class="grow"></div>
            <li><theme-select></theme-select></li>
            <div class="separator"></div>
            <li class="flex"><a href="https://github.com/LPTimey/Tim-Portfolio" class="link flex center"> <span class="underline mobile center">Source</span> ${gh_logo}</a></li>
        </ul>
    </nav>
</header>
<icon-button id="Theme" class="mobile"><div slot="activated" class="full children">${close_icon}</div> <div slot="deactivated" class="full children">${burger_icon}</div></icon-button>
`;

const style = css`
@import "setup.css";

header{
    position: fixed;
    display: block;
    width: 100%;
    overflow: hidden;
    top: 0;
    left: 0;
    right: 0;
    height: fit-content;
    z-index: 1000000000;
    background-color: rgba(from var(--bg) r g b / var(--transparency));
    -webkit-backdrop-filter: blur(var(--blur-r));
    backdrop-filter: blur(var(--blur-r));
}
nav,ul{
    height: 100%;
    width: 100%;
}
ul{
    padding-block:2rem;
    display: flex;
    gap: 1ch;
    justify-content: center;
    align-items: center
}
svg {
    fill: var(--fg);
    /* stroke: var(--fg); */
    &:hover{
        fill: var(--accent);
    }
}

@media(width < 740px) {
    header{
        height: 100%;
    }
    ul {
        flex-direction: column;
        gap: 3rem;

        .separator,.grow{
            display:none;
        }
    }
    icon-button{
        position: fixed;
        padding: 0.5rem;
        right: 2rem;
        top: 2rem;
        aspect-ratio: 1;
        width: 3.5rem;
        z-index: 1000000000;

        &:not([checked]){
            background-color: rgba(from var(--bg) r g b / var(--transparency));
            -webkit-backdrop-filter: blur(var(--blur-r));
            backdrop-filter: blur(var(--blur-r));
            border-radius: var(--border-r);
        }
    }
    header:not(:has(+ icon-button[checked])) {
        transform: translateX(100%);
    }
}
`;

export default class SiteHeader extends HTMLElement {
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
        return;
    }
    connectedCallback() {
        this.shadowRoot.innerHTML = style + template;
    }
    constructor() {
        super()
        this.attachShadow({ mode: "open" })
    }
}
customElements.define("site-header", SiteHeader);
