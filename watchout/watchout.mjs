import { GLTFLoader } from "three/addons/loaders/GLTFLoader.js";
import { THREE, add, addLight, combine, easeIn, easeInCirc, easeInOut, easeOut, easeOutCirc, hide, interpolate, lerp, remove, resize, show, wait } from "../three_utils.mjs";
import { pause_icon, play_icon } from "../script.mjs";
/** @import {Animation, AnimationState} from "../three_utils.mjs" */

const startButton = document.getElementById("StartAnimation");

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
const texPathPrefix = "./assets/Design der Mensch Maschine Schnittstelle/WatchOut/Watch out Exports/"
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

function initAnimation(scene, watch, phone, ...phones) {
    watch.position.set(-4.4, 0, -4.75);
    watch.quaternion.setFromEuler(new THREE.Euler(0, 0.6, 0));

    // phone.position.set(0, 0, 0.25);

    // phones.forEach((phone, i) =>
    //     phone.translateX((i - 6.5) * 5)
    // )

    remove(scene, phone, ...phones).next(0);

    animation = null;
}

function fadeOutBGImg() {
    return {
        next: function () { }
    }
}

/**
 * @param {THREE.Scene} scene 
 * @returns {Animation}
 */
function startWatch(scene) {
    const duration = 1500;
    const _internalState = {
        cur_duration: 0,
    };
    const beginState = {
        pos: new THREE.Vector3(-4.4, 0, -4.75),
        rot: new THREE.Quaternion().setFromEuler(new THREE.Euler(0, 0.6, 0)),
        scale: new THREE.Vector3(1, 1, 1),
    }
    let endState = {
        pos: new THREE.Vector3(),
        rot: new THREE.Quaternion().normalize(),
        scale: new THREE.Vector3(1, 1, 1),
    }
    endState = beginState;
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

/**
 * @param {THREE.Scene} scene 
 * @returns {Animation}
 */
function tiltWatch(scene) {
    const duration = 1500;
    const _internalState = {
        cur_duration: 0,
    };
    const beginState = {
        pos: new THREE.Vector3(0, -1.5, 3.5),
        rot: new THREE.Quaternion(-0.48, 0.48, 0.52, 0.52).normalize(),
        scale: new THREE.Vector3(1, 1, 1),
    }
    const endState = {
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

/**
 * 
 * @param {THREE.Scene} scene 
 */
function moveWatchAway(scene) {
    const duration = 1000;
    const _internalState = {
        cur_duration: 0,
    };
    let beginState = null;
    const endState = {
        pos: new THREE.Vector3(0, -2.5, 5),
        rot: new THREE.Quaternion(-0.0, 0.0, 1, 1).normalize(),
        scale: new THREE.Vector3(1, 1, 1),
    }
    const next = function (deltaTime) {
        beginState = beginState ?? {
            pos: scene.position.clone(),
            rot: scene.quaternion.clone(),
            scale: scene.scale.clone(),
        }
        _internalState.cur_duration += deltaTime;
        _internalState.cur_duration = Math.min(_internalState.cur_duration, duration);
        const percent = _internalState.cur_duration / duration;

        scene.position.lerpVectors(beginState.pos, endState.pos, easeInCirc(percent));
        scene.quaternion.slerpQuaternions(beginState.rot, endState.rot, easeInCirc(percent));
        scene.scale.lerpVectors(beginState.scale, endState.scale, easeInOut(percent));

        if (Math.abs(duration - _internalState.cur_duration) < 0.1) {
            return false
        }
        return true
    };
    return { _internalState, next }
}

/**
 * 
 * @param {THREE.Scene} primary 
 * @param {...THREE.Scene} phones 
 * @returns 
 */
function slidePhonesIn(primary, ...phones) {
    const duration = 1000;
    const _internalState = {
        cur_duration: 0,
    };
    const beginState = {
        pos: new THREE.Vector3(0, 5, 0),
        rot: new THREE.Quaternion().normalize(),
        scale: new THREE.Vector3(1, 1, 1),
    };
    const endState = {
        pos: new THREE.Vector3(),
        rot: new THREE.Quaternion().normalize(),
        scale: new THREE.Vector3(1, 1, 1),
    };
    const next = function (deltaTime) {
        _internalState.cur_duration += deltaTime;
        _internalState.cur_duration = Math.min(_internalState.cur_duration, duration);
        const percent = _internalState.cur_duration / duration;

        primary.position.lerpVectors(beginState.pos, endState.pos, easeInOut(percent));
        primary.quaternion.slerpQuaternions(beginState.rot, endState.rot, easeInOut(percent));
        primary.scale.lerpVectors(beginState.scale, endState.scale, easeInOut(percent));

        //TODO: move other phones relatively

        if (Math.abs(duration - _internalState.cur_duration) < 0.1) {
            return false
        }
        return true
    };
    return { next }
}

/**
 * 
 * @param {THREE.Scene} scene 
 * @returns {Animation}
 */
function HeroAnimation(scene) {
    let children = [
        // show(scene),
        combine(fadeOutBGImg(), add(scene, watchScene), remove(scene, phoneScene, ...phones)),
        startWatch(watchScene),
        interpolate(watchScene, startWatch, tiltWatch),
        tiltWatch(watchScene),
        wait(500),
        moveWatchAway(watchScene),
        combine(remove(scene, watchScene), add(scene, phoneScene, ...phones)),
        slidePhonesIn(phoneScene, ...phones),
        // wait(500),
        // hide(scene),
    ]
    let next = function (delta) {
        let cur = children.shift();
        if (cur?.next(delta)) { children.unshift(cur); }
        if (children.length === 0) {
            return false;
        }
        return true;
    }
    return { next }
}

async function main() {

    startButton.removeEventListener("click", main);
    startButton.addEventListener("click", function () {
        if (!animation) { animation = HeroAnimation(scene); }
    })

    const renderer = new THREE.WebGLRenderer(
        { alpha: true, canvas: heroCanvas, antialias: true }
    );
    const scene = new THREE.Scene();
    const cam = new THREE.PerspectiveCamera(35);
    cam.position.z = 6;
    scene.add(watchScene);

    scene.add(phoneScene);
    phoneTextures.forEach(function (path, i) {
        let newScene = phoneScene.clone();

        newScene.traverse(child => {
            if (child.material && child.material.name === 'iphone-x-screenshot') {

                // Klone das Material und weise es dem Kind zu
                let clonedMaterial = child.material.clone();
                child.material = clonedMaterial;

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
                clonedMaterial.emissiveMap = texture;

                clonedMaterial.map.needsUpdate = true;
                clonedMaterial.emissiveMap.needsUpdate = true;
                clonedMaterial.needsUpdate = true;

            }
        });
        phones.push(newScene);
        scene.add(newScene);
    })

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

        const buttonState = startButton.getAttribute("data-state");

        if (animation) {
            if (!animation.next(deltaTime)) { animation = null; }

            if (!buttonState || buttonState == "play") {
                startButton.innerHTML = pause_icon;
                startButton.setAttribute("data-state", "pause");
            };
        } else {
            if (!buttonState || buttonState == "pause") {
                startButton.innerHTML = play_icon;
                startButton.setAttribute("data-state", "play");
            };
        }

        renderer.render(scene, cam);
        requestAnimationFrame((newTime) => render(newTime, time));
    }
    initAnimation(scene, watchScene, phoneScene, ...phones);
    requestAnimationFrame(render);

    animation = HeroAnimation(scene);
}

startButton.innerHTML = play_icon;
startButton.setAttribute("data-state", "play");
startButton.addEventListener("click", main)