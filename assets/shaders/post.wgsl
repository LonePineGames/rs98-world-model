#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::utils

@group(1) @binding(0)
var texture: texture_2d<f32>;

@group(1) @binding(1)
var our_sampler: sampler;

@group(1) @binding(2)
var<uniform> time: f32;

fn brightness(color: vec3<f32>) -> f32 {
  let ncolor = pow(color, vec3(2.2)); // Gamma correction
  let result = 0.2125*ncolor.r + 0.7154*ncolor.g + 0.0721*ncolor.b;
  return pow(result, 1.0/2.2); // Gamma correction
}

@fragment
fn fragment(
    @builtin(position) position: vec4<f32>,
    #import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {
    // Get screen position with coordinates from 0 to 1
    let uv = coords_to_viewport_uv(position.xy, view.viewport);
    let scanlines = 480.0;
    let vert_res = 1280.0;

    let barrel_distortion = 0.25;
    let rescale = 1.0 - (0.25 * barrel_distortion);

    let texcoord = uv - vec2<f32>(0.5);
    let rsq = texcoord.x * texcoord.x + texcoord.y * texcoord.y;
    let texcoord = texcoord + (texcoord * (barrel_distortion * rsq));
    let texcoord = texcoord * rescale;
    let uv_ndx = texcoord + vec2<f32>(0.5);
    //let uv_ndx = vec2<f32>(uv_ndx.x, floor(uv_ndx.y * vert_res) / vert_res);
    let offset = 0.001;
    
    let dist = max(abs(uv_ndx.x - 0.5), abs(uv_ndx.y - 0.5));
    let dist = dist - 0.5 + offset;
    var border = dist * 10.0;

    // Collect samples in a 3x3 grid
    let samples99 = textureSample(texture, our_sampler, uv_ndx + vec2<f32>(-offset, -offset));
    let samples90 = textureSample(texture, our_sampler, uv_ndx + vec2<f32>(0.0, -offset));
    let samples91 = textureSample(texture, our_sampler, uv_ndx + vec2<f32>(offset, -offset));
    let samples09 = textureSample(texture, our_sampler, uv_ndx + vec2<f32>(-offset, 0.0));
    let samples00 = textureSample(texture, our_sampler, uv_ndx);
    let samples01 = textureSample(texture, our_sampler, uv_ndx + vec2<f32>(offset, 0.0));
    let samples19 = textureSample(texture, our_sampler, uv_ndx + vec2<f32>(-offset, offset));
    let samples10 = textureSample(texture, our_sampler, uv_ndx + vec2<f32>(0.0, offset));
    let samples11 = textureSample(texture, our_sampler, uv_ndx + vec2<f32>(offset, offset));

    // Calculate edge detection
    let edge = length(samples99 - samples00)
        + length(samples90 - samples00)
        + length(samples91 - samples00)
        + length(samples09 - samples00)
        + length(samples01 - samples00)
        + length(samples19 - samples00)
        + length(samples10 - samples00)
        + length(samples11 - samples00);
    let edge = clamp( edge*0.25 , 0.0, 1.0);
    let average_color = (samples99 + samples90 + samples91 + samples09 + samples00 + samples01 + samples19 + samples10 + samples11) / 9.0;

    // let redshift = sin(time * 0.05) * 0.5 + 0.5;
    // let blueshift = cos(time * 0.05) * 0.5 + 0.5;
    // let redshift = redshift + pow(redshift, 240.0) * 2.0 + 0.5;
    // let blueshift = redshift + pow(blueshift, 240.0) * 2.0 + 0.5;
    // let redshift = redshift * 0.5;
    // let blueshift = blueshift * 0.75;
    let redshift = -1.0;
    let blueshift = 2.0;

    // Sample each color channel with an arbitrary shift
    var output_color = vec4<f32>(
        textureSample(texture, our_sampler, uv_ndx + vec2<f32>(offset*redshift, 0.0)).r,
        samples00.g,
        textureSample(texture, our_sampler, uv_ndx + vec2<f32>(offset*blueshift, 0.0)).b,
        1.0
        );
    output_color = mix(output_color, average_color, 0.5);

    // blur edges 
    let samples0 = samples99 + samples90 + samples91 + samples09 + samples00 + samples01 + samples19 + samples10 + samples11;
    let samples0 = samples0 / 9.0;
    output_color = mix(output_color, samples0, clamp(border, 0.0, 1.0));

    //output_color = pow(output_color, vec4<f32>(1.0/2.2)); // Gamma correction

    // Scanlines & effects
    //let scanline = abs(fract(uv.y * scanlines) - 0.5) * 2.0 - 0.5;
    let scanline = sin(uv_ndx.y * scanlines * 3.1415) * 0.5 + 0.5;
    let scanline = scanline * clamp(redshift + blueshift, 1.0, 2.0);
    let vignet = pow(2. - length(uv - vec2<f32>(0.5, 0.5)) * 1.5, 1.5);
    let sync = fract(floor((uv_ndx.y - time * 0.05) * scanlines) / scanlines) * 0.2;
    var brightness = 2.0;
    //0.8 + vignet;// - edge*2.0 - scanline; //vignet - sync - pow(edge, 0.5)*8.0 - 0.5;
    //let brightness = clamp(brightness, 0.5, 2.0);
    //var brightness = pow(brightness, 2.2); // Gamma correction

    if border > 0.0 {
        brightness *= clamp(border, 0.0, 0.01);
        let bevel = 100.0;
        let sides = clamp((uv.y - uv.x)*bevel, 0.0, 1.0) + clamp((uv.x + uv.y - 1.0)*bevel, 0.0, 1.0);
        output_color = output_color + vec4<f32>(0.2, 0.2, 0.2, 0.0) * sides;
    } else {
        brightness = vignet*0.5 - edge*1.5 - scanline*0.25;// - sync;
        brightness *= clamp(-0.1-border*10.0, 0.0, 1.0);
    }
    //let brightness = pow(brightness, 2.2); // Gamma correction
    output_color = vec4<f32>(output_color.rgb * brightness, 0.9);
    //output_color = vec4<f32>(output_color.rgb * (vignet - scanline*1.5 - sync - edge*2.0), 1.0);
    return output_color;
    // if (uv.x < 0.0 || uv.x > 1.0 || uv.y < 0.0 || uv.y > 1.0) {
    //     return output_color * (1.0 - border);
    // }

    // if border > 0.0 {
    //     border = clamp(border, 0.0, 1.0);
    //     border = pow(border, 2.0) * 4.0 + 0.5;
    // } else {
    //     border = clamp(border + 0.1, 0.0, 1.0);
    // }
    // return output_color * (1.0 - border);
}
