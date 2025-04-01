"use strict";

//#region svg's

const gh_logo = `
<svg class="gh_logo" height="32" aria-hidden="true" viewBox="0 0 24 24" version="1.1" width="32" data-view-component="true">
    <path d="M12.5.75C6.146.75 1 5.896 1 12.25c0 5.089 3.292 9.387 7.863 10.91.575.101.79-.244.79-.546 0-.273-.014-1.178-.014-2.142-2.889.532-3.636-.704-3.866-1.35-.13-.331-.69-1.352-1.18-1.625-.402-.216-.977-.748-.014-.762.906-.014 1.553.834 1.769 1.179 1.035 1.74 2.688 1.25 3.349.948.1-.747.402-1.25.733-1.538-2.559-.287-5.232-1.279-5.232-5.678 0-1.25.445-2.285 1.178-3.09-.115-.288-.517-1.467.115-3.048 0 0 .963-.302 3.163 1.179.92-.259 1.897-.388 2.875-.388.977 0 1.955.13 2.875.388 2.2-1.495 3.162-1.179 3.162-1.179.633 1.581.23 2.76.115 3.048.733.805 1.179 1.825 1.179 3.09 0 4.413-2.688 5.39-5.247 5.678.417.36.776 1.05.776 2.128 0 1.538-.014 2.774-.014 3.162 0 .302.216.662.79.547C20.709 21.637 24 17.324 24 12.25 24 5.896 18.854.75 12.5.75Z"></path>
</svg>`;
const light_mode = `
<svg xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 -960 960 960" width="24px">
    <path d="M480-360q50 0 85-35t35-85q0-50-35-85t-85-35q-50 0-85 35t-35 85q0 50 35 85t85 35Zm0 80q-83 0-141.5-58.5T280-480q0-83 58.5-141.5T480-680q83 0 141.5 58.5T680-480q0 83-58.5 141.5T480-280ZM200-440H40v-80h160v80Zm720 0H760v-80h160v80ZM440-760v-160h80v160h-80Zm0 720v-160h80v160h-80ZM256-650l-101-97 57-59 96 100-52 56Zm492 496-97-101 53-55 101 97-57 59Zm-98-550 97-101 59 57-100 96-56-52ZM154-212l101-97 55 53-97 101-59-57Zm326-268Z"/>
</svg>`;
const dark_mode = `
<svg xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 -960 960 960" width="24px">
    <path d="M480-120q-150 0-255-105T120-480q0-150 105-255t255-105q14 0 27.5 1t26.5 3q-41 29-65.5 75.5T444-660q0 90 63 153t153 63q55 0 101-24.5t75-65.5q2 13 3 26.5t1 27.5q0 150-105 255T480-120Zm0-80q88 0 158-48.5T740-375q-20 5-40 8t-40 3q-123 0-209.5-86.5T364-660q0-20 3-40t8-40q-78 32-126.5 102T200-480q0 116 82 198t198 82Zm-10-270Z" />
</svg>`;
const system_mode = `
<svg xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 -960 960 960" width="24px">
    <path d="M80-160v-120h80v-440q0-33 23.5-56.5T240-800h600v80H240v440h240v120H80Zm520 0q-17 0-28.5-11.5T560-200v-400q0-17 11.5-28.5T600-640h240q17 0 28.5 11.5T880-600v400q0 17-11.5 28.5T840-160H600Zm40-120h160v-280H640v280Zm0 0h160-160Z"/>
</svg>`;
const toTop = `
<svg xmlns="http://www.w3.org/2000/svg" height="24px" width="24px" viewBox="0 -960 960 960" width="24px">
    <path d="M160-760v-80h640v80H160Zm280 640v-408L336-424l-56-56 200-200 200 200-56 56-104-104v408h-80Z"/>
</svg>
`;

//#endregion svg's

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
    /**
     * 
     * @param {string} name name of attribute
     * @param {*} oldValue old value of attribute
     * @param {*} newValue new value of attribute
     */
    attributeChangedCallback(name, oldValue, newValue) {
        return;
    }
    constructor() {
        super()

        this.innerHTML = `
        <header>
            <a href="./" class="underline">Home</a>
            |
            <a href="about.html" class="underline">Über Mich</a>
            |
            <div class="popover-wrapper">
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
            <div class="grow"></div>
            <theme-selector id="Theme" class="underline"></theme-selector>
            |
            <a href="https://github.com/LPTimey/Tim-Portfolio" target="_blank" class="center">${gh_logo}</a>
        </header>`;
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
            Created by Tim Ruland
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
        link.innerHTML = toTop
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
