import { GLTFLoader } from "three/addons/loaders/GLTFLoader.js";
import { USDZLoader } from "three/addons/loaders/USDZLoader.js";
import { THREE, addLight, combine, easeIn, easeInOut, easeOut, easeOutCirc, hide, lerp, resize, show } from "../three_utils.mjs";
/** @import {Animation, AnimationState} from "../three_utils.mjs" */

/**
 * @type {Array<Animation|null>}
 */
let animation = null;
const loader = new GLTFLoader();

/** @type {HTMLCanvasElement|null} */
const heroCanvas = document.getElementById("HeroCanvas");
const watchPath = `./assets/Design%20der%20Mensch%20Maschine%20Schnittstelle/WatchOut/TimUhr.glb`;
const watch = await loader.loadAsync(watchPath);
/** @type {THREE.Object3D} */
const watchScene = watch.scene;
const phonePath = `./assets/Design%20der%20Mensch%20Maschine%20Schnittstelle/WatchOut/iphone.glb`;
const phone = await loader.loadAsync(phonePath);
/** @type {THREE.Object3D} */
const phoneScene = phone.scene;

/**
 * @param {THREE.Scene} scene 
 * @returns {Animation}
 */
function tiltWatch(scene) {
    const duration = 1000;
    let _internalState = {
        cur_duration: 0,
    };
    let beginState = {
        pos: new THREE.Vector3(0, -1.5, 3.5),
        rot: new THREE.Quaternion(-0.48, 0.48, 0.52, 0.52).normalize(),
        scale: new THREE.Vector3(1, 1, 1),
    }
    let endState = {
        pos: new THREE.Vector3(0, -1.5, 3.5),
        rot: new THREE.Quaternion(-0.4, 0.4, 0.6, 0.6).normalize(),
        scale: new THREE.Vector3(1, 1, 1),
    }
    let next = (deltaTime) => {
        _internalState.cur_duration += deltaTime;
        _internalState.cur_duration = Math.min(_internalState.cur_duration, duration);
        const percent = _internalState.cur_duration / duration;

        scene.position.lerpVectors(beginState.pos, endState.pos, easeOut(percent));
        scene.quaternion.slerpQuaternions(beginState.rot, endState.rot, lerp(easeIn(percent), easeOutCirc(percent), percent));
        scene.scale.lerpVectors(beginState.scale, endState.scale, easeInOut(percent));

        if (Math.abs(duration - _internalState.cur_duration) < 0.1) {
            return false
        }
        return true
    };
    return { _internalState, beginState, endState, next }
}


/**
 * 
 * @returns {Animation}
 */
function HeroAnimation() {
    let children = [hide(phoneScene), tiltWatch(watchScene), combine(hide(watchScene), show(phoneScene)),]
    let next = (delta) => {
        let cur = children.shift()
        if (cur?.next(delta)) { children.unshift(cur) }
    }
    return { next }
}

async function main() {
    const renderer = new THREE.WebGLRenderer(
        { alpha: true, canvas: heroCanvas, antialias: true }
    );
    const scene = new THREE.Scene();
    const cam = new THREE.PerspectiveCamera(35);
    cam.position.z = 6;
    scene.add(watchScene);
    watchScene.position.set(0, -1.5, 3.5);
    watchScene.quaternion.set(-0.48, 0.48, 0.52, 0.52);

    scene.add(phoneScene);

    phoneScene.position.set(0, 0, 0);

    addLight(scene, {
        x: - 1, y: 2, z: 4
    });
    addLight(scene, {
        x: 1, y: -2, z: 1
    });

    const axesHelper = new THREE.AxesHelper(5);
    // scene.add(axesHelper);

    const render = (time, lastTime) => {
        const deltaTime = time - (lastTime ?? 0);

        resize(renderer, cam);

        animation?.next(deltaTime)

        renderer.render(scene, cam);
        requestAnimationFrame((newTime) => render(newTime, time));
    }
    animation = hide(phoneScene);
    requestAnimationFrame(render);
}

await main();

document.getElementById("StartAnimation").addEventListener("click", () => {
    animation = HeroAnimation();
})