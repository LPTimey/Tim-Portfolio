const template = `

`;
const style = `<style>
</style>`;
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
        this.attachShadow({ mode: "open" })
        this.shadowRoot.innerHTML = style + template;
    }
    constructor() {
        super()
    }
}
customElements.define("site-header", SiteHeader);