/* =========================================================
   Shaders
========================================================= */

const vsSource = `
attribute vec2 a_pos;
attribute vec2 a_uv;

varying vec2 v_uv;

void main() {
  v_uv = a_uv;
  gl_Position = vec4(a_pos, 0.0, 1.0);
}
`;

const fsSource = `
precision mediump float;

uniform sampler2D u_texture;
uniform vec2 u_offset;
uniform vec2 u_scale;

varying vec2 v_uv;

void main() {
  vec2 uv = v_uv * u_scale + u_offset;
  uv = fract(uv);
  gl_FragColor = texture2D(u_texture, uv);
}
`;

/* =========================================================
   WebGL Helpers
========================================================= */

/**
 * @param {WebGLRenderingContext} gl
 * @param {number} type
 * @param {string} source
 */
function createShader(gl, type, source) {
    const shader = gl.createShader(type);
    if (!shader) throw new Error("Shader creation failed");

    gl.shaderSource(shader, source);
    gl.compileShader(shader);

    if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
        const log = gl.getShaderInfoLog(shader);
        gl.deleteShader(shader);
        throw new Error(log || "Shader compile error");
    }
    return shader;
}

/**
 * @param {WebGLRenderingContext} gl
 * @param {string} vsSource
 * @param {string} fsSource
 */
function createProgram(gl, vsSource, fsSource) {
    const vs = createShader(gl, gl.VERTEX_SHADER, vsSource);
    const fs = createShader(gl, gl.FRAGMENT_SHADER, fsSource);

    const program = gl.createProgram();
    if (!program) throw new Error("Program creation failed");

    gl.attachShader(program, vs);
    gl.attachShader(program, fs);
    gl.linkProgram(program);

    if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
        const log = gl.getProgramInfoLog(program);
        throw new Error(log || "Program link error");
    }

    return program;
}

/**
 * @param {WebGLRenderingContext} gl
 * @param {string} url
 */
async function loadTextureGL(gl, url) {
    const img = new Image();
    img.src = url;
    await img.decode();

    const tex = gl.createTexture();
    if (!tex) throw new Error("Texture creation failed");

    gl.bindTexture(gl.TEXTURE_2D, tex);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.REPEAT);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.REPEAT);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);

    gl.texImage2D(
        gl.TEXTURE_2D,
        0,
        gl.RGBA,
        gl.RGBA,
        gl.UNSIGNED_BYTE,
        img
    );

    return {
        texture: tex,
        width: img.width,
        height: img.height,
    };
}

/* =========================================================
   Public API
========================================================= */

/**
 * @param {HTMLCanvasElement} canvas
 */
export async function initWebGL(canvas) {
    const gl = /** @type {WebGLRenderingContext} */ (
        canvas.getContext("webgl2")
    );
    if (!gl) throw new Error("WebGL not supported");

    /* =============================
       State (entspricht WebGPU)
    ============================== */

    const zoom = Number(canvas.dataset.zoom ?? 1);
    const speed = Number(canvas.dataset.speed ?? 1);
    const angleRad = Number(canvas.dataset.angleRad ?? 0);
    const speedScale = 0.0001;

    let scaleX = 1, scaleY = 1;
    let offsetX = 0, offsetY = 0;
    let scrollX = 0, scrollY = 0;

    /* =============================
       Program + Geometry
    ============================== */

    const program = createProgram(gl, vsSource, fsSource);
    gl.useProgram(program);

    const vertices = new Float32Array([
        -1, -1, 0, 1,
        1, -1, 1, 1,
        1, 1, 1, 0,
        -1, -1, 0, 1,
        1, 1, 1, 0,
        -1, 1, 0, 0,
    ]);

    const buffer = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
    gl.bufferData(gl.ARRAY_BUFFER, vertices, gl.STATIC_DRAW);

    const aPos = gl.getAttribLocation(program, "a_pos");
    const aUV = gl.getAttribLocation(program, "a_uv");

    gl.enableVertexAttribArray(aPos);
    gl.vertexAttribPointer(aPos, 2, gl.FLOAT, false, 16, 0);

    gl.enableVertexAttribArray(aUV);
    gl.vertexAttribPointer(aUV, 2, gl.FLOAT, false, 16, 8);

    /* =============================
       Uniforms
    ============================== */

    const uOffset = gl.getUniformLocation(program, "u_offset");
    const uScale = gl.getUniformLocation(program, "u_scale");

    const { texture, width: texW, height: texH } =
        await loadTextureGL(gl, String(canvas.dataset.href));

    gl.activeTexture(gl.TEXTURE0);
    gl.bindTexture(gl.TEXTURE_2D, texture);

    /* =============================
       Resize / Aspect Logic
    ============================== */

    function resizeCanvas() {
        const dpr = devicePixelRatio || 1;
        canvas.width = canvas.clientWidth * dpr;
        canvas.height = canvas.clientHeight * dpr;
        gl.viewport(0, 0, canvas.width, canvas.height);

        const canvasAspect = canvas.width / canvas.height;
        const textureAspect = texW / texH;

        if (canvasAspect > textureAspect) {
            scaleX = (canvasAspect / textureAspect) / zoom;
            scaleY = 1 / zoom;
            offsetX = 0.5 * (1 - 1 / scaleX);
            offsetY = 0;
        } else {
            scaleX = 1 / zoom;
            scaleY = (textureAspect / canvasAspect) / zoom;
            offsetX = 0;
            offsetY = 0.5 * (1 - 1 / scaleY);
        }
    }

    resizeCanvas();
    window.addEventListener("resize", resizeCanvas);

    /* =============================
       Render Loop
    ============================== */

    let lastTime = performance.now();

    /**
     * 
     * @param {number} time 
     */
    function frame(time) {
        const dt = time - lastTime;
        lastTime = time;

        scrollX -= Math.cos(angleRad) * speed * speedScale * dt;
        scrollY += Math.sin(angleRad) * speed * speedScale * dt;

        gl.uniform2f(uOffset, scrollX + offsetX, scrollY + offsetY);
        gl.uniform2f(uScale, scaleX, scaleY);

        gl.clearColor(0, 0, 0, 1);
        gl.clear(gl.COLOR_BUFFER_BIT);
        gl.drawArrays(gl.TRIANGLES, 0, 6);

        requestAnimationFrame(frame);
    }

    requestAnimationFrame(frame);
}
