#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::utils

@group(1) @binding(0)
var texture: texture_2d<f32>;

@group(1) @binding(1)
var our_sampler: sampler;

@group(1) @binding(2)
var<uniform> time: f32;

@fragment
fn fragment(
    @builtin(position) position: vec4<f32>,
    #import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {
    // Get screen position with coordinates from 0 to 1
    let uv = coords_to_viewport_uv(position.xy, view.viewport);
    let scanlines = 1280.0;
    let uv_ndx = vec2<f32>(uv.x, floor(uv.y * scanlines) / scanlines);
    let offset = 0.002;

    // Sample each color channel with an arbitrary shift
    var output_color = vec4<f32>(
        textureSample(texture, our_sampler, uv_ndx + vec2<f32>(-offset, 0.0)).r,
        textureSample(texture, our_sampler, uv_ndx + vec2<f32>(0.0, 0.0)).g,
        textureSample(texture, our_sampler, uv_ndx + vec2<f32>(offset, 0.0)).b,
        1.0
        );
    
    // Scanlines
    let scanline = abs(fract(uv.y * scanlines * 0.25) - 0.5) * 2.0 - 0.5;
    let vignet = pow(2.5 - length(uv - vec2<f32>(0.5, 0.5)) * 2.0, 1.5);
    let sync = fract(floor((uv.y - time * 0.05) * scanlines) / scanlines) * 0.2;
    output_color = vec4<f32>(output_color.rgb * (vignet - scanline*0.5 - sync), 1.0);

    return output_color;
}
