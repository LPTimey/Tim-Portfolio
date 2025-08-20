"use strict";
// @ts-ignore
import { GLTFLoader } from "three/addons/loaders/GLTFLoader.js";
import { OrbitControls } from 'three/addons/controls/OrbitControls.js';

import * as THREE from "three"

import { addLight, rendererNeedsResize, resize } from "./threejs_utils.js"

const loader = new GLTFLoader();
/** @type {HTMLCanvasElement} */
const infoCanvas = /** */(document.getElementById("ErgomoteInfoCanvas"));
// @ts-ignore
const motePath = window.pathToRoot + `./assets/Ergomote/ergo-assembly.glb`;
const ergomote = await loader.loadAsync(motePath);
/** @type {THREE.Object3D} */
// @ts-ignore
const watchScene = ergomote.scene.clone();

async function main() {
    // startButton.innerHTML = play_icon;
    // startButton.setAttribute("data-state", "play");
    // startButton.addEventListener("click", initHero);
    // startButton.style.visibility = "hidden";
    initInfoSpin();
}

main()



export async function initInfoSpin() {
    const renderer = new THREE.WebGLRenderer(
        { alpha: true, canvas: infoCanvas, antialias: true }
    );

    const scene = new THREE.Scene();
    scene.add(ergomote.scene.clone());

    addLight(scene, {
        x: - 1, y: 2, z: 4
    }, 0.75);
    addLight(scene, {
        x: 1, y: -2, z: 1
    }, 0.75);
    addLight(scene, {
        x: -1, y: 2, z: 1
    }, 0.75);
    addLight(scene, {
        x: - 1, y: 2, z: -4
    }, 0.75);
    addLight(scene, {
        x: 1, y: -2, z: -1
    }, 0.75);
    addLight(scene, {
        x: -1, y: 2, z: -1
    }, 0.75);

    const camera = new THREE.PerspectiveCamera(25);

    const controls = new OrbitControls(camera, renderer.domElement);
    controls.autoRotateSpeed = 0.75;
    controls.autoRotate = true;
    controls.rotateSpeed = 0.75;
    controls.enableZoom = false;
    controls.enablePan = false;

    const axesHelper = new THREE.AxesHelper(5);
    // @ts-ignore
    if (infoCanvas.hasAttribute("axis")) { scene.add(axesHelper); }

    //controls.update() must be called after any manual changes to the camera's transform
    // @ts-ignore
    camera.position.set(0, 2.25, 7);
    camera.lookAt(0,0,0);
    controls.update();
    resize(renderer, camera);
    controls.update();

    function animate() {
        requestAnimationFrame(animate);

        if (rendererNeedsResize(renderer)) {
            resize(renderer, camera);
        }

        // required if controls.enableDamping or controls.autoRotate are set to true
        controls.update();

        renderer.render(scene, camera);
    }

    animate()
}
