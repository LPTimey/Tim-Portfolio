"use strict";
import {hexToRGB_CSS} from "../script.mjs"

/** @type {HTMLCanvasElement|null} */
const WerdeCanvas = document.getElementById("WerdeCanvas");

function draw() {
    if (!WerdeCanvas.getContext) {
        console.error("ahhhh");
        return;
    }
    const ctx = WerdeCanvas.getContext("2d");
    if (ctx == null) {
        console.error("ahhhh");
        return;
    }

    // Get the DPR and size of the canvas
    const dpr = window.devicePixelRatio;
    const rect = WerdeCanvas.getBoundingClientRect();

    // Set the "actual" size of the canvas
    WerdeCanvas.width = rect.width * dpr;
    WerdeCanvas.height = rect.height * dpr;

    // Scale the context to ensure correct drawing operations
    ctx.scale(dpr, dpr);

    // Set the "drawn" size of the canvas
    WerdeCanvas.style.width = `${rect.width}px`;
    WerdeCanvas.style.height = `${rect.height}px`;


    let dark = getComputedStyle(WerdeCanvas).getPropertyValue("--dark");
    let light = getComputedStyle(WerdeCanvas).getPropertyValue("--light");
    let accent_light = getComputedStyle(WerdeCanvas).getPropertyValue("--accent-light");
    let accent = getComputedStyle(WerdeCanvas).getPropertyValue("--accent");
    let fg = getComputedStyle(WerdeCanvas).getPropertyValue("--fg");
    let bg = getComputedStyle(WerdeCanvas).getPropertyValue("--bg");

    ctx.fillStyle = hexToRGB_CSS(fg);
    ctx.fillRect(10, 10, 500, 500);

    ctx.fillStyle = hexToRGB_CSS(accent_light,50);
    ctx.fillRect(300, 300, 500, 500);
    requestAnimationFrame(draw)
}
window.addEventListener("load", draw);