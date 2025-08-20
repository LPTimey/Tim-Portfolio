import * as THREE from "three"

/**
 * 
 * @param {THREE.Scene} scene 
 * @param {{x:number,y:number,z:number}} position 
 * @param {number} [intensity = 2] default: 2
 */
export function addLight(scene, position, intensity = 2) {

    const color = 0xFFFFFF;
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

    const canvas = renderer.domElement;
    const pixelRatio = window.devicePixelRatio;
    const width = canvas.clientWidth;
    const height = canvas.clientHeight;

    renderer.setPixelRatio(pixelRatio);
    renderer.setSize(width, height, false);

    // @ts-ignore
    if (camera instanceof THREE.PerspectiveCamera) {
        camera.aspect = width / height;
        // @ts-ignore
    } else if (camera instanceof THREE.OrthographicCamera) {
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
}