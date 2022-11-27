use web_sys::{
    HtmlCanvasElement, WebGl2RenderingContext, WebGlProgram, WebGlShader, WebGlUniformLocation,
    WebGlVertexArrayObject,
};

pub fn compile_shader(
    context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

pub fn link_program(
    context: &WebGl2RenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}

pub fn set_geometry(context: &WebGl2RenderingContext) {
    unsafe {
        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &js_sys::Float32Array::view(&[
                // left column
                0.0, 0.0, 30.0, 0.0, 0.0, 150.0, 0.0, 150.0, 30.0, 0.0, 30.0, 150.0,
                // top rung
                30.0, 0.0, 100.0, 0.0, 30.0, 30.0, 30.0, 30.0, 100.0, 0.0, 100.0, 30.0,
                // middle rung
                30.0, 60.0, 67.0, 60.0, 30.0, 90.0, 30.0, 90.0, 67.0, 60.0, 67.0, 90.0,
            ]),
            WebGl2RenderingContext::STATIC_DRAW,
        )
    }
}

// Draw the scene.
pub fn draw_scene(
    context: &WebGl2RenderingContext,
    canvas: &HtmlCanvasElement,
    program: &WebGlProgram,
    vao: Option<&WebGlVertexArrayObject>,
    resolution_uniform_location: Option<&WebGlUniformLocation>,
    color_location: Option<&WebGlUniformLocation>,
    color: &[f32],
    translation_location: Option<&WebGlUniformLocation>,
    translation: &[f32],
) {
    // Tell WebGL how to convert from clip space to pixels
    context.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);

    // Clear the canvas
    context.clear_color(1.0, 0.5, 0.0, 1.0);
    context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    // Tell it to use our program (pair of shaders)
    context.use_program(Some(&program));

    // Bind the attribute/buffer set we want.
    context.bind_vertex_array(vao);

    // Pass in the canvas resolution so we can convert from
    // pixels to clipspace in the shader
    context.uniform2f(
        resolution_uniform_location,
        canvas.width() as f32,
        canvas.height() as f32,
    );

    // Set the color.
    context.uniform4fv_with_f32_array(color_location, color);

    // Set the translation.
    context.uniform2fv_with_f32_array(translation_location, translation);

    // Draw the geometry.
    let primitive_type = WebGl2RenderingContext::TRIANGLES;
    let offset = 0;
    let count = 18;
    context.draw_arrays(primitive_type, offset, count);
}

#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}
