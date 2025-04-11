import { css, html } from "../script.mjs";

const template = function (src1, alt1, src2, alt2) {
    return html`
<div>
    <img src="${src1}" alt="${alt1}">
</div>
<div class="img-comp-overlay">
    <img src="${src2}" alt="${alt2}">
</div>
<div>
    <input type="range" name="" id="" min="0" max="1000" value="500" class="slider">
</div>
`;
}

const style = css`
@import "setup.css";

*{
    transition: width 0s;
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
    height:100%;
}

div{
    overflow: hidden;
}

.img-comp-overlay {
    position: relative;
    overflow: hidden;
    img{
        position: absolute;
        left: 0;
        height: 100%;
        z-index: 1;
        object-fit: cover;
        /* object-position: left; */
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
    z-index: 2;
}
`;

export default class ImgCmp extends HTMLElement {
    static get observedAttributes() {
        return ["src1", "alt1", "src2", "alt2"];
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
    initComparisons() {
        const clip = (target) => {
            this.shadowRoot.querySelector(".img-comp-overlay").style.width = `${target.value / 10}%`
        }
        clip(this.shadowRoot.querySelector("input"));
        this.shadowRoot.querySelector("input").addEventListener("input", (ev) => clip(ev.target))
    }
    connectedCallback() {
        this.shadowRoot.innerHTML =
            style
            + template(this.getAttribute("src1"), this.getAttribute("alt1"),
                this.getAttribute("src2"), this.getAttribute("alt2"));

        this.initComparisons();
    }
    constructor() {
        super()
        this.attachShadow({ mode: "open" })
    }
}
customElements.define("img-cmp", ImgCmp);