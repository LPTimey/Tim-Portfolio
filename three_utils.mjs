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


/** @typedef {"start"|"center"|"end"} Alignment */

/**
 * 
 * @param {THREE.Mesh} mesh 
 * @param {{
 *     xAlign?: Alignment,
 *     yAlign?: Alignment,
 *     zAlign?: Alignment,
 * }} align 
 */
export function alignOrigin(mesh, align) {
    const {
        xAlign = 'center',
        yAlign = 'center',
        zAlign = 'center',
    } = align;

    mesh.geometry.computeBoundingBox();
    const box = mesh.geometry.boundingBox;

    const width = box.max.x - box.min.x;
    const height = box.max.y - box.min.y;
    const depth = box.max.z - box.min.z;

    let x = 0, y = 0, z = 0;

    if (xAlign === 'start') x = box.min.x;
    else if (xAlign === 'center') x = box.min.x + width / 2;
    else if (xAlign === 'end') x = box.max.x;

    if (yAlign === 'start') y = box.min.y;
    else if (yAlign === 'center') y = box.min.y + height / 2;
    else if (yAlign === 'end') y = box.max.y;

    if (zAlign === 'start') z = box.min.z;
    else if (zAlign === 'center') z = box.min.z + depth / 2;
    else if (zAlign === 'end') z = box.max.z;

    const currentPivot = mesh.position.clone();
    const offset = new THREE.Vector3().subVectors(currentPivot, new THREE.Vector3(x, y, z));
    mesh.geometry.translate(offset.x, offset.y, offset.z);
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
                scene.scale.set(0, 0, 0)
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
                scene.scale.set(1, 1, 1)
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


export function lerp(start, end, t) {
    return (start + (end - start) * t);
}

export function square(x) {
    return x * x;
}

export function cube(x) {
    return x * x * x;
}

export function pow(x) {
    return Math.pow(x, x);
}

export function flip(x) {
    return 1 - x;
}

export function linear(t) {
    return t
}

export function easeOut(t) {
    return flip(square(flip(t)))
}

export function easeIn(t) {
    return square(t)
}

export function easeInOut(t) {
    return lerp(easeIn(t), easeOut(t), t);
}

export function easeInCubic(t) {
    return Math.pow(t, 3);
}
export function easeOutCubic(t) {
    return flip(easeInCubic(flip(t)));
}
export function easeInOutCubic(t) {
    return lerp(easeInCubic(t), easeOutCubic(t), t);
}

export function easeInCirc(t) {
    return 1 - Math.sqrt(1 - Math.pow(t, 2));
}
export function easeOutCirc(t) {
    return Math.sqrt(1 - Math.pow(t - 1, 2));
}
export function easeInOutCirc(t) {
    return lerp(easeInCirc(t), easeOutCirc(t), t);
}
export function easeInOutQuad(x) {
    return x < 0.5 ? 2 * x * x : 1 - Math.pow(-2 * x + 2, 2) / 2;
}