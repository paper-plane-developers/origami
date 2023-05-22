// Based on https://www.shadertoy.com/view/Xltfzj
uniform sampler2D u_texture1;

void mainImage(out vec4 fragColor,
    in vec2 fragCoord,
    in vec2 resolution,
    in vec2 uv) {
    float Pi = 6.28318530718; // Pi*2
    
    // GAUSSIAN BLUR SETTINGS {{{
    float Directions = 32.0; // BLUR DIRECTIONS (Default 16.0 - More is better but slower)
    float Quality = 16.0; // BLUR QUALITY (Default 4.0 - More is better but slower)
    float Size = 128.0; // BLUR SIZE (Radius)
    // GAUSSIAN BLUR SETTINGS }}}
   
    vec2 Radius = Size/resolution.xy;
    
    // Pixel colour
    vec4 Color = texture(u_texture1, uv);
    
    // Blur calculations
    for( float d=0.0; d<Pi; d+=Pi/Directions)
    {
		for(float i=1.0/Quality; i<=1.0; i+=1.0/Quality)
        {
            vec2 coord = uv+vec2(cos(d),sin(d))*Radius*i;
			Color += texture(u_texture1, clamp(coord, vec2(0), resolution));		
        }
    }
    
    // Output to screen
    Color /= Quality * Directions - 15.0;
    fragColor =  Color;
}