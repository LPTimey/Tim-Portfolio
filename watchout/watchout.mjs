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
 * @param {Quaternion} q 
 */
function quaternionToEuler(q) {
    var angles = {};

    // roll (x-axis rotation)
    var sinr_cosp = 2 * (q.w * q.x + q.y * q.z);
    var cosr_cosp = 1 - 2 * (q.x * q.x + q.y * q.y);
    angles.roll = Math.atan2(sinr_cosp, cosr_cosp);

    // pitch (y-axis rotation)
    var sinp = sqrt(1 + 2 * (q.w * q.y - q.x * q.z));
    var cosp = sqrt(1 - 2 * (q.w * q.y - q.x * q.z));
    angles.pitch = 2 * Math.atan2(sinp, cosp) - Math.PI / 2;

    // yaw (z-axis rotation)
    var siny_cosp = 2 * (q.w * q.z + q.x * q.y);
    var cosy_cosp = 1 - 2 * (q.y * q.y + q.z * q.z);
    angles.yaw = Math.atan2(siny_cosp, cosy_cosp);

    return {
        x: angles.roll,
        y: angles.pitch,
        z: angles.yaw
    }
}

/**
 * 
 * @returns {Animation}
 */
function wRotate(speed = 0.05) {
    return function (deltaTime) { watchScene.rotateY(speed * DOMTSToMicroSecs(deltaTime)); };
}
/**
 * 
 * @returns {Animation}
 */
function wTurnToButtons(speed = 0.05) {
    const target = { pos: new THREE.Vector3(-0.6), q: new THREE.Euler(0,1) };
    return function (deltaTime) {
        
        watchScene.translateX(watchScene.position.add(target.pos).x)
        watchScene.setRotationFromEuler(target.q);
        console.log(watchScene.position);
        watchState = NoOp();
    };
}

/**
 * 
 * @returns {Animation}
 */
function wTurnToCrown(speed = 0.05) {
    return function (deltaTime) { watchState = NoOp(); };
}
/**
 * 
 * @returns {Animation}
 */
function wTurnToFace(speed = 0.05) {
    return function (deltaTime) { watchState = NoOp(); };
}
/**
 * 
 * @returns {Animation}
 */
function wTurnToWristBandBack(speed = 0.05) {
    return function (deltaTime) { watchState = NoOp(); };
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
    watchState = wTurnToButtons();
})