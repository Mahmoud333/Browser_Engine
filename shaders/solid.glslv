attribute vec2 a_Pos; //vector 2 called position
attribute vec3 a_Color; //vector 3 called Color
varying vec4 v_Color;   //vector four called v Color

void main() {
    //set V Color equal to vector 4 with a color inside, 1.0 alpha inside of it
    //basically means all colors we will render on our window its alpha will be 1.0
    v_Color = vec4(a_Color, 1.0);   

    //a pos inside vector 4,
    //basiaclly telling the shaders where all of our elements will be
    gl_Position = vec4(a_Pos, 0.0, 1.0); 
}