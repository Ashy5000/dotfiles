#version 320 es
precision highp float;
in vec2 v_texcoord;
out vec4 FragColor;
uniform sampler2D tex;

void main() {
    float gamma = 0.6;
    float numColors = 10.0;
    vec3 c = texture2D(tex, v_texcoord).rgb;
    c = pow(c, vec3(gamma, gamma, gamma));
    c = c * numColors;
    c = floor(c);
    c = c / numColors;
    c = pow(c, vec3(1.0/gamma));
    float dx = 1.0 - abs(v_texcoord.x - 0.5);
    float dy = 1.0 - abs(v_texcoord.y - 0.5);
    float mask = sqrt(dx * dx + dy * dy);
    c = c * mask;
    FragColor = vec4(c, 1.0);
}
