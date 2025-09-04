use gbcore::cartridge::DynCartridge;
use gbcore::{Screen, System};
use gl_matrix::common::*;
use gl_matrix::mat4;
use glfw::{Context, WindowEvent};
use std::convert::TryInto;
use std::ffi::CString;
use std::fs;
use std::time::Duration;
use std::time::Instant;

const SCREEN_COLORS_DEPTH: u32 = 3;
const GB_SCREEN_WIDTH: u32 = 160;
const GB_SCREEN_HEIGHT: u32 = 144;
const WINDOW_WIDTH: u32 = GB_SCREEN_WIDTH * 3;
const WINDOW_HEIGHT: u32 = GB_SCREEN_HEIGHT * 3;
const FRAME_BUFFER_SIZE: u32 = GB_SCREEN_WIDTH * GB_SCREEN_HEIGHT * SCREEN_COLORS_DEPTH;
const TITLE: &str = "GBRUST";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let rom_file = args.get(1).unwrap();
    let rom_data = fs::read(rom_file).unwrap();

    let dyn_cart = DynCartridge::new(rom_data).unwrap();

    println!("Cart type: {:?}", dyn_cart.cart_type);
    println!(
        "ROM type: {:?} ({} banks)",
        dyn_cart.rom_type,
        dyn_cart.rom_type.nb_bank(),
    );
    println!(
        "RAM type: {:?} ({} banks)",
        dyn_cart.ram_type,
        dyn_cart.ram_type.nb_bank(),
    );

    let mut screen = Screen::default();
    let mut sys = System::new(dyn_cart);
    ////////////////////////////////////////////////////////////////////////

    let mut glfw = glfw::init_no_callbacks().unwrap();
    glfw.window_hint(glfw::WindowHint::Resizable(true));
    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    let (mut window, events) = glfw
        .create_window(
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
            TITLE,
            glfw::WindowMode::Windowed,
        )
        .unwrap();
    let (screen_width, screen_height) = window.get_framebuffer_size();

    window.make_current();
    window.set_key_polling(true);
    gl::load_with(|ptr| window.get_proc_address(ptr).unwrap() as *const _);

    unsafe {
        gl::Viewport(0, 0, screen_width, screen_height);
    }
    // -------------------------------------------

    const VERT_SHADER: &str = "#version 330

    uniform mat4 projection;
    
    in vec3 vert;
    in vec2 vertTexCoord;
    
    out vec2 fragTexCoord;
    
    void main() {
        fragTexCoord = vertTexCoord;
        gl_Position = projection * vec4(vert, 1);
    }";

    const FRAG_SHADER: &str = "#version 330
    
    uniform sampler2D tex;
    
    in vec2 fragTexCoord;
    
    out vec4 outputColor;
    
    // Magic matrix
    const mat3 GBCMatrix = mat3( 0.924, 0.021, 0.013, 0.048, 0.787, 0.249, 0.104, 0.09, 0.733 );
    const float gamma = 2.2;
    
    void main() {
        // Apply Color Palette
        outputColor = texture(tex, fragTexCoord);
    
        // Color Correction
        outputColor.rgb = pow(outputColor.rgb, vec3(gamma));
        vec3 Picture = outputColor.xyz;
        // Picture *= Picture;
        Picture *= GBCMatrix;
        // Picture = sqrt(Picture);
        outputColor = vec4(Picture,1.0);
        outputColor.rgb = pow(outputColor.rgb, vec3(1/gamma));
        
    
    }";

    // Init GLOW
    let shader_program = new_gl_program(VERT_SHADER, FRAG_SHADER);
    let mut vao = 0;
    let mut vbo = 0;
    let gb_texture;

    unsafe {
        // Configure the vertex and fragment shaders

        gl::UseProgram(shader_program);

        let mut projection: Mat4 = [0.; 16];
        mat4::ortho(&mut projection, -1.0, 1.0, 1.0, -1.0, 1.0, -1.0);

        // Configure matrix
        let projection_uniform =
            gl::GetUniformLocation(shader_program, CString::new("projection").unwrap().as_ptr());
        gl::UniformMatrix4fv(projection_uniform, 1, gl::FALSE, projection.as_ptr());

        // Prepare texture locations
        let texture_uniform =
            gl::GetUniformLocation(shader_program, CString::new("tex").unwrap().as_ptr());
        gl::Uniform1i(texture_uniform, 0);
        gl::BindFragDataLocation(
            shader_program,
            0,
            CString::new("outputColor").unwrap().as_ptr(),
        );
        gb_texture = new_gb_texture();

        // Configure the vertex data
        let vertecies = [
            -1.0f32, -1.0, 1.0, 0.0, 0.0, 1.0, -1.0, 1.0, 1.0, 0.0, -1.0, 1.0, 1.0, 0.0, 1.0, 1.0,
            -1.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 0.0, 1.0,
        ];

        // vao
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // vbo
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            std::mem::size_of_val(&vertecies) as isize,
            vertecies.as_ptr().cast(),
            gl::STATIC_DRAW,
        );

        let vert_attrib: u32 =
            gl::GetAttribLocation(shader_program, CString::new("vert").unwrap().as_ptr()) as u32;
        gl::EnableVertexAttribArray(vert_attrib);
        gl::VertexAttribPointer(
            vert_attrib,
            3,
            gl::FLOAT,
            gl::FALSE,
            5 * std::mem::size_of::<f32>() as i32,
            std::ptr::null(),
        );

        let tex_coord_attrib: u32 = gl::GetAttribLocation(
            shader_program,
            CString::new("vertTexCoord").unwrap().as_ptr(),
        ) as u32;
        gl::EnableVertexAttribArray(tex_coord_attrib);
        gl::VertexAttribPointer(
            tex_coord_attrib,
            2,
            gl::FLOAT,
            gl::FALSE,
            5 * std::mem::size_of::<f32>() as i32,
            (3 * 4) as *const _,
        );

        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
    }

    // -------------------------------------------

    glfw.set_swap_interval(glfw::SwapInterval::None);

    const TARGET_FPS: f32 = 59.7;
    const F_DURATION: f32 = 1.0 / TARGET_FPS;
    let frame_duration: Duration = Duration::from_secs_f32(F_DURATION);
    let mut start_frame: Instant;

    let mut nb_frames = 0;
    let mut last_update = Instant::now();
    const FPS_UPDATE_RATE: Duration = Duration::from_millis(1000);
    let mut keys: u8 = 0;
    while !window.should_close() {
        start_frame = Instant::now();
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            if let WindowEvent::Key(key, _, action, _) = event {
                match action {
                    glfw::Action::Press => match key {
                        glfw::Key::Up => keys |= gbcore::KEY_A,     // A
                        glfw::Key::Left => keys |= gbcore::KEY_B,   // B
                        glfw::Key::O => keys |= gbcore::KEY_SELECT, // SELECT
                        glfw::Key::P => keys |= gbcore::KEY_START,  // START
                        glfw::Key::D => keys |= gbcore::KEY_RIGHT,  // RIGHT
                        glfw::Key::A => keys |= gbcore::KEY_LEFT,   // LEFT
                        glfw::Key::W => keys |= gbcore::KEY_UP,     // UP
                        glfw::Key::S => keys |= gbcore::KEY_DOWN,   // DOWN
                        _ => {}
                    },
                    glfw::Action::Release => match key {
                        glfw::Key::Up => keys &= !gbcore::KEY_A,     // A
                        glfw::Key::Left => keys &= !gbcore::KEY_B,   // B
                        glfw::Key::O => keys &= !gbcore::KEY_SELECT, // SELECT
                        glfw::Key::P => keys &= !gbcore::KEY_START,  // START
                        glfw::Key::D => keys &= !gbcore::KEY_RIGHT,  // RIGHT
                        glfw::Key::A => keys &= !gbcore::KEY_LEFT,   // LEFT
                        glfw::Key::W => keys &= !gbcore::KEY_UP,     // UP
                        glfw::Key::S => keys &= !gbcore::KEY_DOWN,   // DOWN
                        _ => {}
                    },

                    _ => {}
                }
            }
        }

        sys.tick(&mut screen, &keys);

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            gl::UseProgram(shader_program);
            gl::BindVertexArray(vao);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::ActiveTexture(gl::TEXTURE1);

            // C'est ici que tout ce passe !!!
            gl::BindTexture(gl::TEXTURE_2D, gb_texture);
            gl::TexSubImage2D(
                gl::TEXTURE_2D,
                0,
                0,
                0,
                GB_SCREEN_WIDTH as i32,
                GB_SCREEN_HEIGHT as i32,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                screen.frame_buffer.as_ptr() as _,
            );

            gl::DrawArrays(gl::TRIANGLES, 0, 6 * 2 * 3);

            gl::BindVertexArray(0);
            window.swap_buffers();
        }
        nb_frames += 1;
        if Instant::now() - last_update >= FPS_UPDATE_RATE {
            let fps = nb_frames as f32 / FPS_UPDATE_RATE.as_secs_f32();
            window.set_title(&format!("{} - FPS: {}", TITLE, fps));
            nb_frames = 0;
            last_update = Instant::now();
        }

        let elapsed = Instant::now() - start_frame;
        if elapsed < frame_duration {
            std::thread::sleep(frame_duration - elapsed);
        }
    }
}

pub fn gl_get_string<'a>(name: gl::types::GLenum) -> &'a str {
    let v = unsafe { gl::GetString(name) };
    let v: &std::ffi::CStr = unsafe { std::ffi::CStr::from_ptr(v as *const i8) };
    v.to_str().unwrap()
}

fn new_gl_program(vertex_shader_source: &str, fragment_shader_source: &str) -> u32 {
    let vertex_shader = compile_glsl(vertex_shader_source, gl::VERTEX_SHADER);
    let fragment_shader = compile_glsl(fragment_shader_source, gl::FRAGMENT_SHADER);

    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
        gl::LinkProgram(program);

        let mut status = 0;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);
        if status == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            gl::GetProgramInfoLog(program, 1024, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());
            panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
        }

        gl::DetachShader(program, vertex_shader);
        gl::DetachShader(program, fragment_shader);
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        program
    }
}

fn compile_glsl(source: &str, shader_type: u32) -> u32 {
    unsafe {
        let shader = gl::CreateShader(shader_type);

        gl::ShaderSource(
            shader,
            1,
            &source.as_bytes().as_ptr().cast(),
            &source.len().try_into().unwrap(),
        );
        gl::CompileShader(shader);

        let mut status = 0;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
        if status == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            gl::GetShaderInfoLog(shader, 1024, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());
            panic!(
                "Fragment Shader Compile Error: {}",
                String::from_utf8_lossy(&v)
            );
        }

        shader
    }
}

fn new_gb_texture() -> u32 {
    let empty_texture_data = [0_u8; (FRAME_BUFFER_SIZE) as usize];
    let mut texture: u32 = 0;
    unsafe {
        gl::GenTextures(1, &mut texture);
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, texture);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as i32,
            GB_SCREEN_WIDTH as i32,
            GB_SCREEN_HEIGHT as i32,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            empty_texture_data.as_ptr() as _,
        );
    }
    texture
}
