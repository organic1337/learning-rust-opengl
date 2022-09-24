/*
In this module I'm going to draw my first triangle (super exciting :) )

Shader is a small program written in special language called GLSL (GL Shading Language)
that takes some data as input (by using the "in" keyword) and returns output by setting
some predefined variables inside its "main" function.

There are 3 programmable shaders:
I) Vertex Shader - Responsible for transforming 3D coordinates (AKA vertices) to different
    3D coordinates.
II) Geometry shader - Takes vertices as input and adds more shapes by potentially emitting more
    vertices.
III) Fragment shader - Calculate the final color of a pixel.
 */


use std::ffi::c_void;
use crate::utils::load_shader;
use crate::utils::validate_shader_program_linkage;

const FIRST_SHADER_VERTEX: &str = include_str!("shaders/vertex/first_shader.vert");
const FIRST_SHADER_FRAGMENT: &str = include_str!("shaders/fragment/first_shader.frag");


unsafe fn create_vertex_buffer_object() -> u32 {
    let vertices: [f32; 9] = [
        -0.5, -0.5, 0.0,
        0.5, -0.5, 0.0,
        0.0, 0.5, 0.0
    ];

    // Vertex Buffer Object (VBO) is the object that contains all the vertices which are
    // passed to the GPU. Moving memory from the CPU to the GPU is costly therefore we
    // should do it all at once.
    let mut vertex_buffer_object: u32 = 0;
    gl::GenBuffers(1, &mut vertex_buffer_object);
    gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer_object);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        (vertices.len() * std::mem::size_of::<f32>()) as isize,
        vertices.as_ptr() as *const c_void, gl::STATIC_DRAW);
    gl::VertexAttribPointer(
        0,
        3,
        gl::FLOAT,
        gl::FALSE,
        3 * std::mem::size_of::<f32>() as i32, std::ptr::null() as *const c_void,
    );
    gl::EnableVertexAttribArray(0);

    vertex_buffer_object
}


pub fn draw_triangle(on_loop_start: impl Fn(), on_loop_end: impl Fn()) {
    let vertex_shader = load_shader(FIRST_SHADER_VERTEX, gl::VERTEX_SHADER);
    let fragment_shader = load_shader(FIRST_SHADER_FRAGMENT, gl::FRAGMENT_SHADER);

    unsafe {
        create_vertex_buffer_object();

        // VAO - contains the attributes and the VBO to read the vertices from
        let mut vertex_array_object: u32 = 0;
        gl::GenVertexArrays(0, &mut vertex_array_object);
        gl::BindVertexArray(vertex_array_object);

        // a "program" is the product of linking all the relevant shaders together.
        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);
        validate_shader_program_linkage(shader_program);
        gl::UseProgram(shader_program);

        // Once the program is linked, we can delete the shaders
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        loop {
            on_loop_start();

            // Set the background color to blue
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(shader_program);
            gl::BindVertexArray(vertex_array_object);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);

            on_loop_end();
        }
    }
}