import { initRenderer } from "./scroll_img/renderer.js";

/* =========================================================
    Globals
========================================================= */
//#region Globals
/** @type {NodeListOf<HTMLCanvasElement>} */
const canvasses = /** */(document.querySelectorAll(".scroll-img"));
//#endregion Globals

canvasses.forEach(async (canvas) => {
    await initRenderer(canvas)
})