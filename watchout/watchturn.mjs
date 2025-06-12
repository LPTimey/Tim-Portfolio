"use strict";
// @ts-ignore
import { GLTFLoader } from "three/addons/loaders/GLTFLoader.js";
import { OrbitControls } from 'three/addons/controls/OrbitControls.js';
import { addLight, THREE, resize } from "../three_utils.mjs";

const loader = new GLTFLoader();
/** @type {HTMLCanvasElement} */
const infoCanvas = /** */(document.getElementById("InfoCanvas"));
const watchPath = `./assets/Design%20der%20Mensch%20Maschine%20Schnittstelle/WatchOut/TimUhr.glb`;
const watch = await loader.loadAsync(watchPath);
/** @type {THREE.Object3D} */
// @ts-ignore
const watchScene = watch.scene.clone();

export async function initInfoSpin() {
    const renderer = new THREE.WebGLRenderer(
        { alpha: true, canvas: infoCanvas, antialias: true }
    );

    const scene = new THREE.Scene();
    scene.add(watch.scene.clone());

    addLight(scene, {
        x: - 1, y: 2, z: 4
    });
    addLight(scene, {
        x: 1, y: -2, z: 1
    });

    const camera = new THREE.PerspectiveCamera(35);

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
    camera.position.set(0, 2, 5.75);
    controls.update();

    function animate() {

        requestAnimationFrame(animate);


        resize(renderer, camera);

        // required if controls.enableDamping or controls.autoRotate are set to true
        controls.update();

        renderer.render(scene, camera);

    }
    animate()
}