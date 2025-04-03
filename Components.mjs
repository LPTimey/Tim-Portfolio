"use strict";

import { gh_logo, burger_icon, light_mode_icon, dark_mode_icon, system_mode_icon, toTop_icon, close_icon } from "./script.mjs"

//#region CmpImages
export class CmpImages extends HTMLElement {
    static get observedAttributes() {
        return ['src1', 'alt1', 'src2', 'alt2', "bg"];
    }
    get template() {

        this.image2Div.classList.add("img-comp-overlay")

        this.img1.onload = () => {
            var height = this.img1.height;
            var width = this.img1.width;
            this.img1.style.aspectRatio = width / height;
        }
        this.img2.onload = () => {
            var height = this.img2.height;
            var width = this.img2.width;
            this.img2.style.aspectRatio = width / height;
        }

        this.img1.src = this.getAttribute("src1");
        this.img2.src = this.getAttribute("src2");
        this.img1.alt = this.getAttribute("alt1");
        this.img2.alt = this.getAttribute("alt2");

        this.input.classList.add("slider")
        this.input.type = "range";
        this.input.min = "0";
        this.input.max = "1000";
        this.input.value = "500";
        this.input.style.width = "100%"

        this.image1Div.appendChild(this.img1);
        this.image2Div.appendChild(this.img2);
        this.inputDiv.appendChild(this.input);

        return [this.image1Div, this.image2Div, this.inputDiv]
    }
    get styleSheet() {
        return `
* {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
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
}

.img-comp-overlay {
    position: relative;
    overflow: hidden;
    img{
        position: absolute;
        left: 0;
        height: 100%;
        z-index: 0;
        object-fit: cover;
        object-position: left;
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
    z-index: 1;
}
        `;
    }
    initComparisons() {
        const clip = (target) => {
            this.image2Div.style.width = `${target.value / 10}%`
        }
        clip(this.input);
        this.input.addEventListener("input", (ev) => clip(ev.target))
    }
    connectedCallback() {
        this.attachShadow({ mode: "open" })

        let style = document.createElement("style");
        style.innerHTML = this.styleSheet;

        this.shadowRoot.appendChild(style);
        this.template.forEach((node) => this.shadowRoot.appendChild(node));
        this.initComparisons();
    }
    constructor() {
        super();
        /** @type {HTMLDivElement} */
        this.image1Div = document.createElement("div");
        /** @type {HTMLDivElement} */
        this.image2Div = document.createElement("div");
        /** @type {HTMLDivElement} */
        this.inputDiv = document.createElement("div");
        /** @type {HTMLDivElement} */
        this.div4 = document.createElement("div");
        /** @type {HTMLImageElement} */
        this.img1 = document.createElement("img");
        /** @type {HTMLImageElement} */
        this.img2 = document.createElement("img");
        /** @type {HTMLInputElement} */
        this.input = document.createElement("input");
    }
}
customElements.define("cmp-img", CmpImages)
//#endregion CmpImages

//#region PhoneImage
export class PhoneImage extends HTMLElement {
    static get observedAttributes() {
        return ['src', 'alt', "bg"];
    }
    get template() {
        let phone = document.createElement("img");
        // phone.loading = "lazy";
        // phone.decoding = "async";
        phone.src = "assets/iPhone Template [Konvertiert] noBG.png";
        phone.alt = "";
        phone.classList.add("phone");

        let screen = document.createElement("img");
        // screen.loading = "lazy";
        // screen.decoding = "async";
        screen.src = this.getAttribute("src");
        screen.alt = this.getAttribute("alt");
        screen.classList.add("screen");

        return [screen, phone]
    }
    get styleSheet() {
        return `
:host{
    --iphone16PM-aspect: 1420 / 2868;
    aspect-ratio: var(--iphone16PM-aspect);
    max-height: 100%;
    position: relative;
    display:  block;
    z-index:  0;
    width: 100%;
}
img{
    object-fit: fill;
    max-height: 100%;
    max-width: 100%;
    position: absolute;
    display: block;
    height: auto;
    width: auto;
    left: 50%;
    top: 50%;
    transform: translate(-50%, -50%);
}
.screen {
    border-radius: 10% / 5% ;
    height: 96%;
    ${this.getAttribute("bg") ? `background: ${this.getAttribute("bg")}` : ""};
}
.phone{
    height: 100%;
    aspect-ratio: var(--iphone16PM-aspect);
}
        `;
    }
    connectedCallback() {
        this.attachShadow({ mode: "open" })

        let style = document.createElement("style");
        style.innerHTML = this.styleSheet;

        this.shadowRoot.appendChild(style);
        this.template.forEach((node) => this.shadowRoot.appendChild(node));
    }
    constructor() {
        super();
    }
}
customElements.define("phone-image", PhoneImage)
//#endregion PhoneImage

//#region ScrollImage
export class ScrollImage extends HTMLElement {
    static get observedAttributes() {
        return ['src', 'alt', "bg", "cols", "rows", "time"];
    }
    get template() {

        const imgNr = this.getAttribute("cols") * this.getAttribute("rows");
        // const imgNr = 2 * 3;
        let elements = [...Array(imgNr).keys()].map(() => {
            let img = document.createElement("img");
            // img.loading = "lazy";
            // img.decoding = "async";
            img.src = this.getAttribute("src");
            img.alt = this.getAttribute("alt");
            img.classList.add("scrolling-image");
            return img;
        })

        return elements
    }
    get styleSheet() {
        return `
*{
    margin:  0;
    padding: 0;
    box-sizing: border-box;
}
:host{
    display:  grid;
    grid-template-columns: repeat(${this.getAttribute("cols")},1fr);
    position: relative;
    width:    calc(100% * ${this.getAttribute("cols")});
    height:   100%;
    overflow: hidden;
    z-index:  -1;
    left:     -100%;
    background: ${this.getAttribute("bg")};

    animation:  scrollGrid ${this.getAttribute("time")} linear infinite;
}
img {
    display: block;
    width:   100%;
    height:  100%;
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
    animation: scrollImage ${this.getAttribute("time")} linear infinite;
    ${this.getAttribute("bg") ? `background: ${this.getAttribute("bg")}` : ""};
}

@keyframes scrollImage {
    0% {
        translate: 0 0;
    }

    100% {
        translate: 0 -100%;
        /* Zweites Bild kommt an die Stelle des ersten Bildes */
    }
}

@keyframes scrollGrid {
    0% {
        translate: 0 0;
    }

    100% {
        translate: calc(100% / ${this.getAttribute("cols")}) 0;
    }
}
        `;
    }
    connectedCallback() {
        this.attachShadow({ mode: "open" })

        let style = document.createElement("style");
        style.innerHTML = this.styleSheet;

        this.shadowRoot.appendChild(style);
        this.template.forEach((node) => this.shadowRoot.appendChild(node));
    }
    constructor() {
        super();

        this.setAttribute("cols", this.getAttribute("cols") ?? 2);
        this.setAttribute("rows", this.getAttribute("rows") ?? 3);
        this.setAttribute("time", this.getAttribute("time") ?? "25s");
    }
}
customElements.define("scroll-image", ScrollImage);
//#endregion ScrollImage

//#region SiteHeader
export class SiteHeader extends HTMLElement {
    static get observedAttributes() {
        return [];
    }
    get template() {
        let burger = document.createElement("div");
        burger.innerHTML = `
            <input type="checkbox" name="NavBarOpen" id="NavBarOpen" hidden checked>
            <div>
                <label id="Burger" class="nav-button" for="NavBarOpen">${burger_icon}</label>
                <label id="Close" class="nav-button" for="NavBarOpen">${close_icon}</label>
            </div>
        `;
        burger.classList.add("mobile");
        burger.classList.add("nav-buttons");
        let header = document.createElement("header");
        header.innerHTML = `
            <ul>
                <li><a href="./" class="underline">Home</a></li>
                <li><span class="separator"></span></li>
                <li><a href="about.html" class="underline">Über Mich</a></li>
                <li><span class="separator"></span></li>
                <li><div class="popover-wrapper">
                    <button id="ProjectAnchor" popovertarget="Projects" popover-hover class="underline link"> Projekte &#709; </button>

                    <div id="Projects" popover>
                        <ul id="ProjectsGrid">
                            <li><a href="watchout.html" class="underline">Watch Out</a></li>
                            <li><a href="printer.html" class="underline">Touch Screen</a></li>
                            <li><a href="themes.html" class="underline">Themen & Stile</a></li>
                            <li><a href="tetris.html" class="underline">Arduino Tetris</a></li>
                            <li><a href="webdev.html" class="underline">Website Design</a></li>
                        </ul>
                    </div>
                </div>
                </li>
                <div class="grow"></div>
                <li><theme-selector id="Theme" class="underline"></theme-selector></li>
                <li><span class="separator"></span></li>
                <li><a href="https://github.com/LPTimey/Tim-Portfolio" target="_blank" class="center flex"><span class="center mobile">Source</span> ${gh_logo}</a></li>
            </ul>
            `;
        return [burger, header];
    }
    get styleSheet() {
        return `

site-header {
    min-height: 1.5rem;

    position: sticky;
    top: 0;
    z-index: 100;
    header{
        position: sticky;
        top: 0;

        background-color: rgba(from var(--bg) r g b / var(--transparency));
        -webkit-backdrop-filter: blur(6px);
        backdrop-filter: blur(6px);
        padding: 2em 0;

        display: flex;
        justify-content: center;

        li {
            list-style: none;
            margin: 0;
            padding:0;
        }

    }
}

header > ul {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 1ch;
    width: var(--outer-width);
}

.popover-wrapper {
    position: relative;
    /* Um sicherzustellen, dass das Popover relativ zum Button positioniert wird */
}

#ProjectAnchor {
    anchor-name: --project-anchor;
}

#Projects {
    position: fixed;
    position-anchor: --project-anchor;
    margin: 0;
    inset: auto;
    /* fix for no anchor*/
    top: 4.5rem;
    top: anchor(bottom);
    left: anchor(left);

    z-index: 999;
    border: none;
    padding: 1rem 2ch;
    width: max-content;
    border-radius: var(--border-r);
    background-color: rgba(from var(--bg) r g b / var(--transparency));
    -webkit-backdrop-filter: blur(5px);
    backdrop-filter: blur(5px);

    &:popover-open {
        display: grid;
        gap: 1ch;

        #ProjectsGrid {
            width: 100%;
            display: grid;
            gap: 1ch;
        }

        li {
            list-style: none;
        }

        >* {
            width: max-content;
        }
    }
}

@media (width<740px) {
    /*TODO: make topbar into burger*/
    
    site-header {
        max-width:100%;
        overflow: hidden;
        header{
            position: fixed;
            inset: 0;
            display: grid;
            place-items: center;
            transition: transform 0.2s ease-in-out;
        }
        .nav-button{
            position: fixed;
            display: grid;
            width:fit-content;
            aspect-ratio: 1;
            place-items: center;
            cursor:pointer;
            padding: 1ch;
            top:5%;
            right:10%;
            z-index: 1000;
            svg{
                width: 2rem;
                height: 2rem;
            }
        }
        #Burger{
            border-radius: var(--border-r);
            background-color: rgba(from var(--bg) r g b / var(--transparency));
            -webkit-backdrop-filter: blur(5px);
            backdrop-filter: blur(5px);
        }

        &:has(#NavBarOpen:checked){
            header{
                transform: translateX(100%);
            }
            #Burger{
                visibility: visible;
            }
            #Close{
                visibility: hidden;
            }
        }
        &:has(#NavBarOpen:not(:checked)){
            #Burger{
                visibility: hidden;
            }
            #Close{
                visibility: visible;
            }
        }
    }
    .separator{
        display: none;
    }
    header > ul {
        flex-direction: column;
        gap: 1.75rem;
        /* left align text but element centered
        align-items: start;
        width: fit-content;
        */
    }
}
        `;
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
        // this.attachShadow({mode:"open"})
        let style = document.createElement("style");
        style.innerHTML = this.styleSheet;
        this.appendChild(style)
        this.template.forEach(el => this.appendChild(el));
    }
    constructor() {
        super()
    }
}
customElements.define("site-header", SiteHeader);
//#endregion SiteHeader

//#region SiteFooter
export class SiteFooter extends HTMLElement {
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
        // name will be "value"
        // oldValue will be "7" 
        // newValue will be "11" 

        // your code...
    }
    constructor() {
        super();
        this.innerHTML = `
        <to-top></to-top>
        <footer>
            Created with HTML, CSS & JS by Tim Ruland © 2025
        </footer>`;
    }
}
customElements.define("site-footer", SiteFooter);
//#endregion SiteFooter

//#region ThemeSelect
export class ThemeSelect extends HTMLElement {
    static values = [
        { val: "ThemeLight", display: "Licht" },
        { val: "ThemeSystem", display: "System", default: true },
        { val: "ThemeDark", display: "Dunkel" }];
    static key = "theme"

    get theme() {
        return this.querySelector("#ThemeSelect").value
    }
    /**
     * @param {string} theme
     */
    set theme(theme) {
        if (!ThemeSelect.values.map((va) => va.val).includes(theme)) {
            this.querySelector("#ThemeSelect").value = ThemeSelect.values.find(va => va.default)?.val || ThemeSelect.values[0].val
            return;
        }
        this.querySelector("#ThemeSelect").value = theme
    }

    static getLocalStorageTheme() {
        return window.localStorage.getItem(ThemeSelect.theme);
    }
    static setLocalStorageTheme(theme) {
        window.localStorage.setItem(ThemeSelect.theme, theme);
    }

    constructor() {
        super();
        const str = `
        <select id="ThemeSelect">
        ${ThemeSelect.values.map(opt => `<option id="${opt.val}" value="${opt.val}">${opt.display}-Thema</option>`).reduce((carry, curr) => carry + '\n' + curr)}
        </select>
        `
        this.innerHTML = str;
        this.theme = ThemeSelect.getLocalStorageTheme();
        this.addEventListener("change", (ev) => {
            ThemeSelect.setLocalStorageTheme(this.theme)
        })
    }
}
customElements.define("theme-selector", ThemeSelect);
//#endregion ThemeSelect

//#region ToTop
export class ToTop extends HTMLElement {
    static toTopButtonDelta = 100;

    toTopButtonDisplay(window) {
        const toTopButton = this.shadowRoot.querySelector("#ToTop");

        if (window.pageYOffset < ToTop.toTopButtonDelta) {
            toTopButton.style.visibility = "hidden"
        } else {
            toTopButton.style.visibility = "visible"
        }
    }
    get template() {
        let link = document.createElement("a");
        link.innerHTML = toTop_icon
        link.id = "ToTop";
        link.href = "#"
        return link;
    }
    get styleSheet() {
        return `
#ToTop {
    visibility: hidden;
    color: var(--fg);
    fill: var(--fg);
    stroke: var(--fg);
    position: fixed;
    bottom: 3dvh;
    right: 5dvw;
    border-radius: 100%;
    border: 2px solid var(--fg);
    height: 3em;
    width: 3em;
    display: grid;
    place-items: center;
    background-color: rgba(from var(--bg) r g b / var(--transparency));

    &:hover  {
        color: var(--light);
        border-color: var(--light);

        svg {
            stroke: var(--light);
            fill: var(--light);
        }

        background-color: var(--accent);
    }
}`;
    }

    connectedCallback() {
        this.attachShadow({ mode: "open" })
        let [style] = [document.createElement("style")]
        style.innerHTML = this.styleSheet;

        this.shadowRoot.appendChild(style);
        this.shadowRoot.appendChild(this.template);
        window.addEventListener("scroll", (ev) => this.toTopButtonDisplay(window));
        this.toTopButtonDisplay(window);
    }
    constructor() {
        super();
    }
}
customElements.define("to-top", ToTop);
//#endregion ToTop
