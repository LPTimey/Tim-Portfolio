"use strict";
import { hexToRGB_CSS } from "../script.mjs"
import { addLight, rendererNeedsResize, resize, THREE } from "../three_utils.mjs";

/** @type {HTMLCanvasElement|null} */
const WerdeCanvas = document.getElementById("WerdeCanvas");
let dark = getComputedStyle(WerdeCanvas).getPropertyValue("--dark");
let light = getComputedStyle(WerdeCanvas).getPropertyValue("--light");
let accent = getComputedStyle(WerdeCanvas).getPropertyValue("--accent");
let fg = getComputedStyle(WerdeCanvas).getPropertyValue("--fg");
let bg = getComputedStyle(WerdeCanvas).getPropertyValue("--bg");

const geometry = new THREE.PlaneGeometry(100, 100); // Ein Quadrat mit 100x100
const material = new THREE.MeshBasicMaterial({ color: accent });
const square = new THREE.Mesh(geometry, material);


function renderWerdegang(time, renderer, scene, camera, lastTime) {
    const deltaTime = time - (lastTime ?? 0);
    resize(renderer, camera);
    square.rotation.z += 0.005 * deltaTime;

    renderer.render(scene, camera);
    requestAnimationFrame(newTime => renderWerdegang(newTime, renderer, scene, camera, time))
}

function init() {
    const renderer = new THREE.WebGLRenderer({ antialias: true, canvas: WerdeCanvas, alpha: true });
    const scene = new THREE.Scene();

    const camera = new THREE.OrthographicCamera(
        -window.innerWidth / 2, window.innerWidth / 2,   // left, right
        window.innerHeight / 2, -window.innerHeight / 2, // top, bottom
        1, 1000  // near, far
    );
    camera.position.z = 10; // Die Kamera befindet sich auf der Z-Achse

    scene.add(square); // Füge das Quadrat der Szene hinzu

    requestAnimationFrame(time => renderWerdegang(time, renderer, scene, camera));
}

init();
