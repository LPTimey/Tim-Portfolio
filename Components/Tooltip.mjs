"use strict";
import { css, html } from "../script.mjs";


const template = html`
<div id="container">
    <slot name="trigger"></slot>
    <div id="popover">
        <slot name="content"></slot>
    </div>
</div>
`;

const style = css`
@import "setup.css";

:host {
    display: inline-block;
    position: relative;
}

#container {
    position: relative;
    display: inline-block;
}

#popover {
    position: absolute;
    top: 100%;
    left: 50%;
    transform: translate(-50%, 10px);
    background: rgb(from var(--bg) r g b / var(--transparency));
    backdrop-filter: blur(var(--border-r));
    -webkit-backdrop-filter: blur(var(--border-r));
    border-radius: 12px;
    box-shadow: 0 8px 24px rgb(from var(--fg) r g b / 0.2);
    padding: 12px 16px;
    color: var(--fg);
    font-size: 0.9rem;
    z-index: 2;
    opacity: 0;
    pointer-events: none;
    white-space: nowrap;
    transition: opacity 0.2s ease, transform 0.2s ease;
    min-width: 120px;
}

:host(:hover) #popover, #popover:hover, #container:hover #popover {
    opacity: 1;
    pointer-events: auto;
    transform: translate(-50%, 4px);
}
`;


/**
 * ## Examples:
 * 
 * Mit Text Label:
 * ```html
 * <tool-tip>
 *     <button slot="trigger">ℹ️ Info</button>
 *     <span slot="content">Dies ist ein Tooltip im Apple-Stil.</span>
 * </tool-tip>
 * ```
 * ---
 * 
 * Ohne Text Lable:
 * 
 * ```html
 *  <tool-tip>
 *      <div class="full" slot="trigger"></div>
 *      <span slot="content">Widerstände & LEDs</span>
 *  </tool-tip>
 * ```
 */
export default class ToolTip extends HTMLElement {
    static get observedAttributes() {
        return [];
    }
    connectedCallback() {
        this.shadowRoot.innerHTML = style + template;
        this.initialized = true;
    }
    constructor() {
        super()
        this.attachShadow({ mode: "open" });
        /** @type {ShadowRoot} */
        this.shadowRoot;
        this.initialized = false;
    }
}
customElements.define("tool-tip", ToolTip);