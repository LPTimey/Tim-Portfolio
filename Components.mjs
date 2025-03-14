"use strict";

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
        this.innerHTML = `<header><a href="./">Home</a> <a href="about.html">About</a></header>`
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
            <img src="${img_src??""}" alt="${subtitle}-image"/>
            <h2>${title ?? ""}</h2>
            <h3>${subtitle ?? ""}</h3>
            ${this.innerHTML}
        </section>
        `;
    }
}
customElements.define("project-section", ProjectSection)
