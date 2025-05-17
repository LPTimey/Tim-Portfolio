import { THREE, resize, addDebug, alignText } from "../three_utils.mjs"
import { FontLoader, TextGeometry, OrbitControls, TTFLoader, Font } from "three/addons/Addons.js";
/** @import {Alignment,Justify} from "../three_utils.mjs" */

/** @type {HTMLCanvasElement | null} */
const BitCanvas = document.getElementById("BitListAnimation");

/**
 * @param {Object} param0
 * @param {number} param0.width 
 * @param {number} param0.height 
 * @param {number} [param0.gap=1] 
 * @param {{ width: number; height: number; }} [param0.boxSize={width:4,height:4}] 
 * @returns 
 */
function createFieldMatrix({ width, height, gap = 1, boxSize = { width: 4, height: 4 } }) {
    const geometry = new THREE.PlaneGeometry(boxSize.width, boxSize.height);
    const material = new THREE.MeshBasicMaterial({ color: 0x00ff00, side: THREE.DoubleSide });
    const plane = new THREE.Mesh(geometry, material);
    const group = new THREE.Group();

    for (let x = 0; x < width; x++) {
        for (let y = 0; y < height; y++) {
            const nPlane = new THREE.Mesh(geometry, material);
            nPlane.position.set(
                (boxSize.width + gap) * x,
                (boxSize.height + gap) * -y,
                0
            );
            addDebug(nPlane);

            group.add(nPlane);
        }
    }
    group.position.set(0,0,0);
    group.updateMatrixWorld(true);
    addDebug(group);

    return group;
}


/**
 * @param {Object} param0
 * @param {string} param0.text 
 * @param {number} [param0.fill=0x000000] 
 * @param {string} [param0.fontSrc='assets/JetBrains_Mono/JetBrainsMono-VariableFont_wght.ttf'] 
 * @param {Alignment} [param0.align="start"] 
 * @param {Justify} [param0.justify="start"] 
 * @returns 
 */
async function createText({ text, fill = 0x000000, fontSrc = 'assets/JetBrains_Mono/JetBrainsMono-VariableFont_wght.ttf', align = "center", justify = "start" }) {
    const loader = new TTFLoader();
    const json = await loader.loadAsync(fontSrc);
    const font = new Font(json);
    const textGeometry = new TextGeometry(text, {
        font: font,
        size: 3,
        depth: 0,
        bevelEnabled: false,
    });

    const textMaterial = new THREE.MeshBasicMaterial({ color: fill, side: THREE.DoubleSide });
    const textMesh = new THREE.Mesh(textGeometry, textMaterial);

    textGeometry.computeBoundingBox();
    alignText(textGeometry, justify, align);

    return textMesh;
}

async function bitListAnimation() {
    const renderer = new THREE.WebGLRenderer({ alpha: true, antialias: true, canvas: BitCanvas });
    const scene = new THREE.Scene();
    // const cam = new THREE.OrthographicCamera(
    //     BitCanvas.clientWidth / -50, BitCanvas.clientWidth / 50,
    //     BitCanvas.clientHeight / 50, BitCanvas.clientHeight / -50,
    //     1, 1000
    // );
    const cam = new THREE.PerspectiveCamera(35);
    cam.position.z = 100;

    const geometry = new THREE.PlaneGeometry(4, 4);
    const material = new THREE.MeshBasicMaterial({ color: 0x00ff00 });
    const plane = new THREE.Mesh(geometry, material);
    // scene.add(plane);
    const controls = new OrbitControls(cam, renderer.domElement);

    const axesHelper = new THREE.AxesHelper();
    // scene.add(axesHelper);

    const textMesh = await createText({ text: "Field:", justify: "end" });
    addDebug(textMesh);
    scene.add(textMesh);

    const matrix = createFieldMatrix({ width: 2, height: 3, gap: 0.5, boxSize: { width: 2, height: 2 } });
    scene.add(matrix);

    const render = function (time, lastTime) {
        const deltaTime = time - (lastTime ?? 0);

        resize(renderer, cam);
        renderer.render(scene, cam);
        requestAnimationFrame((newTime) => render(newTime, time));
    }
    requestAnimationFrame(render);
}

function test() {
    const scene = new THREE.Scene();
    const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);

    const renderer = new THREE.WebGLRenderer();
    renderer.setSize(window.innerWidth, window.innerHeight);
    renderer.setAnimationLoop(animate);
    document.body.appendChild(renderer.domElement);

    const geometry = new THREE.BoxGeometry(1, 1, 1);
    const material = new THREE.MeshBasicMaterial({ color: 0x00ff00 });
    const cube = new THREE.Mesh(geometry, material);
    scene.add(cube);

    camera.position.z = 5;

    function animate() {

        cube.rotation.x += 0.01;
        cube.rotation.y += 0.01;

        renderer.render(scene, camera);

    }
}

/** test code als reference nach mehreren tutorials */
function test2() {
    const renderer = new THREE.WebGLRenderer({ antialias: true });
    document.body.appendChild(renderer.domElement);
    const scene = new THREE.Scene();
    const camera = new THREE.OrthographicCamera(
        window.innerWidth / -50, window.innerWidth / 50,
        window.innerHeight / 50, window.innerHeight / -50,
        1, 1000
    );
    camera.position.z = 10;

    renderer.setSize(window.innerWidth, window.innerHeight);
    resize(renderer, camera);
    const cellSize = 4;
    const tapeLength = 15;
    const tape = Array.from({ length: tapeLength }, (_, i) => (i === 7 ? "1" : Math.random() > 0.5 ? "1" : "0"));
    const tapeObjects = [];

    let state = "q0";
    let position = 7;

    /**
     * Erstellt einen Sprite mit Text.
     * @param {string} message - Der Text, der angezeigt wird.
     * @param {string} color - Die Textfarbe.
     * @returns {THREE.Sprite}
     */
    function makeTextSprite(message, color = "#ffffff") {
        const canvas = document.createElement("canvas");
        const context = canvas.getContext("2d");
        context.font = "48px monospace";
        context.fillStyle = color;
        context.fillText(message, 10, 50);

        const texture = new THREE.CanvasTexture(canvas);
        const material = new THREE.SpriteMaterial({ map: texture });
        const sprite = new THREE.Sprite(material);
        sprite.scale.set(2.5, 2.5, 1);
        return sprite;
    }

    /**
     * Zeichnet das Band.
     */
    function drawTape() {
        for (let i = 0; i < tape.length; i++) {
            const x = (i - tape.length / 2) * cellSize;

            const cell = new THREE.Mesh(
                new THREE.PlaneGeometry(cellSize - 0.5, cellSize - 0.5),
                new THREE.MeshBasicMaterial({ color: 0x222222 })
            );
            cell.position.set(x, 0, 0);
            scene.add(cell);

            const symbol = makeTextSprite(tape[i]);
            symbol.position.set(x, 0, 0.1);
            scene.add(symbol);

            tapeObjects.push({ cell, symbol });
        }
    }

    let readHead;

    /**
     * Zeichnet den Lesekopf.
     */
    function drawReadHead() {
        readHead = new THREE.Mesh(
            new THREE.PlaneGeometry(cellSize, 0.5),
            new THREE.MeshBasicMaterial({ color: 0xff0000 })
        );
        readHead.position.set(getX(position), cellSize / 1.5, 0.2);
        scene.add(readHead);
    }

    function getX(pos) {
        return (pos - tape.length / 2) * cellSize;
    }

    function updateReadHead() {
        readHead.position.x = getX(position);
    }

    function updateSymbolAt(index) {
        scene.remove(tapeObjects[index].symbol);
        const newSymbol = makeTextSprite(tape[index]);
        newSymbol.position.set(getX(index), 0, 0.1);
        scene.add(newSymbol);
        tapeObjects[index].symbol = newSymbol;
    }

    // Beispielhafter Übergangsalgorithmus
    function step() {
        const symbol = tape[position];
        if (state === "q0") {
            if (symbol === "1") {
                tape[position] = "0";
                updateSymbolAt(position);
                position++;
                state = "q0";
            } else if (symbol === "0") {
                tape[position] = "1";
                updateSymbolAt(position);
                position--;
                state = "q1";
            } else {
                state = "HALT";
            }
        } else if (state === "q1") {
            if (symbol === "1") {
                tape[position] = " ";
                updateSymbolAt(position);
                position++;
                state = "HALT";
            } else {
                state = "HALT";
            }
        }
    }

    let delay = 0;

    function animate() {
        requestAnimationFrame(animate);
        resize(renderer, camera);

        if (state !== "HALT") {
            delay++;
            if (delay > 60) { // Schritt alle 60 Frames (~1 Sekunde)
                step();
                updateReadHead();
                delay = 0;
            }
        }

        renderer.render(scene, camera);
    }

    drawTape();
    drawReadHead();
    animate();
}

async function main() {
    bitListAnimation();
    // test();
    test2();
}

main()