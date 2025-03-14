"use strict";

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
        ${this.ids.map((id,index) => {
            return `<label for="${id}">${children[index].innerHTML}</label>`
        })}
        `
    }
}
customElements.define("multi-state-button", MultiStateButton)

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
            <a href="about.html">About</a>
            <multi-state-button ids="theme_system;theme_dark;theme_light">
                <span>System</span>
                <span>Dark</span>
                <span>Light</span>
            </multi-state-button>
        </header>`
    }
}
customElements.define("site-header", SiteHeader)

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
        super()
        this.innerHTML = `<footer></footer>`
    }
}
customElements.define("site-footer", SiteFooter)

class SiteHero extends HTMLElement {
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
        super()
        this.innerHTML = `<section><h1>Tim Ruland</h1></section><hr />`
    }
}
customElements.define("site-hero", SiteHero)

class ProjectSection extends HTMLElement {
    static get observedAttributes() {
        return ['title', 'subtitle', 'img-src'];
    }
    constructor() {
        super()
        let title = this.getAttribute("title");
        let subtitle = this.getAttribute("subtitle");
        let img_src = this.getAttribute("img-src");
        this.innerHTML = `
        <section>
            <img src="${img_src ?? ""}" alt="${subtitle}-image"/>
            <h2>${title ?? ""}</h2>
            <h3>${subtitle ?? ""}</h3>
            ${this.innerHTML}
        </section>
        `;
    }
}
customElements.define("project-section", ProjectSection)
