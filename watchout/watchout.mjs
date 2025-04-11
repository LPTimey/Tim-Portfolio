import { GLTFLoader } from "three/addons/loaders/GLTFLoader.js";
import { THREE, add, addLight, combine, easeIn, easeInOut, easeOut, easeOutCirc, hide, lerp, remove, resize, show } from "../three_utils.mjs";
/** @import {Animation, AnimationState} from "../three_utils.mjs" */

/**
 * @type {Animation|null}
 */
let animation = null;
const loader = new GLTFLoader();

/** @type {HTMLCanvasElement|null} */
const heroCanvas = document.getElementById("HeroCanvas");
const watchPath = `./assets/Design%20der%20Mensch%20Maschine%20Schnittstelle/WatchOut/TimUhr.glb`;
const watch = await loader.loadAsync(watchPath);
/** @type {THREE.Object3D} */
const watchScene = watch.scene;
// const phonePath = `./assets/Design%20der%20Mensch%20Maschine%20Schnittstelle/WatchOut/Iphone/iphone spin.gltf`;
const phonePath = `./assets/Design%20der%20Mensch%20Maschine%20Schnittstelle/WatchOut/iphone.glb`;
const phone = await loader.loadAsync(phonePath);
/** @type {THREE.Object3D} */
const phoneScene = phone.scene;
const texPathPrefix = "../assets/Design der Mensch Maschine Schnittstelle/WatchOut/Watch out Exports/"
const phoneTextures = [
    // 'Artboard – 1.png',
    // 'Call List Expanded.png',
    'History  – 1 Log.png',
    'History  – 2 Err.png',
    'History  – 2 Warn.png',
    'Map – 1.png',
    'Map – 2.png',
    // 'Map Overlay – 1 – Event.png',
    // 'Map Overlay – 1 – Warn.png',
    // 'Map Overlay – 2 – Gunther.png',
    // 'Map Overlay – 2 – Joe.png',
    // 'Message Overlay – new.png',
    // 'Message Overlay seen.png',
    'Recent Events  – log.png',
    'Recent Events – 0.png',
    'Recent Events – error.png',
    'Recent Events – warn.png',
    'Settings – 1.png',
];
const phones = []

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
    const next = function (deltaTime) {
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

//TODO
function slidePhones(primary, phones) {
    const duration = 1000;
    let _internalState = {
        cur_duration: 0,
    };
    let beginState = {
        pos: new THREE.Vector3(),
        rot: new THREE.Quaternion().normalize(),
        scale: new THREE.Vector3(1, 1, 1),
    }
    let endState = {
        pos: new THREE.Vector3(),
        rot: new THREE.Quaternion().normalize(),
        scale: new THREE.Vector3(1, 1, 1),
    }
    const next = function (deltaTime) { };
    return { next }
}

/**
 * 
 * @param {THREE.Scene} scene 
 * @returns {Animation}
 */
function HeroAnimation(scene) {
    let children = [remove(scene, phoneScene, ...phones), tiltWatch(watchScene), combine(remove(scene, watchScene), add(scene, phoneScene, ...phones)),]
    let next = function (delta) {
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
    phoneTextures.forEach(function (path, i) {
        let newScene = phoneScene.clone();

        newScene.traverse(child => {
            if (child.material && child.material.name === 'iphone-x-screenshot') {

                // Klone das Material und weise es dem Kind zu
                let clonedMaterial = child.material.clone();
                child.material = clonedMaterial;

                console.log("b", clonedMaterial);

                // Lade die Textur und weise sie zu
                const textureLoader = new THREE.TextureLoader();
                const texture = textureLoader.load(texPathPrefix + path, () => {
                    texture.flipY = false;
                    texture.minFilter = THREE.LinearFilter;  // Deaktiviere Mipmaps
                    texture.magFilter = THREE.LinearFilter;  // Mipmap-Filter anpassen
                    texture.needsUpdate = true;
                });

                // Weise der Textur der geklonten Material-Map eine neue Textur zu
                clonedMaterial.map = texture;

                clonedMaterial.map.needsUpdate = true;
                clonedMaterial.needsUpdate = true;

            }
        });
        phones.push(newScene);
        scene.add(newScene);
        newScene.translateX((i - 5.5) * 1.75);
    })
    phoneScene.position.set(0, 0, 1);

    addLight(scene, {
        x: - 1, y: 2, z: 4
    });
    addLight(scene, {
        x: 1, y: -2, z: 1
    });

    const axesHelper = new THREE.AxesHelper(5);
    // scene.add(axesHelper);

    const render = function (time, lastTime) {
        const deltaTime = time - (lastTime ?? 0);

        resize(renderer, cam);

        animation?.next(deltaTime);

        renderer.render(scene, cam);
        requestAnimationFrame((newTime) => render(newTime, time));
    }
    animation = remove(scene, phoneScene, ...phones);
    requestAnimationFrame(render);

    document.getElementById("StartAnimation").addEventListener("click", function () {
        animation = HeroAnimation(scene);
    })
}

await main();