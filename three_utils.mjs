// @ts-nocheck
"use strict";
import * as THREE from 'three';


export { THREE };

/**
 * 
 * @param {THREE.WebGLRenderer} renderer 
 * @returns true if renderer needs resizing
 */
export function rendererNeedsResize(renderer) {
    const canvas = renderer.domElement;
    const pixelRatio = window.devicePixelRatio;
    const width = Math.floor(canvas.clientWidth * pixelRatio);
    const height = Math.floor(canvas.clientHeight * pixelRatio);
    const needResize = canvas.width !== width || canvas.height !== height;
    return needResize;
}

/**
 * 
 * @param {THREE.Scene} scene 
 * @param {{x:number,y:number,z:number}} position 
 */
export function addLight(scene, position) {

    const color = 0xFFFFFF;
    const intensity = 2;
    const light = new THREE.DirectionalLight(color, intensity);
    light.position.set(position.x, position.y, position.z);
    scene.add(light);

}

/**
 * 
 * @param {THREE.Object3D} object3D
 */
export function addDebugHelpers(object3D, parent, options = {}) {
    const {
        colorBox = 0xff0000,
        colorPivot = 0x00ff00,
        pivotSize = 0.5,
    } = options;

    // Bounding Box anzeigen
    const boxHelper = new THREE.BoxHelper(object3D, colorBox);
    parent?.add(boxHelper);

    // Ursprungs-Marker (kleines Kreuz oder XYZ-Achse)
    const axesHelper = new THREE.AxesHelper(pivotSize);
    object3D.add(axesHelper);

    // Optional: Update BoxHelper automatisch bei Animationen
    // Das kannst du z.B. im Render-Loop machen:
    object3D.userData._debugBoxHelper = boxHelper;
}

/**
 * 
 * @param {THREE.WebGLRenderer} renderer 
 * @param {THREE.Camera} camera 
 * @returns if resize was necessary
 */
export function resize(renderer, camera) {

    if (rendererNeedsResize(renderer)) {
        const canvas = renderer.domElement;
        const pixelRatio = window.devicePixelRatio;
        const width = canvas.clientWidth;
        const height = canvas.clientHeight;

        renderer.setPixelRatio(pixelRatio);
        renderer.setSize(width, height, false);

        if (camera.isPerspectiveCamera) {
            camera.aspect = width / height;
        } else if (camera.isOrthographicCamera) {
            const frustumHeight = camera.top - camera.bottom;
            const aspect = width / height;
            const frustumWidth = frustumHeight * aspect;

            const dx = (camera.left + camera.right) / 2;
            const dy = (camera.top + camera.bottom) / 2;

            camera.left = dx - frustumWidth / 2;
            camera.right = dx + frustumWidth / 2;
            camera.top = dy + frustumHeight / 2;
            camera.bottom = dy - frustumHeight / 2;
        }

        camera.updateProjectionMatrix();
        return true;
    }
    return false;
}

/**
 * Linearly interpolates between two values.
 * @param {number} start - Start value.
 * @param {number} end - End value.
 * @param {number} t - Interpolation factor [0, 1].
 * @returns {number} Interpolated value.
 */
export function lerp(start, end, t) {
    return start + (end - start) * t;
}


/**
 * Creates a cubic Bézier easing function.
 * @param {number} x1 - X of control point 1.
 * @param {number} y1 - Y of control point 1.
 * @param {number} x2 - X of control point 2.
 * @param {number} y2 - Y of control point 2.
 * @returns {(t: number) => number} Easing function that maps t in [0,1] to eased t.
 */
function cubicBezier(x1, y1, x2, y2) {
    /**
     * Cubic Bézier formula.
     * @param {number} t - Parameter [0, 1].
     * @param {number} p0 - First point.
     * @param {number} p1 - First control point.
     * @param {number} p2 - Second control point.
     * @param {number} p3 - Last point.
     * @returns {number}
     */
    function bezier(t, p0, p1, p2, p3) {
        const u = 1 - t;
        return u ** 3 * p0 +
            3 * u ** 2 * t * p1 +
            3 * u * t ** 2 * p2 +
            t ** 3 * p3;
    }

    /**
     * Derivative of the Bézier curve with respect to t.
     * @param {number} t
     * @param {number} p1
     * @param {number} p2
     * @returns {number}
     */
    function derivative(t, p1, p2) {
        return 3 * (1 - t) ** 2 * (p1 - 0) +
            6 * (1 - t) * t * (p2 - p1) +
            3 * t ** 2 * (1 - p2);
    }

    /**
     * Solves for t given x using Newton-Raphson iteration.
     * @param {number} xTarget
     * @param {number} [epsilon=1e-5]
     * @returns {number}
     */
    function getTForX(xTarget, epsilon = 1e-5) {
        let t = xTarget;
        for (let i = 0; i < 8; i++) {
            const x = bezier(t, 0, x1, x2, 1);
            const dx = derivative(t, x1, x2);
            if (Math.abs(x - xTarget) < epsilon) return t;
            if (dx === 0) break;
            t -= (x - xTarget) / dx;
        }
        return t;
    }

    return function (t) {
        const solvedT = getTForX(t);
        return bezier(solvedT, 0, y1, y2, 1);
    };
}

/**
 * Returns a reversed version of an easing function.
 * @param {(t: number) => number} easeFn - Original easing function.
 * @returns {(t: number) => number}
 */
function reverseEase(easeFn) {
    return t => 1 - easeFn(1 - t);
}

/**
 * Returns a mirrored (ping-pong) version of an easing function.
 * @param {(t: number) => number} easeFn - Original easing function.
 * @returns {(t: number) => number}
 */
function mirrorEase(easeFn) {
    return t => {
        if (t < 0.5) return easeFn(t * 2) * 0.5;
        return (1 - easeFn((1 - t) * 2)) * 0.5 + 0.5;
    };
}

export const easeInOut = cubicBezier(0.42, 0, 0.58, 1);     // easeInOut
export const easeOut = cubicBezier(0, 0, 0.58, 1);          // easeOut
export const easeIn = cubicBezier(0.42, 0, 1, 1);           // easeIn
export const ease = cubicBezier(.25, .1, .25, 1);           // ease

export const reversed = reverseEase(easeInOut);
export const mirrored = mirrorEase(easeInOut);

export const overshoot = cubicBezier(.48, -0.04, .49, 1.1);


export const easeInCirc = cubicBezier(0.55, 0, 1, 0.45)
export const easeOutCirc = cubicBezier(0, 0.55, 0.45, 1)
export const easeInOutCirc = cubicBezier(0.85, 0, 0.15, 1)


function easeOutElastic(x) {
    const c4 = (2 * Math.PI) / 3;

    return x === 0
        ? 0
        : x === 1
            ? 1
            : Math.pow(2, -10 * x) * Math.sin((x * 10 - 0.75) * c4) + 1;
}