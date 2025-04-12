import { css, html, to_top_icon } from "../script.mjs";

const template = html`
<a href="#" class="button" aria-label="Hoch scrollen">${to_top_icon}</a>
`;

const style = css`
@import "setup.css";
a {
    --b-bg: rgba(from var(--bg) r g b / var(--transparency));
    position: fixed;
    bottom: 3rem;
    right: calc(var(--outer-pad-inline) / 2);

    padding: 1.5ch;

    border-radius: 50%;
    border: 1px solid var(--fg);

    -webkit-backdrop-filter: blur(var(--blur-r));
    backdrop-filter: blur(var(--blur-r));
    &:hover{
        border-color: transparent;
    }
}


`;

export default class ToTop extends HTMLElement {
    static toTopButtonDelta = 125;

    toTopButtonDisplay(window) {
        if (window.pageYOffset < ToTop.toTopButtonDelta) {
            this.style.visibility = "hidden"
        } else {
            this.style.visibility = "visible"
        }
    }
    connectedCallback() {
        this.shadowRoot.innerHTML = style + template;

        window.addEventListener("scroll", (ev) => this.toTopButtonDisplay(window));
        this.toTopButtonDisplay(window);
    }
    constructor() {
        super()
        this.attachShadow({ mode: "open" })
    }
}
customElements.define("to-top", ToTop);