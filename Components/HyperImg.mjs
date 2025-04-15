import { css, html, parseJSONC } from "../script.mjs";

/**
 * @typedef HyperImgData
 * @prop {string} name
 * @prop {string} src
 * @prop {Array<LinkData>} links
 */

/**
 * @typedef LinkData
 * @prop {string} name
 * @prop {string} href
 * @prop {Object} pos
 * @prop {0|string} pos.top
 * @prop {0|string} pos.right
 * @prop {0|string} pos.bottom
 * @prop {0|string} pos.left
*/

/**
 * @extends {Event}
 */
class PressEvent extends Event {
    static name = "press-event"
    /**
     * @param {EventInit} options
     * @param {HTMLButtonElement} src
     * @param {string} destination
     */
    constructor(options, src, destination) {
        super(PressEvent.name, options);
        /** @type {HTMLButtonElement} */
        this.src = src;
        /** @type {string} */
        this.destination = destination;
    }
}


/** TODO: Impl This: A way to have multiple images wich link to each other through 
 * hidden interaction areas, emulating a Prototype
 * @type {(data: HyperImgData[])=>string}
 */
const template = (datas) => {
    let el = datas
        .map(data => {
            console.log(data);
            console.log(data.src);
            console.log(data.links);
            const img = document.createElement("img");
            img.setAttribute("name", data.name)
            img.src = data.src;
            img.alt = "";
            return { img, links: data.links };
        })
        .map(data => {
            console.log(data)
            return {
                img: data.img,
                links: data.links.map(link => {
                    console.log(link)
                    const htmlLink = document.createElement("button");
                    htmlLink.setAttribute("name", link.name);
                    htmlLink.textContent = "t";
                    htmlLink.dataset.href = link.href;
                    htmlLink.style.top = link.pos.top;
                    htmlLink.style.right = link.pos.right;
                    htmlLink.style.bottom = link.pos.bottom;
                    htmlLink.style.left = link.pos.left;
                    htmlLink.classList.add("hyper-img-link");
                    return htmlLink;
                })
            }
        })
        .map(data => {
            let div = document.createElement("div");
            div.className = "img-wrapper";
            div.appendChild(data.img);
            data.links.forEach(link => div.appendChild(link));
            div.setAttribute("name", data.img.getAttribute("name"));
            return div;
        })
        .reduce((prev, cur, i) => {
            if (i == 0) {
                cur.toggleAttribute("active")
            }
            prev.appendChild(cur);
            return prev;
        }, document.createElement("div"));
    console.log(el);
    console.log(el.innerHTML);
    return el.innerHTML;
};

const style = css`
@import "setup.css";
.hyper-img-link {
    position: absolute;
    background: none;
    color: transparent;
    border: 1px solid red;
    cursor: pointer;
}
.img-wrapper{
    position: relative;
    width: fit-content;
    height: fit-content;
}
.img-wrapper:not([active]){
    display: none;
}
`;

export default class HyperImg extends HTMLElement {
    static get observedAttributes() {
        return ["src"];
    }
    /**
     * 
     * @param {PressEvent} ev 
     */
    handlePress(ev) {
        console.log(ev.src);
        console.log(ev.destination);
    }

    async connectedCallback() {
        // console.log(this.innerHTML);
        let data;
        if (this.getAttribute("src")) {
            data = parseJSONC((await (await fetch(this.getAttribute("src"))).text()));
        } else {
            data = parseJSONC(this.innerHTML);
        }
        // console.log(data);
        this.shadowRoot.innerHTML = style + template(data.data);
        // Listen to Buttons
        this.addEventListener(PressEvent.name, this.handlePress);
        // set EventListener for buttons
        this.shadowRoot.querySelectorAll("button.hyper-img-link").forEach((link)=>{
            link.addEventListener("click", () => {
                console.log("clicked");
                this.dispatchEvent(new PressEvent({ bubbles: true, cancelable: true, composed: true, }, link, link.dataset.href));
            });
        })
    }

    constructor() {
        super();
        this.attachShadow({ mode: "open" });

    }
}
customElements.define("hyper-img", HyperImg);