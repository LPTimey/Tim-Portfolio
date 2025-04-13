import { burger_icon, close_icon, css, gh_logo, html } from "../script.mjs"
import IconButton from "./IconButton.mjs";
import ThemeSelect from "./ThemeSelect.mjs";

const template = html`
<header>
    <nav class="content">
        <ul id="GeneralNav">
            <li><a href="./#" class="underline link" aria-label="Navigiert zur HomePage">Home</a></li>
            <li><div class="separator"></div></li>
            <li><a href="./about.html" class="underline link" aria-label="Navigiert zur AboutPage">Über mich</a></li>
            <li><div class="separator"></div></li>
            <li><button id="ProjectsButton" popovertarget="ProjectsPopover" class="link underline">Projects</button></li>
            <li class="grow"></li>
            <li><theme-select></theme-select></li>
            <li><div class="separator"></div></li>
            <li class="flex"><a href="https://github.com/LPTimey/Tim-Portfolio" class="link flex center" target="_blank" aria-label="Navigiert zum GitHub Repo"> <span class="underline mobile center">Source</span> ${gh_logo}</a></li>
        </ul>
        <ul id="ProjectsPopover" class="popover">
            <li><a href="./watchout.html" class="underline link" aria-label="Navigiert zu WatchOut">WatchOut</a></li>
            <li><div class="separator"></div></li>
            <li><a href="./printer.html" class="underline link" aria-label="Navigiert zur TouchScreen">TouchScreen</a></li>
            <li><div class="separator"></div></li>
            <li><a href="./styles.html" class="underline link" aria-label="Navigiert zur Themen & Stile">Themen & Stile</a></li>
            <li><div class="separator"></div></li>
            <li><a href="./tetris.html" class="underline link" aria-label="Navigiert zur Tetris in Arduino & C">Tetris in Arduino & C</a></li>
            <li><div class="separator"></div></li>
            <li><a href="./webdesign.html" class="underline link" aria-label="Navigiert zur Website Development">Website Development</a></li>
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
li{
    height: 100%;
    place-content: center;
}
nav {
    display: flex;
    flex-direction: column;
    padding-block:2rem;
    align-items: center;
    justify-content: center;
    gap: 0;
}
ul{
    display: flex;
    gap: 1ch;
    justify-content: center;
    align-items: center;
}
#ProjectsPopover{
    padding-left: 19ch;
    justify-content: start;
    flex-wrap: wrap;
}
svg {
    fill: var(--fg);
    /* stroke: var(--fg); */
    &:hover{
        fill: var(--accent);
    }
}

.popover{
    visibility:hidden;
    height: 0;
    /* width: 0; */
    opacity: 0;
    overflow: hidden;
}

nav:has(li:hover #ProjectsButton), nav:has(.popover:hover) {
    #ProjectsPopover{
        display: flex;
        height: max-content;
        width: 100%;
        margin-top: -0.5rem;
        padding-top: 1rem;
    }

    .popover{
        visibility: visible;
        opacity: 1;
    }
}

@media(width < 740px) {
    .popover{
        width: 0;
    }
    header{
        height: 100%;
    }
    nav {
        flex-direction: row;
        justify-content: space-around;
        align-items: center;
        gap: 0;
    }
    ul {
        flex-direction: column;
        gap: 3rem;

    }
    li:has(.separator),li:has(.grow),.separator,.grow{
        display: none;
        > * {
            display: none;
        }
    }
    li {
        height: fit-content;
        width: 100%;
        display: inline-flex;
        place-content: center;
    }

    #ProjectsPopover{
        padding-top: 0.5rem;
        padding-left: 0;
        justify-content:start;
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
