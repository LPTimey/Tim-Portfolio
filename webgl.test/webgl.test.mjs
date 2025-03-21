import * as THREE from 'three';
import { GLTFLoader } from 'three/addons/loaders/GLTFLoader.js';

function main() {

    const canvas = document.querySelector('#c');
    const renderer = new THREE.WebGLRenderer({ antialias: true, canvas });

    const fov = 75;
    const aspect = 2; // the canvas default
    const near = 0.1;
    const far = 5000;
    const camera = new THREE.PerspectiveCamera(fov, aspect, near, far);
    camera.position.z = 4;
    camera.position.x = 1;
    camera.position.y = 0.25;

    const scene = new THREE.Scene();

    /**
     * 
     * @param {{x:number,y:number,z:number}} position 
     */
    function addLight(position) {

        const color = 0xFFFFFF;
        const intensity = 2;
        const light = new THREE.DirectionalLight(color, intensity);
        light.position.set(position.x, position.y, position.z);
        scene.add(light);

    }

    addLight({
        x: - 1, y: 2, z: 4
    });
    addLight({
        x: 1, y: -2, z: 1
    });

    const boxWidth = 1;
    const boxHeight = 1;
    const boxDepth = 1;
    const geometry = new THREE.BoxGeometry(boxWidth, boxHeight, boxDepth);

    function makeInstance(geometry, color, x, materialConstructor) {

        // const material = new THREE.MeshPhongMaterial({ color });
        const material = new materialConstructor({ color });

        const cube = new THREE.Mesh(geometry, material);
        scene.add(cube);

        cube.position.x = x;

        return cube;

    }

    const cubes = [
        makeInstance(geometry, 0x44aa88, 0, THREE.MeshPhongMaterial),
        makeInstance(geometry, 0x8844aa, - 2, THREE.MeshPhysicalMaterial),
        makeInstance(geometry, 0xaa8844, 2, THREE.MeshLambertMaterial),
    ];

    const loader = new GLTFLoader();

    loader.load('./assets/Design%20der%20Mensch%20Maschine%20Schnittstelle/WatchOut/TimUhr.glb', function (gltf) {

        scene.add(gltf.scene);

    }, undefined, function (error) {

        console.error(error);

    });

    /**
     * 
     * @param {THREE.WebGLRenderer} renderer 
     * @returns 
     */
    function resizeRendererToDisplaySize(renderer) {

        const canvas = renderer.domElement;
        const width = canvas.clientWidth;
        const height = canvas.clientHeight;
        const needResize = canvas.width !== width || canvas.height !== height;
        if (needResize) {

            renderer.setSize(width, height, false);

        }

        return needResize;

    }

    function render(time) {

        time *= 0.001;

        if (resizeRendererToDisplaySize(renderer)) {

            const canvas = renderer.domElement;
            camera.aspect = canvas.clientWidth / canvas.clientHeight;
            camera.updateProjectionMatrix();

        }

        cubes.forEach((cube, ndx) => {

            const speed = 1 + ndx * .1;
            const rot = time * speed;
            cube.rotation.x = rot;
            cube.rotation.y = rot;

        });

        renderer.render(scene, camera);

        requestAnimationFrame(render);

    }

    requestAnimationFrame(render);

}

main();
