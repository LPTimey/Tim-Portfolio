"use strict";
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


/** TODO: Scaling the image
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
            img.classList.add("display-img");
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
                    htmlLink.textContent = "";
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
    el.className = "wrapper"
    console.log(el);
    console.log(el.innerHTML);
    return el.outerHTML;
};

const style = (debug) => css`
@import "setup.css";
:host{
    display: block;
    overflow: hidden;
    width: fit-content;
    height: fit-content;
}
.hyper-img-link {
    position: absolute;
    background: none;
    color: transparent;
    ${debug ? `border: 1px solid red;` : ''};
    cursor: pointer;
}
.wrapper{
    position: relative;
    display: grid;
    width: 100%;
    height: 100%;
    place-items: center;
    place-content: center;
    overflow: hidden;
    * {
        grid-column: 1 / -1;
        grid-row: 1 / -1;
    }
}
.img-wrapper{
    position: relative;
    display: grid;
    place-items: center;
    place-content: center;
    max-width: 100%;
    max-height: 100%;
    overflow: hidden;
}
.img-wrapper:not([active]){
    display: none;
}
img{
    max-width: 100%;
    max-height: 100%;
    object-fit: contain;
}
`;

export default class HyperImg extends HTMLElement {
    static get observedAttributes() {
        return ["src", "start-name", "debug"];
    }
    /**
     * 
     * @param {PressEvent} ev 
     */
    handlePress(ev) {
        // console.log(ev.src);
        // console.log(ev.destination);
        let isOverlay = null;

        //#region pre-handle Commands
        if (ev.destination.includes("#Overlay")) {
            console.log("overlay detected");
            isOverlay = true;
            ev.destination = ev.destination.replace("#Overlay", "");
        }
        if (ev.destination.includes("#Back")) {
            console.log("Back detected");
            let backName = ev.src.parentElement.querySelector(".bg-img").getAttribute("name");
            ev.destination = ev.destination.replace("#Back", backName);
        }
        //#endregion pre-handle Commands

        let destinationDiv = this.shadowRoot.querySelector(`div[name=${ev.destination}]`);

        //#region post-handle Commands
        if (isOverlay) {
            let bgImg = destinationDiv.querySelector("img.bg-img") ?? function () {
                let img = document.createElement("img");
                img.className = "bg-img";
                destinationDiv.insertBefore(img, destinationDiv.firstElementChild);
                return img;
            }()
            bgImg.setAttribute("name", ev.src.parentNode.getAttribute("name"));
            bgImg.src = ev.src.parentElement.querySelector(".display-img")?.src
        }
        //#endregion post-handle Commands

        // order is important 1st toggle dest. then as 2nd src
        destinationDiv.toggleAttribute("active");
        ev.src.parentElement.toggleAttribute("active");
    }

    async connectedCallback() {
        // console.log(this.innerHTML);
        let data;
        if (this.getAttribute("src")) {
            data = parseJSONC((await (await fetch(this.getAttribute("src"))).text()));
        } else {
            data = parseJSONC(this.innerHTML);
        }

        this.shadowRoot.innerHTML = style(this.hasAttribute("debug")) + template(data.data);

        // Listen to Buttons
        this.addEventListener(PressEvent.name, this.handlePress);
        // set EventListener for buttons
        this.shadowRoot.querySelectorAll("button.hyper-img-link").forEach((link) => {
            link.addEventListener("click", () => {
                this.dispatchEvent(new PressEvent({ bubbles: true, cancelable: true, composed: true, }, link, link.dataset.href));
            });
        })

        if (this.hasAttribute("start-name")) {
            this.shadowRoot.querySelectorAll(".img-wrapper[active]").forEach(el => el.removeAttribute("active"));
            this.shadowRoot.querySelector(`[name="${this.getAttribute("start-name")}"]`).toggleAttribute("active");
        }
    }

    constructor() {
        super();
        this.attachShadow({ mode: "open" });

    }
}
customElements.define("hyper-img", HyperImg);