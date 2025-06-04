"use strict";
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
            <li><details id="ProjectsButton" >
                <summary class="link underline">Projekte</summary>
                <ul id="ProjectsPopover">
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
            </details></li>
            <li class="grow"></li>
            <li><theme-select></theme-select></li>
            <li><div class="separator"></div></li>
            <li class="flex" style="margin-top: -0.25%;"><a href="https://github.com/LPTimey/Tim-Portfolio" class="link flex center" target="_blank" aria-label="Navigiert zum GitHub Repo"> <span class="underline mobile center">Source</span> ${gh_logo}</a></li>
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
.scrolled{
    /* nur unten ein leichter Schatten */
    box-shadow: 0 4px 10px rgba(0, 0, 0, 0.1);
}

.current{
    font-weight: var(--fw-medium);
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
    align-items: start;
}
details{
    overflow:hidden;
    &::details-content{
        block-size: 0;
        transition: block-size 0.25s ease-in-out, content-visibility 0.25s ease-in-out allow-discrete;
    }
    &[open]::details-content{
        block-size: auto;
    }
}
summary{
    width: fit-content;
}
#ProjectsPopover{
    padding-left: 1.5ch;
    padding-top: 0.5em;
}
a{
    width: fit-content;
}
@media(max-width: 740px) {
    :host{
        --i-gap: 3rem;
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
        gap: var(--i-gap);

    }
    li:has(>.separator),li:has(>.grow),.separator,.grow{
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

    summary{
        margin: auto;
    }

    #ProjectsPopover{
        --h:  calc(var(--i-gap) * 0.66);
        padding-top: var(--h);
        gap: var(--h);
        height: max-content;
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
    setCurrent() {
        let currentUrlPath = window.location.pathname;

        // @ts-ignore
        [...this.shadowRoot.querySelectorAll("[href]")]
            .map(el => {
                const anchor = /** @type {HTMLAnchorElement} */ (el);
                /** @type {{el: HTMLAnchorElement, url: URL}} */
                let res = { el: anchor, url: new URL(anchor.href) };
                return res;
            })
            .forEach(({ el, url }) => {
                if (url.pathname === currentUrlPath) {
                    el.classList.add("current");
                    el.classList.add("permanent");
                } else {
                    el.classList.remove("current");
                    el.classList.remove("permanent");
                }
            });

        console.log(currentUrlPath);
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
        // @ts-ignore
        this.shadowRoot.innerHTML = style + template;
        this.setCurrent();

        const navbar = this.shadowRoot?.querySelector('header');
        window.addEventListener('scroll', () => {
            if (window.scrollY > 5) {
                navbar?.classList.add('scrolled');
            } else {
                navbar?.classList.remove('scrolled');
            }
        });
    }
    constructor() {
        super()
        this.attachShadow({ mode: "open" })
    }
}
customElements.define("site-header", SiteHeader);
