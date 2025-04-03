import * as THREE from 'three';

export { THREE };
/**
 * 
 * @param {THREE.WebGLRenderer} renderer 
 * @returns true if renderer needs resizing
 */
export function rendererNeedsResize(renderer) {
    return renderer.domElement.width !== renderer.domElement.clientWidth
        || renderer.domElement.height !== renderer.domElement.clientHeight;
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
    light.position.set(position.x, position.y, position.z);
    scene.add(light);

}

/**
 * 
 * @param {THREE.WebGLRenderer} renderer 
 * @param {THREE.Camera} camera 
 * @returns if resize was necessary
 */
export function resize(renderer, camera) {
    if (rendererNeedsResize(renderer)) {
        renderer.setSize(renderer.domElement.clientWidth, renderer.domElement.clientHeight, false);

        const canvas = renderer.domElement;
        camera.aspect = canvas.clientWidth / canvas.clientHeight;
        camera.updateProjectionMatrix();
        return true;
    }
    return false;
}