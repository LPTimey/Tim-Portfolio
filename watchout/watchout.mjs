import * as THREE from 'three';
import { GLTFLoader } from 'three/addons/loaders/GLTFLoader.js';

/** @type {HTMLCanvasElement|null} */
const watchCanvas = document.getElementById("3dWatch");
const watchPath = `./assets/Design%20der%20Mensch%20Maschine%20Schnittstelle/WatchOut/TimUhr.glb`;
const loader = new GLTFLoader();
const watch = await loader.loadAsync(watchPath);
/** @type {THREE.Object3D} */
const watchScene = watch.scene;
/** @type {Animation} */
var watchState = wRotate();

/**
 * @typedef {(deltaTime:DOMHighResTimeStamp)=>void} Animation
 */

/**
 * 
 * @returns {Animation}
 */
function wRotate(speed = 0.05) {
    return function (deltaTime) { watchScene.rotateY(speed * DOMTSToMicroSecs(deltaTime)); };
}
/**
 * 
 * @param {number} [duration=1000] in milliseconds
 * @returns {Animation}
 */
function wTurnToButtons(duration = 1000) {
    let timeLeft = duration;

    const quaternion = new THREE.Quaternion();
    quaternion.setFromEuler(new THREE.Euler(0, 1, 0));
    const target = { pos: new THREE.Vector3(-0.6), rot: quaternion };

    const minFloatDelta = 0.1;

    return function (deltaTime) {
        timeLeft = timeLeft - deltaTime

        if (watchScene.position.distanceTo(target.pos) < minFloatDelta
            && watchScene.quaternion.equals(target.rot)) {
            console.log("done")
            watchState = NoOp();
            return;
        }

        watchScene.position.lerp(target.pos, (duration - timeLeft) / duration)

        watchScene.quaternion.slerp(target.rot, (duration - timeLeft) / duration)
    };
}

/**
 * 
 * @returns {Animation}
 */
function wTurnToCrown(duration = 1000) {
    let timeLeft = duration;

    const quaternion = new THREE.Quaternion();
    quaternion.setFromEuler(new THREE.Euler(0, 4.9, 0));
    const target = { pos: new THREE.Vector3(0.6), rot: quaternion };

    const minFloatDelta = 0.1;

    return function (deltaTime) {
        timeLeft = timeLeft - deltaTime

        if (watchScene.position.distanceTo(target.pos) < minFloatDelta
            && watchScene.quaternion.equals(target.rot)) {
            console.log("done")
            watchState = NoOp();
            return;
        }

        watchScene.position.lerp(target.pos, (duration - timeLeft) / duration)

        watchScene.quaternion.slerp(target.rot, (duration - timeLeft) / duration)
    };
}
/**
 * 
 * @returns {Animation}
 */
function wTurnToFace(duration = 1000) {
    let timeLeft = duration;

    const quaternion = new THREE.Quaternion();
    quaternion.setFromEuler(new THREE.Euler(0, 0, 0));
    const target = { pos: new THREE.Vector3(0, 0, 0), rot: quaternion };

    const minFloatDelta = 0.1;

    return function (deltaTime) {
        timeLeft = timeLeft - deltaTime

        if (watchScene.position.distanceTo(target.pos) < minFloatDelta
            && watchScene.quaternion.equals(target.rot)) {
            console.log("done")
            watchState = NoOp();
            return;
        }

        watchScene.position.lerp(target.pos, (duration - timeLeft) / duration)

        watchScene.quaternion.slerp(target.rot, (duration - timeLeft) / duration)
    };
}
/**
 * 
 * @returns {Animation}
 */
function wTurnToWristBandBack(duration = 1000) {
    let timeLeft = duration;

    const quaternion = new THREE.Quaternion();
    quaternion.setFromEuler(new THREE.Euler(0, 3.25, 0));
    const target = { pos: new THREE.Vector3(0, 0, 0), rot: quaternion };

    const minFloatDelta = 0.1;

    return function (deltaTime) {
        timeLeft = timeLeft - deltaTime

        if (watchScene.position.distanceTo(target.pos) < minFloatDelta
            && watchScene.quaternion.equals(target.rot)) {
            console.log("done")
            watchState = NoOp();
            return;
        }

        watchScene.position.lerp(target.pos, (duration - timeLeft) / duration)

        watchScene.quaternion.slerp(target.rot, (duration - timeLeft) / duration)
    };
}
/**
 * 
 * @returns {Animation}
 */
function NoOp() {
    return function (deltaTime) { };
}

function DOMTSToMicroSecs(time) {
    return time / 100;
}
function DOMTSToSecs(time) {
    return time * 100;
}

/**
 * 
 * @param {DOMHighResTimeStamp} time in ms
 * @param {THREE.WebGLRenderer} renderer 
 * @param {THREE.Scene} scene 
 * @param {THREE.Camera} camera 
 * @param {DOMHighResTimeStamp?} lastTime in ms
 */
function renderWatch(time, renderer, scene, camera, lastTime) {
    const deltaTime = time - (lastTime ?? 0);
    // console.log("Hello");
    if (rendererNeedsResize(renderer)) {
        renderer.setSize(renderer.domElement.clientWidth, renderer.domElement.clientHeight, false);

        const canvas = renderer.domElement;
        camera.aspect = canvas.clientWidth / canvas.clientHeight;
        camera.updateProjectionMatrix();
    }

    watchState(deltaTime);

    renderer.render(scene, camera);
    requestAnimationFrame((newTime) => renderWatch(newTime, renderer, scene, camera, time));
}

/**
 * 
 * @param {THREE.WebGLRenderer} renderer 
 * @returns true if renderer needs resizing
 */
function rendererNeedsResize(renderer) {
    return renderer.domElement.width !== renderer.domElement.clientWidth
        || renderer.domElement.height !== renderer.domElement.clientHeight;
}

/**
 * 
 * @param {THREE.Scene} scene 
 * @param {{x:number,y:number,z:number}} position 
 */
function addLight(scene, position) {

    const color = 0xFFFFFF;
    const intensity = 2;
    const light = new THREE.DirectionalLight(color, intensity);
    light.position.set(position.x, position.y, position.z);
    scene.add(light);

}

/**
 * start rendering
*/
async function initWatch() {
    const renderer = new THREE.WebGLRenderer({ antialias: true, canvas: watchCanvas, alpha: true });
    const scene = new THREE.Scene();

    const fov = 35;
    const aspect = 2; // the canvas default
    const near = 0.1;
    const far = 2000;
    const camera = new THREE.PerspectiveCamera(fov, aspect, near, far);

    camera.position.z = 6;
    camera.position.x = 0;
    camera.position.y = 0;

    addLight(scene, {
        x: - 1, y: 2, z: 4
    });
    addLight(scene, {
        x: 1, y: -2, z: 1
    });

    scene.add(watchScene);

    requestAnimationFrame((time) => renderWatch(time, renderer, scene, camera));
}

function init() {
    initWatch();
}

init();

const next_Button = document.getElementById("next")
next_Button.addEventListener("click", () => {
    watchState = wTurnToWristBandBack();
})