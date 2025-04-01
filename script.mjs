"use strict";

/******************************\
 *                            *
 *           Utils            *
 *                            *
\******************************/
//#region Utils

/**
 * 
 * @param {string} hex 
 * @param {number} [alpha=100] 
 * @returns 
 */
export function hexToRGB_CSS(hex, alpha = 100) {
    return `rgb(from ${hex} r g b / ${alpha}%)`;
}

//#endregion Utils