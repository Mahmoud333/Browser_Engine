varying vec4 v_Color; //one vector of dimension 4 inside of it, called v color that have RGB A

void main() {
    gl_FragColor = v_Color;
}