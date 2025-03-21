import * as THREE from 'three';
import { GLTFLoader } from 'three/addons/loaders/GLTFLoader.js';

/** @type {HTMLCanvasElement|null} */
const watchCanvas = document.getElementById("3dWatch");
const watchPath = `./assets/Design%20der%20Mensch%20Maschine%20Schnittstelle/WatchOut/TimUhr.glb`;
const loader = new GLTFLoader();
const watch = await loader.loadAsync(watchPath);
/** @type {THREE.Object3D} */
const watchScene = watch.scene;

function renderWatch(time, renderer, scene, camera) {
    time *= 0.001;
    // console.log("Hello");
    if (rendererNeedsResize(renderer)) {
        renderer.setSize(renderer.domElement.clientWidth, renderer.domElement.clientHeight, false);

        const canvas = renderer.domElement;
        camera.aspect = canvas.clientWidth / canvas.clientHeight;
        camera.updateProjectionMatrix();
    }

    watchScene.rotateY(0.01);

    renderer.render(scene, camera);
    requestAnimationFrame((time) => renderWatch(time, renderer, scene, camera));
}

/**
 * 
 * @param {THREE.WebGLRenderer} renderer 
 * @returns 
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