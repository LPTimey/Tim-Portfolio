"use strict";
import { hexToRGB_CSS } from "../script.mjs"
import { addLight, rendererNeedsResize, resize, THREE } from "../three_utils.mjs";

/** @type {HTMLCanvasElement|null} */
const WerdeCanvas = document.getElementById("WerdeCanvas");
class Vars {
    static get dark() { return getComputedStyle(WerdeCanvas).getPropertyValue("--dark"); }
    static get light() { return getComputedStyle(WerdeCanvas).getPropertyValue("--light"); }
    static get accent() { return getComputedStyle(WerdeCanvas).getPropertyValue("--accent"); }
    static get fg() { return getComputedStyle(WerdeCanvas).getPropertyValue("--fg"); }
    static get bg() { return getComputedStyle(WerdeCanvas).getPropertyValue("--bg"); }
    static hexToColor(hex){
        return new THREE.Color(Vars.fg);
    }
}

const geometry = new THREE.PlaneGeometry(100, 100); // Ein Quadrat mit 100x100
const material = new THREE.MeshBasicMaterial({ color: Vars.fg });
const square = new THREE.Mesh(geometry, material);


function renderWerdegang(time, renderer, scene, camera, lastTime) {
    const deltaTime = time - (lastTime ?? 0);
    resize(renderer, camera);
    square.rotation.z += 0.005 * deltaTime;
    material.color = new THREE.Color(Vars.fg)

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
