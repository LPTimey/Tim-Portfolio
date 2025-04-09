import * as THREE from 'three';

export { THREE };
/**
 * 
 * @param {THREE.WebGLRenderer} renderer 
 * @returns true if renderer needs resizing
 */
export function rendererNeedsResize(renderer) {
    return renderer.domElement.width !== renderer.domElement.clientWidth
        || renderer.domElement.height !== renderer.domElement.clientHeight;
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
 * @param {THREE.WebGLRenderer} renderer 
 * @param {THREE.Camera} camera 
 * @returns if resize was necessary
 */
export function resize(renderer, camera) {
    if (rendererNeedsResize(renderer)) {
        renderer.setSize(renderer.domElement.clientWidth, renderer.domElement.clientHeight, false);

        const canvas = renderer.domElement;
        camera.aspect = canvas.clientWidth / canvas.clientHeight;
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
 * @prop {(deltaTime:DOMHighResTimeStamp)=>bool} next
 */

/**
 * @typedef AnimationState
 * @prop {THREE.Vector3} pos
 * @prop {THREE.Quaternion} rot
 * @prop {THREE.Vector3} scale
 */

/**
 * 
 * @param  {...THREE.Scene} scenes 
 * @returns {Animation}
 */
export function hide(...scenes) {
    return {
        next: () => {
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
        next: () => {
            for (const scene of scenes) {
                scene.scale.set(1, 1, 1)
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
        next: (deltaTime) => {
            let results = [];
            for (let i = 0; i < animations.length; i++) {
                results[i] = animations[i].next(deltaTime);
            }
        }
    }
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

export function pow(x){
    return Math.pow(x,x);
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