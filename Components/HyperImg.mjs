import { css, html } from "../script.mjs";

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
                    htmlLink.href = link.href;
                    htmlLink.style.top = link.pos.top;
                    htmlLink.style.right = link.pos.right;
                    htmlLink.style.bottom = link.pos.bottom;
                    htmlLink.style.left = link.pos.left;
                    htmlLink.classList.add("hyper-img-link")
                    return htmlLink
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
`;

/**
 * @typedef HyperImgData
 * @prop {string} name
 * @prop {URL} src
 * @prop {Array<LinkData>} links
 */

/**
 * @typedef LinkData
 * @prop {string} name
 * @prop {URL} href
 * @prop {Object} pos
 * @prop {0|string} pos.top
 * @prop {0|string} pos.right
 * @prop {0|string} pos.bottom
 * @prop {0|string} pos.left
*/

export default class HyperImg extends HTMLElement {
    connectedCallback() {
        console.log(this.innerHTML);
        const data = JSON.parse(this.innerHTML);
        console.log(data);
        this.shadowRoot.innerHTML = style + template(data);
    }
    constructor() {
        super()
        this.attachShadow({ mode: "open" })
    }
}
customElements.define("hyper-img", HyperImg);