use web_sys::WebGlRenderingContext;

pub fn init_shader(
    ctx: &web_sys::WebGlRenderingContext,
    shader_type: u32,
    shader_src: &str,
) -> Result<web_sys::WebGlShader, String> {
    let shader = ctx
        .create_shader(shader_type)
        .ok_or("failed to create shader")?;

    ctx.shader_source(&shader, shader_src);
    ctx.compile_shader(&shader);

    if !ctx.get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS) {
        let info = ctx
            .get_shader_info_log(&shader)
            .ok_or("failed to get log")?;
        return Err(format!("failed to compile shader: {info}"));
    }

    Ok(shader)
}

pub fn init_program(
    ctx: &web_sys::WebGlRenderingContext,
    shaders: [web_sys::WebGlShader; 2],
) -> Result<web_sys::WebGlProgram, String> {
    let program = ctx
        .create_program()
        .ok_or("failed to create webgl program")?;

    for shader in shaders {
        ctx.attach_shader(&program, &shader);
    }

    ctx.link_program(&program);

    if !ctx.get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS) {
        let info = ctx
            .get_program_info_log(&program)
            .ok_or("failed to get log")?;
        return Err(format!("failed to link program: {info}"));
    }

    Ok(program)
}

pub fn get_attribute_location(
    ctx: &web_sys::WebGlRenderingContext,
    program: &web_sys::WebGlProgram,
    name: &str,
) -> Result<u32, String> {
    let loc = ctx.get_attrib_location(program, name);
    if loc == -1 {
        return Err(format!("failed to get attribute location: {name}"));
    }
    Ok(loc as u32)
}

pub fn get_uniform_location(
    ctx: &web_sys::WebGlRenderingContext,
    program: &web_sys::WebGlProgram,
    name: &str,
) -> Result<web_sys::WebGlUniformLocation, String> {
    ctx.get_uniform_location(program, name)
        .ok_or_else(|| panic!("failed to get uniform location: {name}"))
}
