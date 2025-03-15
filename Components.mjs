"use strict";

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

class MultiStateButton extends HTMLElement {
    static get observedAttributes() {
        return ["ids"];
    }
    constructor() {
        super();

        /**
         * @type {string[]}
         */
        this.ids = (this.getAttribute("ids") ?? "").split(";");

        let children = this.children ?? new HTMLCollection;

        this.innerHTML = `
        ${this.ids.map((id, index) => {
            return `<label for="${id}">${children[index].outerHTML}</label>`
        }).toString().replaceAll(",", "")}
        `
        this.classList.add("center");
    }
}
customElements.define("multi-state-button", MultiStateButton);

class SiteHeader extends HTMLElement {
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
        // fetch("header.html")
        //     .then(req => req.text())
        //     .then((text) => this.innerHTML = text.toString())
        this.innerHTML = `
        <div hidden>
        <input type="radio" name="theme" value="system" id="theme_system" checked>
        <input type="radio" name="theme" value="dark" id="theme_dark">
        <input type="radio" name="theme" value="light" id="theme_light">
        </div>
        <header>
            <a href="./">Home</a> 
            |
            <a href="about.html">About</a>
            <div class="grow"></div>
            <multi-state-button ids="theme_dark;theme_system;theme_light">
                <span class="link" id="dark">${dark_mode}</span>
                <span class="link" id="system">${system_mode}</span>
                <span class="link" id="light">${light_mode}</span>
            </multi-state-button>
            |
            <a href="https://github.com/LPTimey/Tim-Portfolio" class="center">Source ${gh_logo}</a>
        </header>`;
    }
}
customElements.define("site-header", SiteHeader);

class SiteFooter extends HTMLElement {
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
        this.innerHTML = `<footer></footer>`;
    }
}
customElements.define("site-footer", SiteFooter);

class TitleImage extends HTMLElement {
    static get observedAttributes() {
        return ['alt', 'src'];
    }
    constructor() {
        super();
        let src = this.getAttribute("src");
        let alt = this.getAttribute("alt");

        this.innerHTML = `
        <img src="${src ?? ""}" alt="${alt}" />
        `;
    }
}
customElements.define("title-img", TitleImage);

class ProjectSection extends HTMLElement {
    static get observedAttributes() {
        return ['title', 'subtitle', 'img-src'];
    }
    constructor() {
        super();
        let title = this.getAttribute("title");
        let subtitle = this.getAttribute("subtitle");
        let img_src = this.getAttribute("img-src");
        this.innerHTML = `
            <title-img src="${img_src ?? ""}" alt="${subtitle}-image"></title-img>
            <h2>${title ?? ""}</h2>
            <h3>${subtitle ?? ""}</h3>
            <div>${this.innerHTML}</div>
        `;
    }
}
customElements.define("project-section", ProjectSection);
