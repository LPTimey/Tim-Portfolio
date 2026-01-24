import { initWebGPU } from "./wgpu.js"
import { initWebGL } from "./webgl.js"
import { supportsWebGPU } from "../../script.js"

/**
 * 
 * @param {HTMLCanvasElement} canvas 
 * @returns 
 */
export async function initRenderer(canvas) {
    if (supportsWebGPU()) {
        return initWebGPU(canvas)
    } else {
        return initWebGL(canvas)
    }
}
