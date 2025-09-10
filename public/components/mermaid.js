"use strict";
import mermaid from '../vendor/mermaid/mermaid-11.6.0/dist/mermaid.esm.min.mjs';

mermaid.initialize({ startOnLoad: false });
const parser = new DOMParser();
const svgPadding = 10;

/** @type {import('../vendor/mermaid/mermaid-11.6.0/dist/mermaid').MermaidConfig} */
const config = {
    // look: "handDrawn",
    look: "classic",
    htmlLabels: true,
    // themeVariables: { edgeLabelBackground: "transparent" },
    themeCSS: `
        .edgeLabel .label-background {
            fill: none !important;
            stroke: none !important;
        }
        .labelBkg, .edgeLabel *{
            background-color: rgba(0 0 0 / 0);
            border: none;
        }
        .edgeLabel p {
            background-color: rgba(from var(--bg) r g b / 0.9) !important;
            border-radius: 0.5em;
            padding-inline: 0.5ch;
            border: 1pt solid gray;
        }
    `
}
/** @type {import('../vendor/mermaid/mermaid-11.6.0/dist/mermaid').MermaidConfig} */
const configLight = {
    theme: "default",
    ...config
}
/** @type {import('../vendor/mermaid/mermaid-11.6.0/dist/mermaid').MermaidConfig} */
const configDark = {
    theme: "dark",
    darkMode: true,
    ...config
}

window.addEventListener('DOMContentLoaded', async () => {
    const containers = document.querySelectorAll('.diagram-container');
    containers.forEach(async container => {
        const definitions = [...container.querySelectorAll('.diagram-code')]
        if (!definitions) {
            container.innerHTML += `
            <p class="error-msg">no Definition found</p>
            <p class="error-msg">add a &lt;script class="diagram-code" type="text/plain"&gt;</p>`;
            return;
        }

        definitions.forEach(async definition => {
            try {
                const Name = (container?.getAttribute("name") || 'diagram-') + Math.random().toString(36).slice(2);
                const { svg: svg_light } = await mermaid.render(
                    (Name) + "THEME_Light",
                    `%%{init: ${JSON.stringify(configLight)}}%% ${definition.textContent}`
                );
                const { svg: svg_dark } = await mermaid.render(
                    (Name) + "THEME_Dark",
                    `%%{init: ${JSON.stringify(configDark)}}%% ${definition.textContent}`
                );

                container.innerHTML += svg_light;
                container.innerHTML += svg_dark;

                [...container.querySelectorAll('svg')].filter(svgElement =>
                    svgElement.id.includes(Name)
                ).forEach(svgElement => {
                    const g = /** @type {SVGGElement} */(svgElement.querySelector('g'));
                    const boundingBox = g.getBBox();

                    svgElement.setAttribute('viewBox', `${Math.max(0, boundingBox.x - svgPadding)} ${Math.max(0, boundingBox.y - svgPadding)} ${boundingBox.width + svgPadding} ${boundingBox.height + svgPadding}`);
                    svgElement.setAttribute('preserveAspectRatio', 'xMidYMid meet');
                    svgElement.style.width = '100%';
                    svgElement.style.height = 'auto';
                    // svgElement.style.overflow = 'visible';

                    if (svgElement.id.includes("THEME_Dark")) {
                        svgElement.classList.add("dark-only");
                    }
                    if (svgElement.id.includes("THEME_Light")) {
                        svgElement.classList.add("light-only");
                    }
                    let test = definition.classList;
                    // console.log(test);
                    svgElement.classList.add(...test);
                })

            } catch (err) {
                console.error('Fehler beim Mermaid-Rendern:', err);
            }
        })

    });
});