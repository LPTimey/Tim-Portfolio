"use strict";
import * as THREE from 'three';
import * as THREE_ADDONS from 'three/addons/Addons.js';

export { THREE, THREE_ADDONS };

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


/** @typedef {"start"|"center"|"end"|number} Alignment */

/**
 * 
 * @param {THREE.Mesh|THREE.Group} obj 
 * @param {{
 *     xAlign?: Alignment,
 *     yAlign?: Alignment,
 *     zAlign?: Alignment,
 * }} align 
 */
export function alignOrigin(obj, align) {
    if (obj.isMesh) {
        alignOriginMesh(obj, align);
    } else if (obj.isGroup) {
        alignOriginGroup(obj, align);
    }
}
/**
 * 
 * @param {THREE.Mesh} mesh 
 * @param {{
 *     xAlign?: Alignment,
 *     yAlign?: Alignment,
 *     zAlign?: Alignment,
 * }} align 
 */
function alignOriginMesh(mesh, align) {
    mesh.geometry.computeBoundingBox();
    const box = mesh.geometry.boundingBox;

    const offset = alignCalcs(box, mesh.position, align);
    mesh.geometry.translate(offset.x, offset.y, offset.z);
}
/**
 * 
 * @param {THREE.Group} group 
 * @param {{
 *     xAlign?: Alignment,
 *     yAlign?: Alignment,
 *     zAlign?: Alignment,
 * }} align 
 */
function alignOriginGroup(group, align) {
    let box = new THREE.Box3().setFromObject(group);

    const offset = alignCalcs(box, group.position, align);

    group.children.forEach(child => child.translateX(offset.x));
    group.children.forEach(child => child.translateY(offset.y));
    group.children.forEach(child => child.translateZ(offset.z));
}


/**
 * 
 * @param {THREE.Box3} box 
 * @param {THREE.Vector3} position 
 * @param {{
 *     xAlign?: Alignment,
 *     yAlign?: Alignment,
 *     zAlign?: Alignment,
 * }} align 
 * @returns 
 */
function alignCalcs(box, position, align) {
    const {
        xAlign = 'center',
        yAlign = 'center',
        zAlign = 'center',
    } = align;

    const width = box.max.x - box.min.x;
    const height = box.max.y - box.min.y;
    const depth = box.max.z - box.min.z;

    let x = 0, y = 0, z = 0;

    if (xAlign === 'start') x = box.min.x;
    else if (xAlign === 'center') x = box.min.x + width / 2;
    else if (xAlign === 'end') x = box.max.x;
    else if (typeof xAlign === 'number') x = box.min.x + width * xAlign;

    if (yAlign === 'start') y = box.min.y;
    else if (yAlign === 'center') y = box.min.y + height / 2;
    else if (yAlign === 'end') y = box.max.y;
    else if (typeof yAlign === 'number') y = box.min.y + height * yAlign;

    if (zAlign === 'start') z = box.min.z;
    else if (zAlign === 'center') z = box.min.z + depth / 2;
    else if (zAlign === 'end') z = box.max.z;
    else if (typeof zAlign === 'number') z = box.min.z + depth * zAlign;

    const offset = new THREE.Vector3().subVectors(position.clone(), new THREE.Vector3(x, y, z));
    return offset;
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
 * @typedef Animation
 * @prop {any} [_internalState?]
 * @prop {AnimationState} [beginState?]
 * @prop {AnimationState} [endState?]
 * @prop {(deltaTime:DOMHighResTimeStamp)=>bool|undefined} next returns false or undefined when done and true if not
 */

/**
 * @typedef AnimationState
 * @prop {THREE.Vector3} pos
 * @prop {THREE.Quaternion} rot
 * @prop {THREE.Vector3} scale
 */

/**
 * 
 * @param {DOMHighResTimeStamp} duration 
 * @returns {Animation}
 */
export function wait(duration) {
    const _internalState = {
        cur_duration: 0,
    }
    return {
        next: function (deltaTime) {
            _internalState.cur_duration += deltaTime;
            return _internalState.cur_duration < duration;
        }
    }
}

/**
 * 
 * @param  {...THREE.Scene} scenes 
 * @returns {Animation}
 */
export function hide(...scenes) {
    return {
        next: function () {
            for (const scene of scenes) {
                scene.userData.scale_before_hide = scene.scale.clone();
                scene.scale.set(0, 0, 0);
            }
        }
    }
}
/**
 * 
 * @param  {...THREE.Scene} scenes 
 * @returns {Animation}
 */
export function show(...scenes) {
    return {
        next: function () {
            for (const scene of scenes) {
                if (scene.userData.scale_before_hide) {
                    scene.scale.set(scene.userData.scale_before_hide.x, scene.userData.scale_before_hide.y, scene.userData.scale_before_hide.z);
                }
                else { scene.scale.set(1, 1, 1); }
            }
        }
    }
}

/**
 * 
 * @param {THREE.Scene} scene 
 * @param  {...THREE.Scene} scenes 
 * @returns {Animation}
 */
export function remove(scene, ...scenes) {
    return {
        next: function () {
            for (const nextScene of scenes) {
                scene.remove(nextScene);
            }
        }
    }
}

/**
 * @typedef FadeType
 * @prop {{start: 0, end: 1}} In
 * @prop {{start: 1, end: 0}} Out
 */

/**
 * @template {keyof FadeType} K
 * @param {K} type 
 * @param {THREE.Scene} scene 
 * @param {number} [duration=1000] 
 * @returns {Animation}
 * @returns {{next: FadeType[K]}}
 */
export function fade(type, scene, duration = 1000) {
    const start = type === "In" ? 0 : 1;
    const end = type === "In" ? 1 : 0;
    let opacity = start;
    let cur_duration = 0;

    setOriginalAlpha(scene);

    return {
        next: function (deltaTime) {
            cur_duration += deltaTime;
            cur_duration = Math.min(cur_duration, duration);
            const percent = cur_duration / duration;

            opacity = lerp(start, end, percent);
            scene.traverse((child) => {
                if (child.isMesh && child.material) {
                    /** @type {THREE.Material[]} */
                    const materials = Array.isArray(child.material) ? child.material : [child.material];

                    materials.filter(mat => mat.userData !== undefined).forEach((mat) => {
                        mat.transparent = true;
                        mat.opacity = mat.userData.originalOpacity * opacity; // z.B. opacity = 0.5
                    });
                }
            });

            if (Math.abs(duration - cur_duration) < 0.1) {
                return false
            }
            return true
        }
    }
}
export function setOriginalAlpha(scene) {
    scene.traverse((child) => {
        if (child.isMesh && child.material) {
            const materials = Array.isArray(child.material) ? child.material : [child.material];

            materials.forEach((mat) => {
                if (mat.userData.originalOpacity === undefined) {
                    mat.userData.originalOpacity = mat.opacity;
                }
            });
        }
    });
}

/**
 * 
 * @param {THREE.Scene} scene 
 * @param  {...THREE.Scene} scenes 
 * @returns {Animation}
 */
export function add(scene, ...scenes) {
    return {
        next: function () {
            for (const nextScene of scenes) {
                scene.add(nextScene);
            }
        }
    }
}

/**
 * 
 * @param  {...Animation} animations 
 * @returns {Animation}
 */
export function combine(...animations) {
    return {
        // _internalState:,
        next: function (deltaTime) {
            let results = [];
            for (let i = 0; i < animations.length; i++) {
                results[i] = animations[i]?.next(deltaTime);
            }
            if (results.includes(true)) {
                return true;
            }
            return false;
        }
    }
}

/**
 * 
 * @param {THREE.Scene} scene 
 * @param  {(scene:THREE.Scene)=>Animation} begin 
 * @param  {(scene:THREE.Scene)=>Animation} end 
 * @param {Object} [options?]
 * @param {number} [options.duration=1000] 
 * @param {(t: any) => any} [options.pos_easing=easeInOut] 
 * @param {(t: any) => any} [options.rot_easing=easeInOut] 
 * @param {(t: any) => any} [options.scale_easing=easeInOut] 
 * @returns {Animation}
 */
export function interpolate(scene, begin, end, { duration = 1000, pos_easing = easeInOut, rot_easing = easeInOut, scale_easing = easeInOut } = {}) {
    const _internalState = {
        cur_duration: 0,
    };
    const beginState = begin(scene).endState;
    const endState = end(scene).beginState;
    const next = function (deltaTime) {
        _internalState.cur_duration += deltaTime;
        _internalState.cur_duration = Math.min(_internalState.cur_duration, duration);
        const percent = _internalState.cur_duration / duration;

        scene.position.lerpVectors(beginState.pos, endState.pos, pos_easing(percent));
        scene.quaternion.slerpQuaternions(beginState.rot, endState.rot, rot_easing(percent));
        scene.scale.lerpVectors(beginState.scale, endState.scale, scale_easing(percent));

        if (Math.abs(duration - _internalState.cur_duration) < 0.1) {
            return false
        }
        return true
    };
    return { _internalState, beginState, endState, next }
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