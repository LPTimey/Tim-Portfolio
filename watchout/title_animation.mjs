"use strict";
// @ts-ignore
import { GLTFLoader } from "three/addons/loaders/GLTFLoader";
import { THREE } from "../three_utils.mjs";

/**
 * @typedef {import('../three_utils.mjs')} Three
 */



const WatchPath = "./assets/Design%20der%20Mensch%20Maschine%20Schnittstelle/WatchOut/TimUhr.glb";
const PhonePath = "./assets/Design%20der%20Mensch%20Maschine%20Schnittstelle/WatchOut/iphone.glb";
const loader = new GLTFLoader();

const AnimationState = {
    running: false,
    time: 0,
    watch: loader.loadAsync(WatchPath),
    phone: loader.loadAsync(PhonePath)
}

async function TitleAnimation() {
    let renderer = new THREE.WebGLRenderer({alpha: true, antialias: true})
}

async function test() {
    debugger;
    let test = await AnimationState.watch;
    let test2 = await AnimationState.watch;
    console.log(test)
    console.log(test2)
}

test()