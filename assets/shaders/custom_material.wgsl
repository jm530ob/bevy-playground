// Uniforms
@group(0) @binding(0) var<uniform> iTime: f32;
@group(0) @binding(1) var<uniform> texture1: texture_2d<f32>;
@group(0) @binding(2) var<uniform> textures: array<texture_2d<f32>, 4>;
@group(0) @binding(3) var<uniform> sampler1: sampler;

// Vertex Input/Output
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) vUv: vec2<f32>,
    @location(1) cloudUV: vec2<f32>,
    @location(2) vColor: vec3<f32>,
};

// Vertex Shader
@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.vUv = input.uv;
    output.cloudUV = input.uv;
    output.vColor = input.color;

    var cpos = input.position;

    let waveSize = 10.0;
    let tipDistance = 0.3;
    let centerDistance = 0.1;

    if (input.color.x > 0.6) {
        cpos.x += sin((iTime / 500.0) + (input.uv.x * waveSize)) * tipDistance;
    } else if (input.color.x > 0.0) {
        cpos.x += sin((iTime / 500.0) + (input.uv.x * waveSize)) * centerDistance;
    }

    let diff = input.position.x - cpos.x;
    output.cloudUV.x += iTime / 20000.0;
    output.cloudUV.y += iTime / 10000.0;

    // Assuming identity matrices for projection and modelView for simplicity
    output.position = vec4<f32>(cpos, 1.0);
    return output;
}

// Fragment Shader
@fragment
fn fs_main(@location(0) vUv: vec2<f32>, 
           @location(1) cloudUV: vec2<f32>, 
           @location(2) vColor: vec3<f32>) -> @location(0) vec4<f32> {
    let contrast = 1.5;
    let brightness = 0.1;

    // Texture sampling
    let baseColor = textureSample(textures[0], sampler1, vUv).rgb;
    var color = baseColor * contrast;
    color += vec3<f32>(brightness);

    let cloudColor = textureSample(textures[1], sampler1, cloudUV).rgb;
    color = mix(color, cloudColor, 0.4);

    return vec4<f32>(color, 1.0);
}
