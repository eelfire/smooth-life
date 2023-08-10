#version 330

// Input vertex attributes (from vertex shader)
in vec2 fragTexCoord;
in vec4 fragColor;

// Input uniform values
uniform sampler2D texture0;
uniform vec4 colDiffuse;

// Output fragment color
out vec4 finalColor;

// NOTE: Add here your custom variables
uniform vec2 resolution;

uniform float ra = 12;
uniform float ri = 4;
uniform float b1 = 0.278;
uniform float b2 = 0.365;
uniform float d1 = 0.267;
uniform float d2 = 0.445;
uniform float alpha_n = 0.028;
uniform float alpha_m = 0.147;
uniform float dt = 6.9;

int emod(int a, int b) {
    return (a % b + b) % b;
}

float sigma(float x, float a, float alpha) {
    return 1.0 / (1.0 + exp(-(x - a) * 4.0 / alpha));
}

float sigma_n(float x, float a, float b) {
    return sigma(x, a, alpha_n) * (1 - sigma(x, b, alpha_n));
}

float sigma_m(float x, float y, float m) {
    return x * (1 - sigma(m, 0.5, alpha_m)) + y * sigma(m, 0.5, alpha_m);
}

float s(float n, float m) {
    return sigma_n(n, sigma_m(b1, d1, m), sigma_m(b2, d2, m));
}

float grid(float x, float y) {
    float tx = float(x) / resolution.x;
    float ty = float(y) / resolution.y;
    vec4 t = texture(texture0, vec2(tx, ty));
    return max(max(t.x, t.y), t.z);
}

void main() {
// #if 1
    float cx = fragTexCoord.x * resolution.x;
    float cy = (1 - fragTexCoord.y) * resolution.y;
    float m = 0, M = 0;
    float n = 0, N = 0;

    for(float dy = -(ra - 1); dy <= (ra - 1); ++dy) {
        for(float dx = -(ra - 1); dx <= (ra - 1); ++dx) {
            // int x = emod(cx + dx, int(resolution.x));
            // int y = emod(cy + dy, int(resolution.y));
            float x = cx + dx;
            float y = cy + dy;
            if(dx * dx + dy * dy <= ri * ri) {
                m += grid(x, y);
                M += 1;
            } else if(dx * dx + dy * dy <= ra * ra) {
                n += grid(x, y);
                N += 1;
            }
        }
    }
    m /= M;
    n /= N;
    float q = s(n, m);
    float diff = 2 * q - 1;
    float v = clamp(grid(cx, cy) + dt * diff, 0.0, 1.0);
// #endif

    // finalColor = texture(texture0, fragTexCoord) * vec4(0, 0.99, 0, 1);
    // finalColor = vec4(gl_FragCoord.x / resolution.x, gl_FragCoord.y / resolution.y, 0, 1);
    finalColor = vec4(v, v, v, 1);
    // finalColor = texture(texture0, fragTexCoord);
}