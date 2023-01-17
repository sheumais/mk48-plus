precision mediump float;

varying vec2 vPosition;
uniform vec2 uMiddle;
uniform vec3 uAbove_uArea_uBorder;
uniform vec2 uRestrict_uVisual;

float preciseLength(vec2 vec) {
    #define LENGTH_SCALE 64.0
    return length(vec * (1.0 / LENGTH_SCALE)) * LENGTH_SCALE;
}

void main() {
    float area = (vPosition.y - uAbove_uArea_uBorder.y) * uAbove_uArea_uBorder.x;
    float border = preciseLength(vPosition) - uAbove_uArea_uBorder.z;
    gl_FragColor = vec4(0.01, 0.01, 0.01, 1.0) * clamp(max(border, area) * 0.1, 0.0, 0.4);
    vec2 frPos = fract(0.01 * vPosition);
    gl_FragColor.x += (smoothstep(1., frPos.x,.98) + smoothstep(1., frPos.y,.98) + smoothstep(.0, frPos.x,.02) + smoothstep(.0, frPos.y,.02)) * clamp(max(border, area) * 0.06, 0.0, 0.9);
    gl_FragColor = mix(gl_FragColor, vec4(0.0, 0.0174, 0.0835, 1.0), clamp((preciseLength(vPosition - uMiddle) - uRestrict_uVisual.y) * 0.1, 0.0, uRestrict_uVisual.x));
}
