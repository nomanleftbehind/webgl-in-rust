use utils::{compile_shader, draw_scene, link_program, set_geometry};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext;

mod utils;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context = canvas
        .get_context("webgl2")?
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()?;

    let vert_shader = compile_shader(
        &context,
        WebGl2RenderingContext::VERTEX_SHADER,
        r#"#version 300 es

        // an attribute is an input (in) to a vertex shader.
        // It will receive data from a buffer
        in vec2 a_position;
        
        // Used to pass in the resolution of the canvas
        uniform vec2 u_resolution;
        
        // translation to add to position
        uniform vec2 u_translation;
        
        // all shaders have a main function
        void main() {
          // Add in the translation
          vec2 position = a_position + u_translation;
        
          // convert the position from pixels to 0.0 to 1.0
          vec2 zeroToOne = position / u_resolution;
        
          // convert from 0->1 to 0->2
          vec2 zeroToTwo = zeroToOne * 2.0;
        
          // convert from 0->2 to -1->+1 (clipspace)
          vec2 clipSpace = zeroToTwo - 1.0;
        
          gl_Position = vec4(clipSpace * vec2(1, -1), 0, 1);
        }
        "#,
    )?;

    let frag_shader = compile_shader(
        &context,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        r#"#version 300 es

        precision highp float;
        
        uniform vec4 u_color;
        
        // we need to declare an output for the fragment shader
        out vec4 outColor;
        
        void main() {
          outColor = u_color;
        }
        "#,
    )?;
    let program = link_program(&context, &vert_shader, &frag_shader)?;
    // look up where the vertex data needs to go.
    let position_attribute_location = context.get_attrib_location(&program, "a_position");

    // look up uniform locations
    let resolution_uniform_location = context.get_uniform_location(&program, "u_resolution");
    let color_location = context.get_uniform_location(&program, "u_color");
    let translation_location = context.get_uniform_location(&program, "u_translation");

    // Create a buffer
    let position_buffer = context.create_buffer();

    // Create a vertex array object (attribute state)
    let vao = context.create_vertex_array();

    // and make it the one we're currently working with
    context.bind_vertex_array(vao.as_ref());

    // Turn on the attribute
    context.enable_vertex_attrib_array(position_attribute_location as u32);

    // Bind it to ARRAY_BUFFER (think of it as ARRAY_BUFFER = positionBuffer)
    context.bind_buffer(
        WebGl2RenderingContext::ARRAY_BUFFER,
        position_buffer.as_ref(),
    );

    set_geometry(&context);

    // Tell the attribute how to get data out of positionBuffer (ARRAY_BUFFER)
    let size = 2; // 2 components per iteration
    let r#type = WebGl2RenderingContext::FLOAT; // the data is 32bit floats
    let normalize = false; // don't normalize the data
    let stride = 0; // 0 = move forward size * sizeof(type) each iteration to get the next position
    let offset = 0; // start at the beginning of the buffer
    context.vertex_attrib_pointer_with_i32(
        position_attribute_location as u32,
        size,
        r#type,
        normalize,
        stride,
        offset,
    );

    // First let's make some variables
    // to hold the translation,
    let translation = [150.0, 130.0];
    let color = [0.6, 0.3, 0.6, 1.0];

    context.use_program(Some(&program));
    context.bind_vertex_array(vao.as_ref());

    context.uniform2f(
        resolution_uniform_location.as_ref(),
        canvas.width() as f32,
        canvas.height() as f32,
    );

    draw_scene(
        &context,
        &canvas,
        &program,
        vao.as_ref(),
        resolution_uniform_location.as_ref(),
        color_location.as_ref(),
        &color,
        translation_location.as_ref(),
        &translation,
    );

    Ok(())
}
