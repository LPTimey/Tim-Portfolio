/** @type {NodeListOf<HTMLCanvasElement>} */
const canvasses = /** */(document.querySelectorAll(".scroll-img"));

const gpu = /** @type {any} */ (navigator).gpu;
const GPUTextureUsage = /** @type {any} */ (window).GPUTextureUsage;
const GPUBufferUsage = /** @type {any} */ (window).GPUBufferUsage;
const code = `
struct Uniforms {
  offset : vec2<f32>,
  scale  : vec2<f32>,
};

@group(0) @binding(0) var mySampler : sampler;
@group(0) @binding(1) var myTexture : texture_2d<f32>;
@group(0) @binding(2) var<uniform> uniforms : Uniforms;

struct VertexOut {
  @builtin(position) position : vec4<f32>,
  @location(0) uv : vec2<f32>,
};

@vertex
fn vs(
  @location(0) pos : vec2<f32>,
  @location(1) uv  : vec2<f32>
) -> VertexOut {
  var out : VertexOut;
  out.position = vec4<f32>(pos, 0.0, 1.0);
  out.uv = uv;
  return out;
}

@fragment
fn fs(in : VertexOut) -> @location(0) vec4<f32> {
  let uv = in.uv * uniforms.scale + uniforms.offset;
  let scrolledUV = fract(uv);
  return textureSample(myTexture, mySampler, scrolledUV);
}
`;

canvasses.forEach(async (canvas) => {
    /* =========================================================
        consts
    ========================================================= */
    //#region consts
    const href = String(canvas.dataset.href);
    const zoom = Number(canvas.dataset.zoom);
    const speed = Number(canvas.dataset.speed);
    const angleRad = Number(canvas.dataset.angleRad);
    const dY = Math.sin(angleRad);
    const dX = Math.cos(angleRad);
    const speedScale = 0.0001;
    //#endregion consts

    /* =========================================================
        globals
    ========================================================= */
    //#region globals
    /** @type {any} */
    let device;
    /** @type {any} */
    let context;
    /** @type {any} */
    let format;
    /** @type {any} */
    let texture;

    let textureWidth = 1;
    let textureHeight = 1;

    let scaleX = 1;
    let scaleY = 1;
    let offsetX = 0;
    let offsetY = 0;
    //#endregion globals

    /* =========================================================
        Functions
    ========================================================= */
    //#region Functions
    async function initWebGPU() {
        const adapter = await gpu.requestAdapter();
        device = await adapter.requestDevice();

        context = canvas.getContext("webgpu");
        format = gpu.getPreferredCanvasFormat();

        resizeCanvas();

        context.configure({
            device,
            format,
            alphaMode: "opaque",
        });
    }
    function resizeCanvas() {
        const dpr = window.devicePixelRatio || 1;
        canvas.width = canvas.clientWidth * dpr;
        canvas.height = canvas.clientHeight * dpr;

        if (!texture) return;

        const canvasAspect = canvas.width / canvas.height;
        const textureAspect = textureWidth / textureHeight;

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
    /**
     * @param {string} url
     */
    async function loadTexture(url) {
        const img = new Image();
        img.src = url;
        await img.decode();

        const bitmap = await createImageBitmap(img);

        textureWidth = bitmap.width;
        textureHeight = bitmap.height;

        const tex = device.createTexture({
            size: [bitmap.width, bitmap.height, 1],
            format: "rgba8unorm",
            usage:
                GPUTextureUsage.TEXTURE_BINDING |
                GPUTextureUsage.COPY_DST |
                GPUTextureUsage.RENDER_ATTACHMENT,
        });

        device.queue.copyExternalImageToTexture(
            { source: bitmap },
            { texture: tex },
            [bitmap.width, bitmap.height]
        );

        return tex;
    }
    //#endregion Functions

    /* =========================================================
        Main
    ========================================================= */
    //#region Main
    await initWebGPU();
    texture = await loadTexture(href);
    resizeCanvas();

    const sampler = device.createSampler({
        addressModeU: "repeat",
        addressModeV: "repeat",
        magFilter: "linear",
        minFilter: "linear",
    });


    /* =========================================================
       Geometry
    ========================================================= */
    const vertices = new Float32Array([
        -1, -1, 0, 1,
        1, -1, 1, 1,
        1, 1, 1, 0,

        -1, -1, 0, 1,
        1, 1, 1, 0,
        -1, 1, 0, 0,
    ]);

    const vertexBuffer = device.createBuffer({
        size: vertices.byteLength,
        usage: GPUBufferUsage.VERTEX | GPUBufferUsage.COPY_DST,
    });

    device.queue.writeBuffer(vertexBuffer, 0, vertices);

    /* =========================================================
       Uniforms
    ========================================================= */
    const uniformBuffer = device.createBuffer({
        size: 16,
        usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
    });

    /* =========================================================
       Shader
    ========================================================= */
    const shaderModule = device.createShaderModule({
        code
    });

    /* =========================================================
       Pipeline + BindGroup
    ========================================================= */
    const pipeline = device.createRenderPipeline({
        layout: "auto",
        vertex: {
            module: shaderModule,
            entryPoint: "vs",
            buffers: [{
                arrayStride: 16,
                attributes: [
                    { shaderLocation: 0, offset: 0, format: "float32x2" },
                    { shaderLocation: 1, offset: 8, format: "float32x2" },
                ],
            }],
        },
        fragment: {
            module: shaderModule,
            entryPoint: "fs",
            targets: [{ format }],
        },
        primitive: { topology: "triangle-list" },
    });

    const bindGroup = device.createBindGroup({
        layout: pipeline.getBindGroupLayout(0),
        entries: [
            { binding: 0, resource: sampler },
            { binding: 1, resource: texture.createView() },
            { binding: 2, resource: { buffer: uniformBuffer } },
        ],
    });

    /* =========================================================
       Render Loop
    ========================================================= */
    let texScrollX = 0;
    let texScrollY = 0;

    /**
     * 
     * @param {number} time 
     * @param {number} lastTime 
     */
    function frame(time, lastTime) {
        const deltaTime = time - lastTime;

        resizeCanvas();

        texScrollX -= (dX * speed * speedScale * deltaTime);
        texScrollY += (dY * speed * speedScale * deltaTime);

        device.queue.writeBuffer(
            uniformBuffer,
            0,
            new Float32Array([
                texScrollX + offsetX,
                texScrollY + offsetY,
                scaleX,
                scaleY,
            ])
        );

        const encoder = device.createCommandEncoder();
        const pass = encoder.beginRenderPass({
            colorAttachments: [{
                view: context.getCurrentTexture().createView(),
                loadOp: "clear",
                storeOp: "store",
                clearValue: { r: 0, g: 0, b: 0, a: 1 },
            }],
        });

        pass.setPipeline(pipeline);
        pass.setBindGroup(0, bindGroup);
        pass.setVertexBuffer(0, vertexBuffer);
        pass.draw(6);
        pass.end();

        device.queue.submit([encoder.finish()]);
        requestAnimationFrame((newTime) => frame(newTime, time));
    }

    requestAnimationFrame((time) => frame(time, time));
    //#endregion Main
})