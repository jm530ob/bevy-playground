@group(0) @binding(0)
var<uniform> time: f32;

@vertex
fn vs_main(@location(0) pos: vec3<f32>) -> @builtin(position) vec4<f32> {
    var new_pos = pos;
    new_pos.y += sin(pos.x * 0.2 + time) * 0.5 + cos(pos.z * 0.2 + time) * 0.5;
    return vec4<f32>(new_pos, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(0.3, 0.6, 0.3, 1.0);  // Green color for terrain
}

