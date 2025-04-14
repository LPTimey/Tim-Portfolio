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
        const pixelRatio = window.devicePixelRatio;
        renderer.setPixelRatio(pixelRatio);
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