use raylib::{
    ffi::{GuiGetFont, GuiSetFont},
    prelude::*,
};
use std::{borrow::BorrowMut, mem::swap};

macro_rules! gui_slider {
    ($d:expr, $shader:expr, $dt_loc:expr, $dt:expr, $x:expr, $y:expr, $width:expr, $height:expr, $min:expr, $max:expr) => {
        let gui_label_bounds = Rectangle::new($x - 80.0, $y, 80.0, $height);
        $d.gui_label(
            gui_label_bounds,
            Some(std::ffi::CString::new(stringify!($dt)).unwrap().as_c_str()),
        );
        let bounds = Rectangle::new($x, $y, $width, $height);
        // let name = format!("{}", $dt).to_string();
        $dt = $d.gui_slider(
            bounds,
            Some(rstr!($min)),
            Some(rstr!($max)),
            $dt,
            $min,
            $max,
        );
        $shader.set_shader_value($dt_loc, $dt);
    };
}

pub fn run() {
    let screen_scale = 100;
    let screen_width = 16 * screen_scale;
    let screen_height = 9 * screen_scale;
    let fps = 60;

    let texture_scale = 0.5;
    let texture_width = screen_width as f32 * texture_scale;
    let texture_height = screen_height as f32 * texture_scale;

    let (mut rl, thread) = raylib::init()
        .size(screen_width, screen_height)
        .title("Smooth Life")
        .build();
    rl.set_target_fps(fps);

    // let image = Image::gen_image_white_noise(texture_width as i32, texture_height as i32, 0.69f32);
    // let image =
    //     Image::gen_image_perlin_noise(texture_width as i32, texture_height as i32, 0, 0, 5f32);
    let image = Image::gen_image_cellular(
        texture_width as i32,
        texture_height as i32,
        (texture_height / 6.0) as i32,
    );

    // let texture = rl.load_texture_from_image(&thread, &image);

    let mut state0 = rl
        .load_render_texture(&thread, texture_width as u32, texture_height as u32)
        .expect("could not load render texture in state0");
    let pixels: Vec<u8> = image
        .get_image_data()
        .iter()
        .flat_map(|c| vec![c.r, c.g, c.b, c.a])
        .collect();
    state0.update_texture(pixels.as_slice());
    state0.set_texture_wrap(&thread, TextureWrap::TEXTURE_WRAP_REPEAT);
    state0.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_BILINEAR);
    // state0.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_ANISOTROPIC_16X);
    // state0.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_POINT);

    let mut state1 = rl
        .load_render_texture(&thread, texture_width as u32, texture_height as u32)
        .expect("could not load render texture in state1");
    state1.set_texture_wrap(&thread, TextureWrap::TEXTURE_WRAP_REPEAT);
    state1.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_BILINEAR);
    // state1.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_ANISOTROPIC_16X);
    // state1.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_POINT);

    // let mut state = vec![texture0, texture1];
    // println!("{:?}\n-{:?}", state.0, state.1);

    let mut shader = rl
        .load_shader(&thread, None, Some("src/shaders/smooth_life.fs"))
        .expect("could not load shader");
    let resolution = [texture_width as f32, texture_height as f32];
    let resolution_loc = shader.get_shader_location("resolution");
    shader.set_shader_value(resolution_loc, resolution);

    let ra_loc = shader.get_shader_location("ra");
    let ri_loc = shader.get_shader_location("ri");
    let b1_loc = shader.get_shader_location("b1");
    let b2_loc = shader.get_shader_location("b2");
    let d1_loc = shader.get_shader_location("d1");
    let d2_loc = shader.get_shader_location("d2");
    let alpha_n_loc = shader.get_shader_location("alpha_n");
    let alpha_m_loc = shader.get_shader_location("alpha_m");
    let dt_loc = shader.get_shader_location("dt");

    let mut ra = 12.0;
    let mut ri = 4.0;
    let mut b1 = 0.278;
    let mut b2 = 0.365;
    let mut d1 = 0.267;
    let mut d2 = 0.445;
    let mut alpha_n = 0.028;
    let mut alpha_m = 0.147;
    let mut dt = 6.9;

    let mut stop = false;

    while !rl.window_should_close() {
        {
            // begin texture mode
            let mut td = rl.borrow_mut();
            let mut td = td.begin_texture_mode(&thread, &mut state1);
            td.clear_background(Color::BLACK);
            {
                // begin shader mode
                let mut sd = td.begin_shader_mode(&shader);
                let statei_ref = &state0;
                sd.draw_texture(statei_ref, 0, 0, Color::WHITE);
            }
        }

        let mut d = rl.begin_drawing(&thread);

        // {
        d.clear_background(Color::BLACK);
        // d.draw_texture(&state1, 0, 0, Color::WHITE);
        d.draw_texture_ex(
            &state1,
            Vector2::new(0f32, 0f32),
            0f32,
            1.0 / texture_scale,
            Color::WHITE,
        );

        // let state = state0; state0 = state1; state1 = state;
        let key_pressed = d.is_key_pressed(KeyboardKey::KEY_SPACE);
        if key_pressed {
            stop = !stop;
            println!("stop: {}", stop);
        }
        if !stop {
            swap(&mut state0, &mut state1);
        }
        if d.is_key_pressed(KeyboardKey::KEY_T) {
            state0.update_texture(pixels.as_slice());
        }
        if d.is_key_pressed(KeyboardKey::KEY_R) {
            ra = 10.0;
            b1 = 0.278;
            b2 = 0.365;
            d1 = 0.267;
            d2 = 0.445;
            alpha_n = 0.028;
            alpha_m = 0.147;
            dt = 6.9;
        }

        // let mut d: RaylibDrawHandle<'_> = drop(&mut sd);
        d.draw_text("smooth life", 12, 12, 20, Color::SKYBLUE);
        // }

        //
        // let bounds = Rectangle::new(120.0, 120.0, 250.0, 30.0);
        // dt = d.gui_slider(
        //     bounds,
        //     Some(rstr!("0.0")),
        //     Some(rstr!("10.0")),
        //     dt,
        //     0.0,
        //     10.0,
        // );
        // // d.gui_button(bounds, Some(rstr!("SCROLLPANEL STYLE")));
        // shader.set_shader_value(dt_loc, dt);
        // // d.gui_enable();

        // d.gui_set_font();
        let mut style = d.gui_get_font();
        style.baseSize = 50;
        d.gui_set_font(style);
        // println!("{}", d.gui_get_font().base_size());

        let mut font = unsafe { GuiGetFont() };
        font.baseSize = 30;
        unsafe {
            GuiSetFont(font);
        }

        gui_slider!(d, shader, ra_loc, ra, 120.0, 120.0, 690.0, 30.0, 0.0, 20.0);
        gui_slider!(d, shader, ri_loc, ri, 120.0, 160.0, 690.0, 30.0, 0.0, 20.0);
        gui_slider!(d, shader, b1_loc, b1, 120.0, 200.0, 690.0, 30.0, 0.0, 1.0);
        gui_slider!(d, shader, b2_loc, b2, 120.0, 240.0, 690.0, 30.0, 0.0, 1.0);
        gui_slider!(d, shader, d1_loc, d1, 120.0, 280.0, 690.0, 30.0, 0.0, 1.0);
        gui_slider!(d, shader, d2_loc, d2, 120.0, 320.0, 690.0, 30.0, 0.0, 1.0);
        #[rustfmt::skip]
        gui_slider!(d, shader, alpha_n_loc, alpha_n, 120.0, 360.0, 690.0, 30.0, 0.0, 1.0);
        #[rustfmt::skip]
        gui_slider!(d, shader, alpha_m_loc, alpha_m, 120.0, 400.0, 690.0, 30.0, 0.0, 1.0);
        gui_slider!(d, shader, dt_loc, dt, 120.0, 440.0, 690.0, 30.0, 0.0, 10.0);

        // d.gui_button(
        //     Rectangle::new(120.0, 600.0, 150.0, 30.0),
        //     Some(rstr!("reset")),
        // );

        // d.gui_enable();
    }
}
