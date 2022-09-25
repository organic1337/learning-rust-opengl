#version 330 core

layout (location=0) vec3 aPosition;
layout (location=1) vec3 aColor;
layout (location=2) vec2 aTextureCoordinates;

out vec3 vertexColor;
out vec2 TexCoord;

void main() {
    gl_Position = vec4(aPosition, 1.0f);
    vertexColor = aColor;
    TexCoord = aTextureCoordinates;
}