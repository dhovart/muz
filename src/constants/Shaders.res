let vertexShader = `
  attribute vec2 a_position;
  void main() {
    gl_Position = vec4(a_position, 0.0, 1.0);
  }
`

let defaultFragmentShader = `precision mediump float;

uniform float u_time;
uniform float u_progress;
uniform sampler2D u_spectrumTexture;
uniform vec2 u_resolution;

float getAmplitude(float index) {
  return texture2D(u_spectrumTexture, vec2(index / 64.0, 0.5)).r;
}

void main() {
  vec2 uv = gl_FragCoord.xy / u_resolution.xy;
  
  // Sample spectrum data
  float bass = getAmplitude(4.0);
  float mid = getAmplitude(20.0);
  float treble = getAmplitude(50.0);
  float overallSpectrum = (bass + mid + treble) / 3.0;
  
  bass *= 5.0;
  mid *= 2.0;
  treble *= 2.0;
  overallSpectrum *= 2.0;
  
  float localFreq = getAmplitude(uv.x * 32.0) * 3.0;
  
  vec3 timeOffset = vec3(0.0, 2.0, 4.0);
  vec3 spectrumOffset = vec3(bass * 3.0, mid, treble);
  
  vec3 col = 0.5 + 0.5 * cos(
    u_time + 
    uv.xyx + 
    timeOffset + 
    spectrumOffset * 2.0 + 
    localFreq * 0.5
  );
  
  col = mix(col, vec3(col.r, col.g * 0.5, col.b * 0.3), bass * 0.4);
  col = mix(col, col * col, overallSpectrum * 0.6);
  col *= (0.7 + overallSpectrum * 0.5 + bass * 0.3);
  
  gl_FragColor = vec4(col, 1.0);
}`

let colorfulThingyFragmentShader = `precision mediump float;

uniform float u_time;
uniform float u_progress;
uniform sampler2D u_spectrumTexture;
uniform vec2 u_resolution;

#define PI 3.14159
#define TWO_PI 6.28318

// Perlin Noise from https://gist.github.com/patriciogonzalezvivo/670c22f3966e662d2f83
float rand(vec2 c){
    return fract(sin(dot(c.xy ,vec2(12.9898,78.233))) * 43758.5453);
}

float getAmplitude(float index) {
  return texture2D(u_spectrumTexture, vec2(index / 64.0, 0.5)).r;
}


float noise(vec2 p, float freq ){
    float unit = 1./freq;
    vec2 ij = floor(p/unit);
    vec2 xy = mod(p,unit)/unit;
    xy = .5*(1.-cos(PI*xy));
    float a = rand((ij+vec2(0.,0.)));
    float b = rand((ij+vec2(1.,0.)));
    float c = rand((ij+vec2(0.,1.)));
    float d = rand((ij+vec2(1.,1.)));
    float x1 = mix(a, b, xy.x);
    float x2 = mix(c, d, xy.x);
    return mix(x1, x2, xy.y);
}

float pNoise(vec2 p, int res){
    float persistance = .5;
    float n = 0.;
    float normK = 0.;
    float f = 4.;
    float amp = 1.;
    int iCount = 0;
    for (int i = 0; i<50; i++){
        n+=amp*noise(p, f);
        f*=2.;
        normK+=amp;
        amp*=persistance;
        if (iCount == res) break;
        iCount++;
    }
    float nf = n/normK;
    return nf*nf*nf*nf;
}

void main() {
    float bass = getAmplitude(4.0);
    float mid = getAmplitude(20.0);
    float treble = getAmplitude(50.0);
    float overallSpectrum = (bass + mid + treble) / 3.0;

    vec2 fragCoord = gl_FragCoord.xy;
    vec2 uv = (fragCoord.xy-.5*u_resolution.xy)/u_resolution.y;
    vec2 st = vec2(atan(uv.x, uv.y), length(uv));
    uv = vec2(.5+st.x/TWO_PI, st.y);

    float t = u_time * 2.0;
    float x, y, m;
    
    float n = pNoise(vec2((cos(uv.x * TWO_PI) + 1.), (sin(uv.y * TWO_PI) + 1.) )+u_time/9., 10);
    
    vec3 col;
    
    for(int i = 0; i < 3; i++) {
        x = uv.x + n * .1;
        y = uv.y + n * .09 + sin(uv.x * TWO_PI * 10. + t) * .05;
        
        x *= 20. + bass;
        m = min(fract(x), fract(1.-x)) + bass * 10.;
        
        float colorIntensity = (0.2 + m * 0.1 - y) + overallSpectrum * 0.2;

        col[i] = smoothstep(0., .1, colorIntensity);
        t+= 1.;
    }
    
    gl_FragColor = vec4(col, 1.0);
}`

let simpleFragmentShader = `precision mediump float;

uniform float u_time;
uniform float u_progress;
uniform sampler2D u_spectrumTexture;
uniform vec2 u_resolution;

void main() {
  vec2 uv = gl_FragCoord.xy / u_resolution.xy;
  vec3 color = vec3(0.1, 0.2, 0.3);
  gl_FragColor = vec4(color, 1.0);
}`