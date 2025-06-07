"use strict";
import mermaid from './vendor/mermaid/mermaid-11.6.0/dist/mermaid.esm.min.mjs';

mermaid.initialize({ startOnLoad: false, logLevel: 1 });

/** @type {import('./vendor/mermaid/mermaid-11.6.0/dist/mermaid').MermaidConfig} */
const config = {
    look: "handDrawn",
    // themeVariables: { edgeLabelBackground: "transparent" },
    // themeCSS: `
    //     .edgeLabel .label-background {
    //         fill: none !important;
    //         stroke: none !important;
    //     }
    //     .edgeLabel, .edgeLabel * {
    //         // background-color: rgba(from var(--bg) / 0.1) !important;
    //     }
    // `
}
/** @type {import('./vendor/mermaid/mermaid-11.6.0/dist/mermaid').MermaidConfig} */
const configLight = {
    theme: "default",
    ...config
}
/** @type {import('./vendor/mermaid/mermaid-11.6.0/dist/mermaid').MermaidConfig} */
const configDark = {
    theme: "dark",
    darkMode: true,
    ...config
}

window.addEventListener('DOMContentLoaded', async () => {
    const containers = document.querySelectorAll('.diagram-container');
    containers.forEach(async container => {
        const definitions = container.querySelectorAll('.diagram-code');
        if (!definitions) {
            container.innerHTML += `
            <p class="error-msg">no Definition found</p>
            <p class="error-msg">add a &lt;script class="diagram-code" type="text/plain"&gt;</p>`;
            return;
        }

        definitions.forEach(async definition => {
            try {
                const { svg: svg_light } = await mermaid.render(
                    (container?.getAttribute("name") || 'diagram-' + Math.random().toString(36).slice(2)) + "THEME_Light",
                    `%%{init: ${JSON.stringify(configLight)}}%% ${definition.textContent}`
                );
                const { svg: svg_dark } = await mermaid.render(
                    (container?.getAttribute("name") || 'diagram-' + Math.random().toString(36).slice(2)) + "THEME_Dark",
                    `%%{init: ${JSON.stringify(configDark)}}%% ${definition.textContent}`
                );

                container.innerHTML = svg_light;
                container.innerHTML += svg_dark;

                container.querySelectorAll('svg').forEach(svgElement => {
                    const g = /** @type {SVGGElement} */(svgElement.querySelector('g'));
                    const boundingBox = g.getBBox();

                    svgElement.setAttribute('viewBox', `${boundingBox.x} ${boundingBox.y} ${boundingBox.width} ${boundingBox.height}`);
                    svgElement.setAttribute('preserveAspectRatio', 'xMidYMid meet');
                    svgElement.style.width = '100%';
                    svgElement.style.height = 'auto';

                    if (svgElement.id.includes("THEME_Dark")) {
                        svgElement.classList.add("dark-only");
                    }
                    if (svgElement.id.includes("THEME_Light")) {
                        svgElement.classList.add("light-only");
                    }
                })

            } catch (err) {
                console.error('Fehler beim Mermaid-Rendern:', err);
            }
        })

    });
});