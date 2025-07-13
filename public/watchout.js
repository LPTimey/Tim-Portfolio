"use strict";
// @ts-ignore
import { GLTFLoader } from "three/addons/loaders/GLTFLoader.js";
import { OrbitControls } from 'three/addons/controls/OrbitControls.js';

import * as THREE from "three";

const loader = new GLTFLoader();
/** @type {HTMLCanvasElement} */
const infoCanvas = /** */(document.getElementById("WatchInfoCanvas"));
// @ts-ignore
const watchPath = window.pathToRoot + `./assets/Design%20der%20Mensch%20Maschine%20Schnittstelle/WatchOut/TimUhr.glb`;
const watch = await loader.loadAsync(watchPath);
/** @type {THREE.Object3D} */
// @ts-ignore
const watchScene = watch.scene.clone();

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

/**
 * 
 * @param {THREE.Scene} scene 
 * @param {{x:number,y:number,z:number}} position 
 */
export function addLight(scene, position) {

    const color = 0xFFFFFF;
    const intensity = 2;
    const light = new THREE.DirectionalLight(color, intensity);
    // @ts-ignore
    light.position.set(position.x, position.y, position.z);
    scene.add(light);
}

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
 * @param {THREE.WebGLRenderer} renderer 
 * @param {THREE.PerspectiveCamera | THREE.OrthographicCamera} camera 
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

        // @ts-ignore
        if (camera.isPerspectiveCamera) {
            camera =/** @type {THREE.PerspectiveCamera} */(camera)
            camera.aspect = width / height;
        // @ts-ignore
        } else if (camera.isOrthographicCamera) {
            camera =/** @type {THREE.OrthographicCamera} */(camera)
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