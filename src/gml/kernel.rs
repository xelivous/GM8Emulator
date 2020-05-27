// This file was auto-generated based on a function table dump

#![allow(unused_macros)]

use crate::{
    asset,
    game::{draw, Game, GetAsset},
    gml::{self, file, Context, Value},
    input::MouseButton,
    instance::{DummyFieldHolder, Instance},
    util,
};
use std::convert::TryFrom;

macro_rules! _arg_into {
    (any, $v: expr) => {{ Ok($v.clone()) }};
    (int, $v: expr) => {{ Ok(<Value as Into<i32>>::into($v.clone())) }};
    (real, $v: expr) => {{ Ok(<Value as Into<f64>>::into($v.clone())) }};
    (string, $v: expr) => {{ Ok(<Value as Into<std::rc::Rc<str>>>::into($v.clone())) }};
}

macro_rules! _count_rep {
    () => { 0usize };
    ($x: ident $($ys: ident)*) => { 1usize + _count_rep!($($ys)*) };
}

include!(concat!(env!("OUT_DIR"), "/_apply_args.macro.rs"));

/// Helper macro to validate input arguments from a GML function.
macro_rules! expect_args {
    ($args: expr, [$($x: ident),*]) => {{
        (|| -> gml::Result<_> {
            let argc = _count_rep!($($x)*);
            if $args.len() != argc {
                Err(gml::runtime::Error::WrongArgumentCount(argc, $args.len()))
            } else {
                _apply_args!($args, $($x)*)
            }
        })()
    }};
    ($args: expr, [$($x: ident,)*]) => { expect_args!($args, $($x),*) };
}

impl Game {
    pub fn display_get_width(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function display_get_width")
    }

    pub fn display_get_height(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function display_get_height")
    }

    pub fn display_get_colordepth(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function display_get_colordepth")
    }

    pub fn display_get_frequency(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function display_get_frequency")
    }

    pub fn display_set_size(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function display_set_size")
    }

    pub fn display_set_colordepth(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function display_set_colordepth")
    }

    pub fn display_set_frequency(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function display_set_frequency")
    }

    pub fn display_set_all(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        unimplemented!("Called unimplemented kernel function display_set_all")
    }

    pub fn display_test_all(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        unimplemented!("Called unimplemented kernel function display_test_all")
    }

    pub fn display_reset(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        unimplemented!("Called unimplemented kernel function display_reset")
    }

    pub fn display_mouse_get_x(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function display_mouse_get_x")
    }

    pub fn display_mouse_get_y(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function display_mouse_get_y")
    }

    pub fn display_mouse_set(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function display_mouse_set")
    }

    pub fn window_set_visible(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let visible = expect_args!(args, [any])?;
        self.window.set_visible(visible.is_truthy());
        Ok(Default::default())
    }

    pub fn window_get_visible(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        Ok(self.window.get_visible().into())
    }

    pub fn window_set_fullscreen(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function window_set_fullscreen")
    }

    pub fn window_get_fullscreen(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function window_get_fullscreen")
    }

    pub fn window_set_showborder(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function window_set_showborder")
    }

    pub fn window_get_showborder(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function window_get_showborder")
    }

    pub fn window_set_showicons(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function window_set_showicons")
    }

    pub fn window_get_showicons(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function window_get_showicons")
    }

    pub fn window_set_stayontop(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function window_set_stayontop")
    }

    pub fn window_get_stayontop(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function window_get_stayontop")
    }

    pub fn window_set_sizeable(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function window_set_sizeable")
    }

    pub fn window_get_sizeable(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function window_get_sizeable")
    }

    pub fn window_set_caption(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let caption = expect_args!(args, [string])?;
        self.window.set_title(&caption);
        Ok(Default::default())
    }

    pub fn window_get_caption(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        Ok(self.window.get_title().to_owned().into())
    }

    pub fn window_set_cursor(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function window_set_cursor")
    }

    pub fn window_get_cursor(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function window_get_cursor")
    }

    pub fn window_set_color(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function window_set_color")
    }

    pub fn window_get_color(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function window_get_color")
    }

    pub fn window_set_position(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function window_set_position")
    }

    pub fn window_set_size(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (width, height) = expect_args!(args, [int, int])?;
        self.window.resize(width as _, height as _);
        Ok(Default::default())
    }

    pub fn window_set_rectangle(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        unimplemented!("Called unimplemented kernel function window_set_rectangle")
    }

    pub fn window_center(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function window_center")
    }

    pub fn window_default(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function window_default")
    }

    pub fn window_get_x(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function window_get_x")
    }

    pub fn window_get_y(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function window_get_y")
    }

    pub fn window_get_width(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function window_get_width")
    }

    pub fn window_get_height(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function window_get_height")
    }

    pub fn window_set_region_size(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function window_set_region_size")
    }

    pub fn window_get_region_width(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function window_get_region_width")
    }

    pub fn window_get_region_height(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function window_get_region_height")
    }

    pub fn window_set_region_scale(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function window_set_region_scale")
    }

    pub fn window_get_region_scale(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function window_get_region_scale")
    }

    pub fn window_mouse_get_x(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function window_mouse_get_x")
    }

    pub fn window_mouse_get_y(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function window_mouse_get_y")
    }

    pub fn window_mouse_set(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function window_mouse_set")
    }

    pub fn window_view_mouse_get_x(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function window_view_mouse_get_x")
    }

    pub fn window_view_mouse_get_y(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function window_view_mouse_get_y")
    }

    pub fn window_view_mouse_set(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function window_view_mouse_set")
    }

    pub fn window_views_mouse_get_x(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function window_views_mouse_get_x")
    }

    pub fn window_views_mouse_get_y(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function window_views_mouse_get_y")
    }

    pub fn window_views_mouse_set(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function window_views_mouse_set")
    }

    pub fn set_synchronization(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function set_synchronization")
    }

    pub fn set_automatic_draw(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function set_automatic_draw")
    }

    pub fn screen_redraw(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function screen_redraw")
    }

    pub fn screen_refresh(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function screen_refresh")
    }

    pub fn screen_wait_vsync(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function screen_wait_vsync")
    }

    pub fn screen_save(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function screen_save")
    }

    pub fn screen_save_part(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        unimplemented!("Called unimplemented kernel function screen_save_part")
    }

    pub fn draw_getpixel(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function draw_getpixel")
    }

    pub fn draw_set_color(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let col = expect_args!(args, [int])?;
        self.draw_colour = (col as u32).into();
        Ok(Default::default())
    }

    pub fn draw_set_alpha(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        self.draw_alpha = expect_args!(args, [real])?;
        Ok(Default::default())
    }

    pub fn draw_get_color(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        Ok(u32::from(self.draw_colour).into())
    }

    pub fn draw_get_alpha(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        Ok(self.draw_alpha.into())
    }

    pub fn make_color(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [int, int, int]).map(|(r, g, b)| r + (g * 256) + (b * 256 * 256)).map(Value::from)
    }

    pub fn make_color_rgb(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [int, int, int]).map(|(r, g, b)| r + (g * 256) + (b * 256 * 256)).map(Value::from)
    }

    pub fn make_color_hsv(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (h, s, v) = expect_args!(args, [real, real, real])?;
        let h = h * 360.0 / 255.0;
        let s = s / 255.0;
        let v = v / 255.0;
        let chroma = v * s;
        let hprime = (h / 60.0) % 6.0;
        let x = chroma * (1.0 - ((hprime % 2.0) - 1.0).abs());
        let m = v - chroma;

        let (r, g, b) = match hprime.floor() as i32 {
            0 => (chroma, x, 0.0),
            1 => (x, chroma, 0.0),
            2 => (0.0, chroma, x),
            3 => (0.0, x, chroma),
            4 => (x, 0.0, chroma),
            5 => (chroma, 0.0, x),
            _ => (0.0, 0.0, 0.0),
        };

        let out_r = util::ieee_round((r + m) * 255.0);
        let out_g = util::ieee_round((g + m) * 255.0);
        let out_b = util::ieee_round((b + m) * 255.0);
        Ok((out_r | (out_g << 8) | (out_b << 16)).into())
    }

    pub fn color_get_red(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function color_get_red")
    }

    pub fn color_get_green(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function color_get_green")
    }

    pub fn color_get_blue(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function color_get_blue")
    }

    pub fn color_get_hue(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function color_get_hue")
    }

    pub fn color_get_saturation(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function color_get_saturation")
    }

    pub fn color_get_value(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function color_get_value")
    }

    pub fn merge_color(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function merge_color")
    }

    pub fn draw_set_blend_mode(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function draw_set_blend_mode")
    }

    pub fn draw_set_blend_mode_ext(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function draw_set_blend_mode_ext")
    }

    pub fn draw_clear(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function draw_clear")
    }

    pub fn draw_clear_alpha(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function draw_clear_alpha")
    }

    pub fn draw_point(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function draw_point")
    }

    pub fn draw_line(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        unimplemented!("Called unimplemented kernel function draw_line")
    }

    pub fn draw_line_width(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        unimplemented!("Called unimplemented kernel function draw_line_width")
    }

    pub fn draw_rectangle(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        unimplemented!("Called unimplemented kernel function draw_rectangle")
    }

    pub fn draw_roundrect(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        unimplemented!("Called unimplemented kernel function draw_roundrect")
    }

    pub fn draw_triangle(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 7
        unimplemented!("Called unimplemented kernel function draw_triangle")
    }

    pub fn draw_circle(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        unimplemented!("Called unimplemented kernel function draw_circle")
    }

    pub fn draw_ellipse(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        unimplemented!("Called unimplemented kernel function draw_ellipse")
    }

    pub fn draw_arrow(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        unimplemented!("Called unimplemented kernel function draw_arrow")
    }

    pub fn draw_button(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        unimplemented!("Called unimplemented kernel function draw_button")
    }

    pub fn draw_healthbar(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 11
        unimplemented!("Called unimplemented kernel function draw_healthbar")
    }

    pub fn draw_path(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        unimplemented!("Called unimplemented kernel function draw_path")
    }

    pub fn draw_point_color(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function draw_point_color")
    }

    pub fn draw_line_color(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        unimplemented!("Called unimplemented kernel function draw_line_color")
    }

    pub fn draw_line_width_color(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 7
        unimplemented!("Called unimplemented kernel function draw_line_width_color")
    }

    pub fn draw_rectangle_color(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 9
        unimplemented!("Called unimplemented kernel function draw_rectangle_color")
    }

    pub fn draw_roundrect_color(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 7
        unimplemented!("Called unimplemented kernel function draw_roundrect_color")
    }

    pub fn draw_triangle_color(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 10
        unimplemented!("Called unimplemented kernel function draw_triangle_color")
    }

    pub fn draw_circle_color(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        unimplemented!("Called unimplemented kernel function draw_circle_color")
    }

    pub fn draw_ellipse_color(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 7
        unimplemented!("Called unimplemented kernel function draw_ellipse_color")
    }

    pub fn draw_set_circle_precision(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function draw_set_circle_precision")
    }

    pub fn draw_primitive_begin(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function draw_primitive_begin")
    }

    pub fn draw_primitive_begin_texture(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function draw_primitive_begin_texture")
    }

    pub fn draw_primitive_end(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function draw_primitive_end")
    }

    pub fn draw_vertex(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function draw_vertex")
    }

    pub fn draw_vertex_color(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        unimplemented!("Called unimplemented kernel function draw_vertex_color")
    }

    pub fn draw_vertex_texture(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        unimplemented!("Called unimplemented kernel function draw_vertex_texture")
    }

    pub fn draw_vertex_texture_color(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        unimplemented!("Called unimplemented kernel function draw_vertex_texture_color")
    }

    pub fn sprite_get_texture(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function sprite_get_texture")
    }

    pub fn background_get_texture(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function background_get_texture")
    }

    pub fn texture_exists(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function texture_exists")
    }

    pub fn texture_set_interpolation(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function texture_set_interpolation")
    }

    pub fn texture_set_blending(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function texture_set_blending")
    }

    pub fn texture_set_repeat(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function texture_set_repeat")
    }

    pub fn texture_get_width(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function texture_get_width")
    }

    pub fn texture_get_height(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function texture_get_height")
    }

    pub fn texture_preload(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function texture_preload")
    }

    pub fn texture_set_priority(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function texture_set_priority")
    }

    pub fn draw_set_font(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let font_id = expect_args!(args, [int])?;
        if self.draw_font_id != font_id {
            self.draw_font = self.assets.fonts.get_asset(font_id).map(|x| x.as_ref().clone());
            self.draw_font_id = font_id;
        }
        Ok(Default::default())
    }

    pub fn draw_set_halign(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        self.draw_halign = match expect_args!(args, [int])? {
            0 => draw::Halign::Left,
            1 => draw::Halign::Middle,
            2 | _ => draw::Halign::Right,
        };
        Ok(Default::default())
    }

    pub fn draw_set_valign(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        self.draw_valign = match expect_args!(args, [int])? {
            0 => draw::Valign::Top,
            1 => draw::Valign::Middle,
            2 | _ => draw::Valign::Bottom,
        };
        Ok(Default::default())
    }

    pub fn string_width(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let string = expect_args!(args, [string])?;
        let (width, _) = self.get_string_size(&string, None, None);
        Ok(width.into())
    }

    pub fn string_height(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let string = expect_args!(args, [string])?;
        let (_, height) = self.get_string_size(&string, None, None);
        Ok(height.into())
    }

    pub fn string_width_ext(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (string, line_height, max_width) = expect_args!(args, [string, int, int])?;
        let (width, _) = self.get_string_size(
            &string,
            if line_height < 0 { None } else { Some(line_height as _) },
            if max_width < 0 { None } else { Some(max_width as _) },
        );
        Ok(width.into())
    }

    pub fn string_height_ext(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (string, line_height, max_width) = expect_args!(args, [string, int, int])?;
        let (_, height) = self.get_string_size(
            &string,
            if line_height < 0 { None } else { Some(line_height as _) },
            if max_width < 0 { None } else { Some(max_width as _) },
        );
        Ok(height.into())
    }

    pub fn draw_text(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y, text) = expect_args!(args, [int, int, any])?;
        match text {
            Value::Real(r) if r.fract() == 0.0 => self.draw_string(x, y, &format!("{:.0}", r), None, None),
            Value::Real(r) => self.draw_string(x, y, &format!("{:.2}", r), None, None),
            Value::Str(string) => self.draw_string(x, y, &string, None, None),
        }
        Ok(Default::default())
    }

    pub fn draw_text_ext(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y, text, line_height, max_width) = expect_args!(args, [int, int, any, int, int])?;
        let line_height = if line_height < 0 { None } else { Some(line_height as _) };
        let max_width = if max_width < 0 { None } else { Some(max_width as _) };

        match text {
            Value::Real(r) if r.fract() == 0.0 => self.draw_string(x, y, &format!("{:.0}", r), line_height, max_width),
            Value::Real(r) => self.draw_string(x, y, &format!("{:.2}", r), line_height, max_width),
            Value::Str(string) => self.draw_string(x, y, &string, line_height, max_width),
        }
        Ok(Default::default())
    }

    pub fn draw_text_transformed(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        unimplemented!("Called unimplemented kernel function draw_text_transformed")
    }

    pub fn draw_text_ext_transformed(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 8
        unimplemented!("Called unimplemented kernel function draw_text_ext_transformed")
    }

    pub fn draw_text_color(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 8
        unimplemented!("Called unimplemented kernel function draw_text_color")
    }

    pub fn draw_text_transformed_color(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 11
        unimplemented!("Called unimplemented kernel function draw_text_transformed_color")
    }

    pub fn draw_text_ext_color(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 10
        unimplemented!("Called unimplemented kernel function draw_text_ext_color")
    }

    pub fn draw_text_ext_transformed_color(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 13
        unimplemented!("Called unimplemented kernel function draw_text_ext_transformed_color")
    }

    pub fn draw_self(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        let instance = self.instance_list.get(context.this);
        if let Some(sprite) = self.assets.sprites.get_asset(instance.sprite_index.get()) {
            let image_index = instance.image_index.get().floor() as i32 % sprite.frames.len() as i32;
            if let Some(atlas_ref) = sprite.frames.get(image_index as usize).map(|x| &x.atlas_ref) {
                self.renderer.draw_sprite(
                    atlas_ref,
                    util::ieee_round(instance.x.get()),
                    util::ieee_round(instance.y.get()),
                    instance.image_xscale.get(),
                    instance.image_yscale.get(),
                    instance.image_angle.get(),
                    instance.image_blend.get(),
                    instance.image_alpha.get(),
                );
            }
            Ok(Default::default())
        } else {
            Err(gml::Error::NonexistentAsset(asset::Type::Sprite, instance.sprite_index.get()))
        }
    }

    pub fn draw_sprite(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (sprite_index, image_index, x, y) = expect_args!(args, [int, real, int, int])?;
        let instance = self.instance_list.get(context.this);
        if let Some(sprite) = self.assets.sprites.get_asset(sprite_index) {
            let image_index = if image_index < 0.0 { instance.image_index.get() } else { image_index };
            if let Some(atlas_ref) =
                sprite.frames.get(image_index.floor() as usize % sprite.frames.len()).map(|x| &x.atlas_ref)
            {
                self.renderer.draw_sprite(
                    atlas_ref,
                    x,
                    y,
                    instance.image_xscale.get(),
                    instance.image_yscale.get(),
                    instance.image_angle.get(),
                    instance.image_blend.get(),
                    instance.image_alpha.get(),
                );
            }
            Ok(Default::default())
        } else {
            Err(gml::Error::NonexistentAsset(asset::Type::Sprite, sprite_index))
        }
    }

    pub fn draw_sprite_pos(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 11
        unimplemented!("Called unimplemented kernel function draw_sprite_pos")
    }

    pub fn draw_sprite_ext(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (sprite_index, image_index, x, y, xscale, yscale, angle, colour, alpha) =
            expect_args!(args, [int, real, int, int, real, real, real, int, real])?;
        if let Some(sprite) = self.assets.sprites.get_asset(sprite_index) {
            let image_index =
                if image_index < 0.0 { self.instance_list.get(context.this).image_index.get() } else { image_index };
            if let Some(atlas_ref) =
                sprite.frames.get(image_index.floor() as usize % sprite.frames.len()).map(|x| &x.atlas_ref)
            {
                self.renderer.draw_sprite(atlas_ref, x, y, xscale, yscale, angle, colour, alpha);
            }
            Ok(Default::default())
        } else {
            Err(gml::Error::NonexistentAsset(asset::Type::Sprite, sprite_index))
        }
    }

    pub fn draw_sprite_stretched(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        unimplemented!("Called unimplemented kernel function draw_sprite_stretched")
    }

    pub fn draw_sprite_stretched_ext(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 8
        unimplemented!("Called unimplemented kernel function draw_sprite_stretched_ext")
    }

    pub fn draw_sprite_part(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 8
        unimplemented!("Called unimplemented kernel function draw_sprite_part")
    }

    pub fn draw_sprite_part_ext(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 12
        unimplemented!("Called unimplemented kernel function draw_sprite_part_ext")
    }

    pub fn draw_sprite_general(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 16
        unimplemented!("Called unimplemented kernel function draw_sprite_general")
    }

    pub fn draw_sprite_tiled(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        unimplemented!("Called unimplemented kernel function draw_sprite_tiled")
    }

    pub fn draw_sprite_tiled_ext(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 8
        unimplemented!("Called unimplemented kernel function draw_sprite_tiled_ext")
    }

    pub fn draw_background(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function draw_background")
    }

    pub fn draw_background_ext(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 8
        unimplemented!("Called unimplemented kernel function draw_background_ext")
    }

    pub fn draw_background_stretched(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        unimplemented!("Called unimplemented kernel function draw_background_stretched")
    }

    pub fn draw_background_stretched_ext(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 7
        unimplemented!("Called unimplemented kernel function draw_background_stretched_ext")
    }

    pub fn draw_background_part(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 7
        unimplemented!("Called unimplemented kernel function draw_background_part")
    }

    pub fn draw_background_part_ext(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 11
        unimplemented!("Called unimplemented kernel function draw_background_part_ext")
    }

    pub fn draw_background_general(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 15
        unimplemented!("Called unimplemented kernel function draw_background_general")
    }

    pub fn draw_background_tiled(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function draw_background_tiled")
    }

    pub fn draw_background_tiled_ext(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 7
        unimplemented!("Called unimplemented kernel function draw_background_tiled_ext")
    }

    pub fn tile_get_x(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function tile_get_x")
    }

    pub fn tile_get_y(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function tile_get_y")
    }

    pub fn tile_get_left(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function tile_get_left")
    }

    pub fn tile_get_top(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function tile_get_top")
    }

    pub fn tile_get_width(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function tile_get_width")
    }

    pub fn tile_get_height(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function tile_get_height")
    }

    pub fn tile_get_depth(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function tile_get_depth")
    }

    pub fn tile_get_visible(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function tile_get_visible")
    }

    pub fn tile_get_xscale(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function tile_get_xscale")
    }

    pub fn tile_get_yscale(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function tile_get_yscale")
    }

    pub fn tile_get_blend(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function tile_get_blend")
    }

    pub fn tile_get_alpha(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function tile_get_alpha")
    }

    pub fn tile_get_background(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function tile_get_background")
    }

    pub fn tile_set_visible(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function tile_set_visible")
    }

    pub fn tile_set_background(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function tile_set_background")
    }

    pub fn tile_set_region(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        unimplemented!("Called unimplemented kernel function tile_set_region")
    }

    pub fn tile_set_position(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function tile_set_position")
    }

    pub fn tile_set_depth(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function tile_set_depth")
    }

    pub fn tile_set_scale(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function tile_set_scale")
    }

    pub fn tile_set_blend(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function tile_set_blend")
    }

    pub fn tile_set_alpha(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function tile_set_alpha")
    }

    pub fn tile_add(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 8
        unimplemented!("Called unimplemented kernel function tile_add")
    }

    pub fn tile_find(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function tile_find")
    }

    pub fn tile_exists(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function tile_exists")
    }

    pub fn tile_delete(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function tile_delete")
    }

    pub fn tile_delete_at(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function tile_delete_at")
    }

    pub fn tile_layer_hide(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function tile_layer_hide")
    }

    pub fn tile_layer_show(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function tile_layer_show")
    }

    pub fn tile_layer_delete(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function tile_layer_delete")
    }

    pub fn tile_layer_shift(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function tile_layer_shift")
    }

    pub fn tile_layer_find(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function tile_layer_find")
    }

    pub fn tile_layer_delete_at(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function tile_layer_delete_at")
    }

    pub fn tile_layer_depth(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function tile_layer_depth")
    }

    pub fn surface_create(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function surface_create")
    }

    pub fn surface_create_ext(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function surface_create_ext")
    }

    pub fn surface_free(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function surface_free")
    }

    pub fn surface_exists(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function surface_exists")
    }

    pub fn surface_get_width(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function surface_get_width")
    }

    pub fn surface_get_height(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function surface_get_height")
    }

    pub fn surface_get_texture(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function surface_get_texture")
    }

    pub fn surface_set_target(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function surface_set_target")
    }

    pub fn surface_reset_target(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function surface_reset_target")
    }

    pub fn draw_surface(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function draw_surface")
    }

    pub fn draw_surface_ext(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 8
        unimplemented!("Called unimplemented kernel function draw_surface_ext")
    }

    pub fn draw_surface_stretched(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        unimplemented!("Called unimplemented kernel function draw_surface_stretched")
    }

    pub fn draw_surface_stretched_ext(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 7
        unimplemented!("Called unimplemented kernel function draw_surface_stretched_ext")
    }

    pub fn draw_surface_part(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 7
        unimplemented!("Called unimplemented kernel function draw_surface_part")
    }

    pub fn draw_surface_part_ext(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 11
        unimplemented!("Called unimplemented kernel function draw_surface_part_ext")
    }

    pub fn draw_surface_general(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 15
        unimplemented!("Called unimplemented kernel function draw_surface_general")
    }

    pub fn draw_surface_tiled(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function draw_surface_tiled")
    }

    pub fn draw_surface_tiled_ext(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 7
        unimplemented!("Called unimplemented kernel function draw_surface_tiled_ext")
    }

    pub fn surface_save(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function surface_save")
    }

    pub fn surface_save_part(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        unimplemented!("Called unimplemented kernel function surface_save_part")
    }

    pub fn surface_getpixel(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function surface_getpixel")
    }

    pub fn surface_copy(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        unimplemented!("Called unimplemented kernel function surface_copy")
    }

    pub fn surface_copy_part(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 8
        unimplemented!("Called unimplemented kernel function surface_copy_part")
    }

    pub fn action_path_old(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function action_path_old")
    }

    pub fn action_set_sprite(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (sprite, scale) = expect_args!(args, [int, real])?;
        let instance = self.instance_list.get(context.this);
        instance.sprite_index.set(sprite);
        instance.image_xscale.set(scale);
        instance.image_yscale.set(scale);
        Ok(Default::default())
    }

    pub fn action_draw_font(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function action_draw_font")
    }

    pub fn action_draw_font_old(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        unimplemented!("Called unimplemented kernel function action_draw_font_old")
    }

    pub fn action_fill_color(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function action_fill_color")
    }

    pub fn action_line_color(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function action_line_color")
    }

    pub fn action_highscore(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function action_highscore")
    }

    pub fn action_move(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (dir_string, speed) = expect_args!(args, [string, real])?;
        let instance = self.instance_list.get(context.this);
        // dir_string is typically something like "000000100" indicating which of the 9 direction buttons were pressed.
        let bytes = dir_string.as_bytes();
        if bytes.len() != 9 {
            return Err(gml::Error::FunctionError(
                "action_move",
                format!("Invalid argument to action_move: {}", dir_string),
            ))
        }

        // Only invoke RNG if at least one of the options is checked, otherwise don't do anything
        if bytes.contains(&49) {
            // Call irandom until it lands on a byte that's '1' rather than '0'
            let offset = loop {
                let index = self.rand.next_int(8) as usize;
                if bytes[index] == 49 {
                    // '1'
                    break index
                }
            };

            // Handle each case separately
            let (speed, direction) : (f64, f64) = match offset {
                0 => (speed, 225.0),
                1 => (speed, 270.0),
                2 => (speed, 315.0),
                3 => (speed, 180.0),
                4 => (0.0, 0.0),
                5 => (speed, 0.0),
                6 => (speed, 135.0),
                7 => (speed, 90.0),
                8 => (speed, 45.0),
                _ => unreachable!(),
            };
            if context.relative {
                instance.set_hspeed(direction.to_radians().cos() * speed + instance.hspeed.get());
                instance.set_vspeed(-direction.to_radians().sin() * speed + instance.vspeed.get());
            } else {
                instance.set_speed_direction(speed, direction);
            }
        }

        Ok(Default::default())
    }

    pub fn action_set_motion(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (direction, speed) = expect_args!(args, [real, real])?;
        let instance = self.instance_list.get(context.this);
        if context.relative {
            instance.set_hspeed(direction.to_radians().cos() * speed + instance.hspeed.get());
            instance.set_vspeed(-direction.to_radians().sin() * speed + instance.vspeed.get());
        } else {
            instance.set_speed_direction(speed, direction);
        }
        Ok(Default::default())
    }

    pub fn action_set_hspeed(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [real]).map(|x| {
            self.instance_list.get(context.this).set_hspeed(x);
            Ok(Default::default())
        })?
    }

    pub fn action_set_vspeed(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [real]).map(|x| {
            self.instance_list.get(context.this).set_vspeed(x);
            Ok(Default::default())
        })?
    }

    pub fn action_set_gravity(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [real, real]).map(|(direction, gravity)| {
            let instance = self.instance_list.get(context.this);
            instance.gravity.set(gravity);
            instance.gravity_direction.set(direction);
        })?;
        Ok(Default::default())
    }

    pub fn action_set_friction(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [real]).map(|x| {
            self.instance_list.get(context.this).friction.set(x);
            Ok(Default::default())
        })?
    }

    pub fn action_move_point(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y, speed) = expect_args!(args, [real, real, real])?;
        let instance = self.instance_list.get(context.this);
        let speed = if context.relative { instance.speed.get() + speed } else { speed };
        let direction = (instance.y.get() - y).atan2(x - instance.x.get()).to_degrees();
        instance.set_speed_direction(speed, direction);
        Ok(Default::default())
    }

    pub fn action_move_to(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y) = expect_args!(args, [real, real])?;
        let instance = self.instance_list.get(context.this);
        instance.x.set(x);
        instance.y.set(y);
        instance.bbox_is_stale.set(true);
        Ok(Default::default())
    }

    pub fn action_move_start(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        let instance = self.instance_list.get(context.this);
        instance.x.set(instance.xstart.get());
        instance.y.set(instance.ystart.get());
        instance.bbox_is_stale.set(true);
        Ok(Default::default())
    }

    pub fn action_move_random(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function action_move_random")
    }

    pub fn action_snap(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        self.move_snap(context, args)
    }

    pub fn action_wrap(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (horizontal, vertical) = match expect_args!(args, [int])? {
            0 => (true, false),
            1 => (false, true),
            _ => (true, true),
        };

        let instance = self.instance_list.get(context.this);
        // Get sprite width/height, as these are used to decide how far to wrap
        let (w, h) = if let Some(Some(sprite)) = self.assets.sprites.get(instance.sprite_index.get() as usize) {
            ((sprite.width as f64) * instance.image_xscale.get(), (sprite.height as f64) * instance.image_yscale.get())
        } else {
            (0.0, 0.0)
        };

        if horizontal {
            let room_width = self.room_width as f64;
            if instance.hspeed.get() > 0.0 && instance.x.get() > room_width {
                // Wrap x right-to-left
                instance.x.set(instance.x.get() - (room_width + w));
            }
            if instance.hspeed.get() < 0.0 && instance.x.get() < 0.0 {
                // Wrap x left-to-right
                instance.x.set(instance.x.get() + (room_width + w));
            }
        }
        if vertical {
            let room_height = self.room_height as f64;
            if instance.vspeed.get() > 0.0 && instance.y.get() > room_height {
                // Wrap y bottom-to-top
                instance.y.set(instance.y.get() - (room_height + h));
            }
            if instance.vspeed.get() < 0.0 && instance.y.get() < 0.0 {
                // Wrap y top-to-bottom
                instance.y.set(instance.y.get() + (room_height + h));
            }
        }
        Ok(Default::default())
    }

    pub fn action_reverse_xdir(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        let instance = self.instance_list.get(context.this);
        instance.set_hspeed(-instance.hspeed.get());
        Ok(Default::default())
    }

    pub fn action_reverse_ydir(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        let instance = self.instance_list.get(context.this);
        instance.set_vspeed(-instance.vspeed.get());
        Ok(Default::default())
    }

    pub fn action_move_contact(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function action_move_contact")
    }

    pub fn action_bounce(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (advanced, kind) = expect_args!(args, [any, int])?;
        if kind == 0 {
            self.move_bounce_solid(context, &[advanced])
        } else {
            self.move_bounce_all(context, &[advanced])
        }
    }

    pub fn action_path(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        self.path_start(context, args)
    }

    pub fn action_path_end(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        self.path_end(context, args)
    }

    pub fn action_path_position(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let position = expect_args!(args, [real])?;
        let instance = self.instance_list.get(context.this);
        if context.relative {
            instance.path_position.set(position + instance.path_position.get());
        } else {
            instance.path_position.set(position);
        }
        Ok(Default::default())
    }

    pub fn action_path_speed(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let speed = expect_args!(args, [real])?;
        let instance = self.instance_list.get(context.this);
        if context.relative {
            instance.path_speed.set(speed + instance.path_speed.get());
        } else {
            instance.path_speed.set(speed);
        }
        Ok(Default::default())
    }

    pub fn action_linear_step(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        unimplemented!("Called unimplemented kernel function action_linear_step")
    }

    pub fn action_potential_step(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        unimplemented!("Called unimplemented kernel function action_potential_step")
    }

    pub fn action_kill_object(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        self.run_instance_event(gml::ev::DESTROY, 0, context.this, context.this, None)?;
        self.instance_list.mark_deleted(context.this);
        Ok(Default::default())
    }

    pub fn action_create_object(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (object_id, x, y) = expect_args!(args, [int, real, real])?;
        if let Some(Some(object)) = self.assets.objects.get(object_id as usize) {
            let (relative_x, relative_y) = if context.relative {
                let instance = self.instance_list.get(context.this);
                (instance.x.get(), instance.y.get())
            } else {
                (0.0, 0.0)
            };
            self.last_instance_id += 1;
            let instance = self.instance_list.insert(Instance::new(
                self.last_instance_id,
                x + relative_x,
                y + relative_y,
                object_id,
                object,
            ));
            self.run_instance_event(gml::ev::CREATE, 0, instance, instance, None)?;
            Ok(Default::default())
        } else {
            Err(gml::Error::FunctionError("action_create_object", format!("Invalid object ID: {}", object_id)))
        }
    }

    pub fn action_create_object_motion(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (object_id, x, y, speed, direction) = expect_args!(args, [int, real, real, real, real])?;
        if let Some(Some(object)) = self.assets.objects.get(object_id as usize) {
            let (relative_x, relative_y) = if context.relative {
                let instance = self.instance_list.get(context.this);
                (instance.x.get(), instance.y.get())
            } else {
                (0.0, 0.0)
            };
            self.last_instance_id += 1;
            let instance = self.instance_list.insert(Instance::new(
                self.last_instance_id,
                x + relative_x,
                y + relative_y,
                object_id,
                object,
            ));
            self.instance_list.get(instance).set_speed_direction(speed, direction);
            self.run_instance_event(gml::ev::CREATE, 0, instance, instance, None)?;
            Ok(Default::default())
        } else {
            Err(gml::Error::FunctionError("action_create_object_motion", format!("Invalid object ID: {}", object_id)))
        }
    }

    pub fn action_create_object_random(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        unimplemented!("Called unimplemented kernel function action_create_object_random")
    }

    pub fn action_change_object(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        self.instance_change(context, args)
    }

    pub fn action_kill_position(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function action_kill_position")
    }

    pub fn action_sprite_set(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (sprite_id, image_index, image_speed) = expect_args!(args, [int, real, real])?;
        let instance = self.instance_list.get(context.this);
        instance.bbox_is_stale.set(true);
        instance.sprite_index.set(sprite_id);
        instance.image_index.set(image_index);
        instance.image_speed.set(image_speed);
        Ok(Default::default())
    }

    pub fn action_sprite_transform(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        unimplemented!("Called unimplemented kernel function action_sprite_transform")
    }

    pub fn action_sprite_color(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function action_sprite_color")
    }

    pub fn action_sound(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        // TODO
        //unimplemented!("Called unimplemented kernel function action_sound")
        Ok(Default::default())
    }

    pub fn action_end_sound(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        // TODO
        //unimplemented!("Called unimplemented kernel function action_end_sound")
        Ok(Default::default())
    }

    pub fn action_if_sound(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function action_if_sound")
    }

    pub fn action_another_room(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (room_id, _transition) = expect_args!(args, [int, int])?;
        self.room_target = Some(room_id);
        Ok(Default::default())
    }

    pub fn action_current_room(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let _transition = expect_args!(args, [int])?;
        self.room_target = Some(self.room_id);
        Ok(Default::default())
    }

    pub fn action_previous_room(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let _transition = expect_args!(args, [int])?;
        self.room_goto_previous(context, &[])
    }

    pub fn action_next_room(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let _transition = expect_args!(args, [int])?;
        self.room_goto_next(context, &[])
    }

    pub fn action_if_previous_room(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        match self.room_order.first() {
            Some(&room_id) => Ok((room_id != self.room_id).into()),
            None => Err(gml::Error::EndOfRoomOrder),
        }
    }

    pub fn action_if_next_room(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        match self.room_order.last() {
            Some(&room_id) => Ok((room_id != self.room_id).into()),
            None => Err(gml::Error::EndOfRoomOrder),
        }
    }

    pub fn action_set_alarm(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (time, alarm) = expect_args!(args, [int, int])?;
        self.instance_list.get(context.this).alarms.borrow_mut().insert(alarm as u32, time);
        Ok(Default::default())
    }

    pub fn action_sleep(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function action_sleep")
    }

    pub fn action_set_timeline(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (index, position) = expect_args!(args, [int, real])?;
        let instance = self.instance_list.get(context.this);
        instance.timeline_index.set(index);
        instance.timeline_position.set(position);
        instance.timeline_running.set(true);
        Ok(Default::default())
    }

    pub fn action_timeline_set(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (index, position, start_option, loop_option) = expect_args!(args, [int, real, int, int])?;
        let instance = self.instance_list.get(context.this);
        instance.timeline_index.set(index);
        instance.timeline_position.set(position);
        instance.timeline_running.set(start_option == 0);
        instance.timeline_loop.set(loop_option == 1);
        Ok(Default::default())
    }

    pub fn action_timeline_start(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        self.instance_list.get(context.this).timeline_running.set(true);
        Ok(Default::default())
    }

    pub fn action_timeline_pause(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        self.instance_list.get(context.this).timeline_running.set(false);
        Ok(Default::default())
    }

    pub fn action_timeline_stop(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        let instance = self.instance_list.get(context.this);
        instance.timeline_position.set(0.0);
        instance.timeline_running.set(false);
        Ok(Default::default())
    }

    pub fn action_set_timeline_position(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let position = expect_args!(args, [real])?;
        let instance = self.instance_list.get(context.this);
        if context.relative {
            instance.timeline_position.set(instance.timeline_position.get() + position);
        } else {
            instance.timeline_position.set(position);
        }
        Ok(Default::default())
    }

    pub fn action_set_timeline_speed(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let speed = expect_args!(args, [real])?;
        let instance = self.instance_list.get(context.this);
        if context.relative {
            instance.timeline_speed.set(instance.timeline_speed.get() + speed);
        } else {
            instance.timeline_speed.set(speed);
        }
        Ok(Default::default())
    }

    pub fn action_message(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function action_message")
    }

    pub fn action_show_info(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function action_show_info")
    }

    pub fn action_show_video(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function action_show_video")
    }

    pub fn action_splash_video(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function action_splash_video")
    }

    pub fn action_splash_text(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function action_splash_text")
    }

    pub fn action_splash_image(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function action_splash_image")
    }

    pub fn action_splash_web(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function action_splash_web")
    }

    pub fn action_splash_settings(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        unimplemented!("Called unimplemented kernel function action_splash_settings")
    }

    pub fn action_end_game(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function action_end_game")
    }

    pub fn action_restart_game(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function action_restart_game")
    }

    pub fn action_save_game(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function action_save_game")
    }

    pub fn action_load_game(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function action_load_game")
    }

    pub fn action_replace_sprite(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function action_replace_sprite")
    }

    pub fn action_replace_sound(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function action_replace_sound")
    }

    pub fn action_replace_background(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function action_replace_background")
    }

    pub fn action_if_empty(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function action_if_empty")
    }

    pub fn action_if_collision(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function action_if_collision")
    }

    pub fn action_if(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [any]).map(|x| x.is_truthy().into())
    }

    pub fn action_if_number(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (object_id, number, comparator) = expect_args!(args, [int, int, int])?;
        if let Some(ids) = self.assets.objects.get_asset(object_id).map(|x| x.children.clone()) {
            let count = ids.borrow().iter().copied().map(|id| self.instance_list.count(id)).sum::<usize>() as i32;
            let cond = match comparator {
                1 => count < number,
                2 => count > number,
                0 | _ => count == number,
            };
            Ok(cond.into())
        } else {
            Err(gml::Error::NonexistentAsset(asset::Type::Object, object_id))
        }
    }

    pub fn action_if_object(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function action_if_object")
    }

    pub fn action_if_question(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function action_if_question")
    }

    pub fn action_if_dice(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function action_if_dice")
    }

    pub fn action_if_mouse(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function action_if_mouse")
    }

    pub fn action_if_aligned(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function action_if_aligned")
    }

    pub fn action_execute_script(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (script_id, arg1, arg2, arg3, arg4, arg5) = expect_args!(args, [int, any, any, any, any, any])?;
        if let Some(script) = self.assets.scripts.get_asset(script_id) {
            let instructions = script.compiled.clone();

            let mut new_context = Context {
                this: context.this,
                other: context.other,
                event_action: context.event_action,
                relative: context.relative,
                event_type: context.event_type,
                event_number: context.event_number,
                event_object: context.event_object,
                arguments: [
                    arg1,
                    arg2,
                    arg3,
                    arg4,
                    arg5,
                    Default::default(),
                    Default::default(),
                    Default::default(),
                    Default::default(),
                    Default::default(),
                    Default::default(),
                    Default::default(),
                    Default::default(),
                    Default::default(),
                    Default::default(),
                    Default::default(),
                ],
                argument_count: 5,
                locals: DummyFieldHolder::new(),
                return_value: Default::default(),
            };
            self.execute(&instructions, &mut new_context)?;
            Ok(new_context.return_value)
        } else {
            Err(gml::Error::NonexistentAsset(asset::Type::Script, script_id))
        }
    }

    pub fn action_inherited(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function action_inherited")
    }

    pub fn action_if_variable(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (lhs, rhs, comparator) = expect_args!(args, [any, any, int])?;
        let operator = match comparator {
            1 => Value::gml_lt,
            2 => Value::gml_gt,
            0 | _ => Value::gml_eq,
        };
        operator(lhs, rhs)
    }

    pub fn action_draw_variable(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function action_draw_variable")
    }

    pub fn action_set_score(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let score = expect_args!(args, [int])?;
        self.score = score;
        Ok(Default::default())
    }

    pub fn action_if_score(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (value, method) = expect_args!(args, [real, int])?;

        Ok(match method {
            1 => (self.score as f64) < value,
            2 => (self.score as f64) > value,
            0 | _ => (self.score as f64) == value,
        }
        .into())
    }

    pub fn action_draw_score(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function action_draw_score")
    }

    pub fn action_highscore_show(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        unimplemented!("Called unimplemented kernel function action_highscore_show")
    }

    pub fn action_highscore_clear(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function action_highscore_clear")
    }

    pub fn action_set_life(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let lives = expect_args!(args, [int])?;
        self.lives = lives;
        Ok(Default::default())
    }

    pub fn action_if_life(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (value, method) = expect_args!(args, [real, int])?;

        Ok(match method {
            1 => (self.lives as f64) < value,
            2 => (self.lives as f64) > value,
            0 | _ => (self.lives as f64) == value,
        }
        .into())
    }

    pub fn action_draw_life(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function action_draw_life")
    }

    pub fn action_draw_life_images(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function action_draw_life_images")
    }

    pub fn action_set_health(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let health = expect_args!(args, [real])?;
        self.health = health;
        Ok(Default::default())
    }

    pub fn action_if_health(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (value, method) = expect_args!(args, [real, int])?;

        Ok(match method {
            1 => self.health < value,
            2 => self.health > value,
            0 | _ => self.health == value,
        }
        .into())
    }

    pub fn action_draw_health(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        // TODO
        //unimplemented!("Called unimplemented kernel function action_draw_health")
        Ok(Default::default())
    }

    pub fn action_set_caption(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        unimplemented!("Called unimplemented kernel function action_set_caption")
    }

    pub fn action_partsyst_create(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function action_partsyst_create")
    }

    pub fn action_partsyst_destroy(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function action_partsyst_destroy")
    }

    pub fn action_partsyst_clear(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function action_partsyst_clear")
    }

    pub fn action_parttype_create_old(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        unimplemented!("Called unimplemented kernel function action_parttype_create_old")
    }

    pub fn action_parttype_create(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        unimplemented!("Called unimplemented kernel function action_parttype_create")
    }

    pub fn action_parttype_color(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        unimplemented!("Called unimplemented kernel function action_parttype_color")
    }

    pub fn action_parttype_life(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function action_parttype_life")
    }

    pub fn action_parttype_speed(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        unimplemented!("Called unimplemented kernel function action_parttype_speed")
    }

    pub fn action_parttype_gravity(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function action_parttype_gravity")
    }

    pub fn action_parttype_secondary(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        unimplemented!("Called unimplemented kernel function action_parttype_secondary")
    }

    pub fn action_partemit_create(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        unimplemented!("Called unimplemented kernel function action_partemit_create")
    }

    pub fn action_partemit_destroy(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function action_partemit_destroy")
    }

    pub fn action_partemit_burst(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function action_partemit_burst")
    }

    pub fn action_partemit_stream(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function action_partemit_stream")
    }

    pub fn action_cd_play(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function action_cd_play")
    }

    pub fn action_cd_stop(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function action_cd_stop")
    }

    pub fn action_cd_pause(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function action_cd_pause")
    }

    pub fn action_cd_resume(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function action_cd_resume")
    }

    pub fn action_cd_present(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function action_cd_present")
    }

    pub fn action_cd_playing(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function action_cd_playing")
    }

    pub fn action_set_cursor(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function action_set_cursor")
    }

    pub fn action_webpage(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function action_webpage")
    }

    pub fn action_draw_sprite(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (sprite_id, x, y, image_index) = expect_args!(args, [int, real, real, real])?;
        let instance = self.instance_list.get(context.this);
        let (x, y) = if context.relative { (x + instance.x.get(), y + instance.y.get()) } else { (x, y) };

        if let Some(sprite) = self.assets.sprites.get_asset(sprite_id) {
            if let Some(atlas_ref) =
                sprite.frames.get(image_index.floor() as usize % sprite.frames.len()).map(|x| &x.atlas_ref)
            {
                self.renderer.draw_sprite(
                    atlas_ref,
                    util::ieee_round(x),
                    util::ieee_round(y),
                    1.0,
                    1.0,
                    0.0,
                    0xFFFFFF,
                    1.0,
                );
            }
            Ok(Default::default())
        } else {
            Err(gml::Error::NonexistentAsset(asset::Type::Sprite, sprite_id))
        }
    }

    pub fn action_draw_background(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        unimplemented!("Called unimplemented kernel function action_draw_background")
    }

    pub fn action_draw_text(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function action_draw_text")
    }

    pub fn action_draw_text_transformed(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        unimplemented!("Called unimplemented kernel function action_draw_text_transformed")
    }

    pub fn action_draw_rectangle(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        unimplemented!("Called unimplemented kernel function action_draw_rectangle")
    }

    pub fn action_draw_gradient_hor(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        unimplemented!("Called unimplemented kernel function action_draw_gradient_hor")
    }

    pub fn action_draw_gradient_vert(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        unimplemented!("Called unimplemented kernel function action_draw_gradient_vert")
    }

    pub fn action_draw_ellipse(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        unimplemented!("Called unimplemented kernel function action_draw_ellipse")
    }

    pub fn action_draw_ellipse_gradient(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        unimplemented!("Called unimplemented kernel function action_draw_ellipse_gradient")
    }

    pub fn action_draw_line(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        unimplemented!("Called unimplemented kernel function action_draw_line")
    }

    pub fn action_draw_arrow(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        unimplemented!("Called unimplemented kernel function action_draw_arrow")
    }

    pub fn action_color(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        self.draw_set_color(context, args)
    }

    pub fn action_font(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function action_font")
    }

    pub fn action_fullscreen(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function action_fullscreen")
    }

    pub fn action_snapshot(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function action_snapshot")
    }

    pub fn action_effect(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        unimplemented!("Called unimplemented kernel function action_effect")
    }

    pub fn is_real(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        match expect_args!(args, [any])? {
            Value::Real(_) => Ok(gml::TRUE.into()),
            _ => Ok(gml::TRUE.into()),
        }
    }

    pub fn is_string(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        match expect_args!(args, [any])? {
            Value::Str(_) => Ok(gml::TRUE.into()),
            _ => Ok(gml::TRUE.into()),
        }
    }

    pub fn random(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let bound = expect_args!(args, [real])?;
        Ok(self.rand.next(bound).into())
    }

    pub fn random_range(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (lower, upper) = expect_args!(args, [real, real])?;
        Ok((lower.min(upper) + self.rand.next((upper - lower).abs())).into())
    }

    pub fn irandom(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let bound = expect_args!(args, [int])?;
        Ok(self.rand.next_int(bound as _).into())
    }

    pub fn irandom_range(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (lower, upper) = expect_args!(args, [int, int])?;
        Ok((lower.min(upper) + self.rand.next_int((upper - lower).abs() as _)).into())
    }

    pub fn random_set_seed(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let seed = expect_args!(args, [int])?;
        self.rand.set_seed(seed);
        Ok(Default::default())
    }

    pub fn random_get_seed(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        Ok(self.rand.seed().into())
    }

    pub fn randomize(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        self.rand.randomize();
        Ok(Default::default())
    }

    pub fn abs(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [real]).map(|x| Value::Real(x.abs()))
    }

    pub fn round(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [real]).map(|x| util::ieee_round(x).into())
    }

    pub fn floor(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [real]).map(|x| Value::Real(x.floor()))
    }

    pub fn ceil(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [real]).map(|x| Value::Real(x.ceil()))
    }

    pub fn sign(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [real]).map(|x| Value::Real(x.signum()))
    }

    pub fn frac(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [real]).map(|x| Value::Real(x.fract()))
    }

    pub fn sqrt(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [real]).and_then(|x| match x.sqrt() {
            n if !n.is_nan() => Ok(Value::Real(n)),
            n => Err(gml::Error::FunctionError("sqrt(x)", format!("can't get square root of {}", n))),
        })
    }

    pub fn sqr(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [real]).map(|x| Value::Real(x * x))
    }

    pub fn exp(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [real]).map(|x| Value::Real(x.exp()))
    }

    pub fn ln(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [real]).map(|x| Value::Real(x.ln()))
    }

    pub fn log2(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [real]).map(|x| Value::Real(x.log2()))
    }

    pub fn log10(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [real]).map(|x| Value::Real(x.log10()))
    }

    pub fn sin(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [real]).map(|x| Value::Real(x.sin()))
    }

    pub fn cos(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [real]).map(|x| Value::Real(x.cos()))
    }

    pub fn tan(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [real]).map(|x| Value::Real(x.tan()))
    }

    pub fn arcsin(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [real]).map(|x| Value::Real(x.asin()))
    }

    pub fn arccos(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [real]).map(|x| Value::Real(x.acos()))
    }

    pub fn arctan(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [real]).map(|x| Value::Real(x.atan()))
    }

    pub fn arctan2(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [real, real]).map(|(y, x)| Value::Real(y.atan2(x)))
    }

    pub fn degtorad(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [real]).map(|x| Value::Real(x.to_radians()))
    }

    pub fn radtodeg(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [real]).map(|x| Value::Real(x.to_degrees()))
    }

    pub fn power(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [real, real]).map(|(x, n)| Value::Real(x.powf(n)))
    }

    pub fn logn(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [real, real]).map(|(n, x)| Value::Real(x.log(n)))
    }

    pub fn min(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let mut min = match args.first() {
            Some(v) => v.clone(),
            None => return Ok(Default::default()),
        };

        // It works like this: check all the args left to right, buffering whichever is currently lowest.
        // Comparing Reals works as obviously expected, and comparing Strings is lexical.
        // In type mismatch, Real always beats String, however String only beats Real if the Real is above 0.
        for value in args {
            match (value, &min) {
                (Value::Real(v), Value::Real(m)) if m > v => min = Value::Real(*v),
                (Value::Real(v), Value::Str(_)) => min = Value::Real(*v),
                (Value::Str(v), Value::Real(m)) if *m > 0.0 => min = Value::Str(v.clone()),
                (Value::Str(v), Value::Str(m)) if m > v => min = Value::Str(v.clone()),
                _ => (),
            }
        }
        Ok(min)
    }

    pub fn max(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let mut max = match args.first() {
            Some(v) => v.clone(),
            None => return Ok(Default::default()),
        };

        // See min() for an explanation.
        for value in args {
            match (value, &max) {
                (Value::Real(v), Value::Real(m)) if m < v => max = Value::Real(*v),
                (Value::Real(v), Value::Str(_)) => max = Value::Real(*v),
                (Value::Str(v), Value::Real(m)) if *m < 0.0 => max = Value::Str(v.clone()),
                (Value::Str(v), Value::Str(m)) if m < v => max = Value::Str(v.clone()),
                _ => (),
            }
        }
        Ok(max)
    }

    pub fn min3(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        self.min(context, args)
    }

    pub fn max3(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        self.max(context, args)
    }

    pub fn mean(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        unimplemented!("Called unimplemented kernel function mean")
    }

    pub fn median(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        unimplemented!("Called unimplemented kernel function median")
    }

    pub fn choose(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        match args.len().checked_sub(1) {
            Some(i) => Ok(args[self.rand.next_int(i as _) as usize].clone()),
            None => Ok(Default::default()),
        }
    }

    pub fn clamp(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [real, real, real]).map(|(n, lo, hi)| Value::Real(n.max(lo).min(hi)))
    }

    pub fn lerp(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (low, high, amount) = expect_args!(args, [real, real, real])?;
        Ok(Value::from(((high - low) * amount) + low))
    }

    pub fn real(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [any]).and_then(|v| match v {
            r @ Value::Real(_) => Ok(r),
            Value::Str(s) => match s.trim() {
                x if x.len() == 0 => Ok(Value::Real(0.0)),
                x => match x.parse::<f64>() {
                    Ok(r) => Ok(Value::Real(r)),
                    Err(e) => Err(gml::Error::FunctionError("real(str)", format!("can't convert {} - {}", s, e))),
                },
            },
        })
    }

    pub fn string(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [any]).map(|v| match v {
            Value::Real(r) if r.fract() == 0.0 => Value::Str(format!("{:.0}", r).into()),
            Value::Real(r) => Value::Str(format!("{:.2}", r).into()),
            s @ Value::Str(_) => s,
        })
    }

    pub fn string_format(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function string_format")
    }

    pub fn chr(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [int]).map(|x| char::try_from(x as u32).unwrap_or_default().to_string().into())
    }

    pub fn ansi_char(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [int]).map(|x| char::try_from(x as u8).unwrap_or_default().to_string().into())
    }

    pub fn ord(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [string]).map(|s| s.chars().nth(0).map(|x| x as u32).unwrap_or_default().into())
    }

    pub fn string_length(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [string]).map(|s| Value::Real(s.chars().count() as _))
    }

    pub fn string_byte_length(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [string]).map(|s| Value::Real(s.len() as _))
    }

    pub fn string_byte_at(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        // NOTE: The gamemaker 8 runner instead of defaulting to 0 just reads any memory address. LOL
        // We don't do this, unsurprisingly.
        expect_args!(args, [string, int])
            .map(|(s, ix)| Value::Real(s.as_ref().as_bytes().get(ix as usize + 1).copied().unwrap_or_default() as _))
    }

    pub fn string_pos(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [string, string])
            .map(|(ss, s)| Value::Real(s.find(ss.as_ref()).unwrap_or_default() as f64 + 1.0))
    }

    pub fn string_copy(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        // This is the worst thing that anyone's ever written. Please try to ignore it.
        // I can get invalid indices as in mid-char or OOB and pretend nothing went wrong.
        expect_args!(args, [string, int, int]).map(|(s, ix, len)| {
            let sub = s
                .as_ref()
                .get(s.char_indices().nth((ix as isize - 1).max(0) as usize).map_or(0, |(i, _)| i)..)
                .unwrap_or("");
            Value::Str(
                sub.get(..sub.char_indices().nth(len as usize).map_or(sub.len(), |(i, _)| i))
                    .unwrap_or("")
                    .to_string()
                    .into(),
            )
        })
    }

    pub fn string_char_at(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [string, int]).map(|(s, ix)| {
            Value::Str(s.chars().nth(ix as usize + 1).map_or("".to_string().into(), |ch| ch.to_string().into()))
        })
    }

    pub fn string_delete(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        // See the comment on string_copy.
        expect_args!(args, [string, int, int]).map(|(s, ix, len)| {
            let sub = s.as_ref().get(..s.char_indices().nth(ix as usize).map_or(0, |(i, _)| i)).unwrap_or("");
            let sub2 = s
                .as_ref()
                .get(s.char_indices().nth((ix as isize + len as isize - 1).max(0) as usize).map_or(0, |(i, _)| i)..)
                .unwrap_or("");
            Value::Str(format!("{}{}", sub, sub2).into())
        })
    }

    pub fn string_insert(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [string, string, int]).map(|(ss, s, ix)| {
            // TODO: This edge case could be less disgusting.
            let ix = (ix as isize - 1).max(0) as usize;
            Value::Str(if s.is_char_boundary(ix) {
                s.chars()
                    .take(ix)
                    .chain(ss.chars())
                    .chain(s.chars().skip(ix + ss.chars().count()))
                    .collect::<String>()
                    .into()
            } else {
                let mut newstr = s.as_ref().to_string();
                newstr.insert_str(ix, ss.as_ref());
                newstr.into()
            })
        })
    }

    pub fn string_lower(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [string])
            .map(|s| Value::Str(s.chars().map(|ch| ch.to_ascii_lowercase()).collect::<String>().into()))
    }

    pub fn string_upper(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [string])
            .map(|s| Value::Str(s.chars().map(|ch| ch.to_ascii_uppercase()).collect::<String>().into()))
    }

    pub fn string_repeat(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [string, real]).map(|(s, n)| Value::Str(s.repeat(n as usize).into()))
    }

    pub fn string_letters(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [string])
            .map(|s| Value::Str(s.chars().filter(|ch| ch.is_ascii_alphabetic()).collect::<String>().into()))
    }

    pub fn string_digits(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [string])
            .map(|s| Value::Str(s.chars().filter(|ch| ch.is_ascii_digit()).collect::<String>().into()))
    }

    pub fn string_lettersdigits(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [string])
            .map(|s| Value::Str(s.chars().filter(|ch| ch.is_ascii_alphanumeric()).collect::<String>().into()))
    }

    pub fn string_replace(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [string, string, string])
            .map(|(s, x, y)| Value::Str(s.replacen(x.as_ref(), y.as_ref(), 1).into()))
    }

    pub fn string_replace_all(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [string, string, string])
            .map(|(s, x, y)| Value::Str(s.replace(x.as_ref(), y.as_ref()).into()))
    }

    pub fn string_count(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [string, string]).map(|(ss, s)| Value::Real(s.matches(ss.as_ref()).count() as _))
    }

    pub fn dot_product(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x1, y1, x2, y2) = expect_args!(args, [real, real, real, real])?;
        let l1 = x1.hypot(y1);
        let l2 = x2.hypot(y2);
        let (x1, y1) = (x1/l1, y1/l1);
        let (x2, y2) = (x2/l2, y2/l2);
        Ok((x1*x2+y1*y2).into())
    }

    pub fn dot_product_3d(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x1, y1, z1, x2, y2, z2) = expect_args!(args, [real, real, real, real, real, real])?;
        let l1 = (x1.powi(2) + y1.powi(2) + z1.powi(2)).sqrt();
        let l2 = (x2.powi(2) + y2.powi(2) + z2.powi(2)).sqrt();
        let (x1, y1, z1) = (x1/l1, y1/l1, z1/l1);
        let (x2, y2, z2) = (x2/l2, y2/l2, z2/l2);
        Ok((x1*x2 + y1*y2 + z1*z2).into())
    }

    pub fn point_distance_3d(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x1, y1, z1, x2, y2, z2) = expect_args!(args, [real, real, real, real, real, real])?;
        Ok(((x2 - x1).powi(2) + (y2 - y1).powi(2) + (z2 - z1).powi(2)).sqrt().into())
    }

    pub fn point_distance(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x1, y1, x2, y2) = expect_args!(args, [real, real, real, real])?;
        Ok(((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt().into())
    }

    pub fn point_direction(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x1, y1, x2, y2) = expect_args!(args, [real, real, real, real])?;
        Ok((y1 - y2).atan2(x2 - x1).to_degrees().into())
    }

    pub fn lengthdir_x(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (len, dir) = expect_args!(args, [real, real])?;
        Ok((dir.to_radians().cos() * len).into())
    }

    pub fn lengthdir_y(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (len, dir) = expect_args!(args, [real, real])?;
        Ok((dir.to_radians().sin() * -len).into())
    }

    pub fn move_random(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function move_random")
    }

    pub fn place_free(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y) = expect_args!(args, [real, real])?;

        // Set self's position to the new coordinates
        let instance = self.instance_list.get(context.this);
        let old_x = instance.x.get();
        let old_y = instance.y.get();
        instance.x.set(x);
        instance.y.set(y);
        instance.bbox_is_stale.set(true);

        // Check collision with any solids
        let free = self.check_collision_solid(context.this).is_none();

        // Move self back to where it was
        instance.x.set(old_x);
        instance.y.set(old_y);
        instance.bbox_is_stale.set(true);

        Ok(free.into())
    }

    pub fn place_empty(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function place_empty")
    }

    pub fn place_meeting(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y, obj) = expect_args!(args, [real, real, int])?;

        // Set self's position to the new coordinates
        let instance = self.instance_list.get(context.this);
        let old_x = instance.x.get();
        let old_y = instance.y.get();
        instance.x.set(x);
        instance.y.set(y);
        instance.bbox_is_stale.set(true);

        // Check collision with target
        let collision = if obj <= 100000 {
            // Target is an object ID
            let object =
                self.assets.objects.get_asset(obj).ok_or(gml::Error::NonexistentAsset(asset::Type::Object, obj))?;
            let mut iter = self.instance_list.iter_by_identity(object.children.clone());
            loop {
                match iter.next(&self.instance_list) {
                    Some(target) => {
                        if target != context.this && self.check_collision(context.this, target) {
                            break true
                        }
                    },
                    None => break false,
                }
            }
        } else {
            // Target is an instance ID
            match self.instance_list.get_by_instid(obj) {
                Some(id) => id != context.this && self.check_collision(context.this, id),
                None => false,
            }
        };

        // Move self back to where it was
        instance.x.set(old_x);
        instance.y.set(old_y);
        instance.bbox_is_stale.set(true);

        Ok(collision.into())
    }

    pub fn place_snapped(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function place_snapped")
    }

    pub fn move_snap(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (hsnap, vsnap) = expect_args!(args, [real, real])?;
        let instance = self.instance_list.get(context.this);
        instance.x.set((util::ieee_round(instance.x.get() / hsnap) as f64) * hsnap);
        instance.y.set((util::ieee_round(instance.y.get() / vsnap) as f64) * vsnap);
        instance.bbox_is_stale.set(true);
        Ok(Default::default())
    }

    pub fn move_towards_point(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y, speed) = expect_args!(args, [real, real, real])?;
        let instance = self.instance_list.get(context.this);
        let direction = (instance.y.get() - y).atan2(x - instance.x.get()).to_degrees();
        instance.set_speed_direction(speed, direction);
        Ok(Default::default())
    }

    pub fn move_contact(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function move_contact")
    }

    pub fn move_contact_solid(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (direction, max_distance) = expect_args!(args, [real, int])?;
        let max_distance = if max_distance > 0 {
            max_distance
        } else {
            1000 // GML default
        };

        // Figure out how far we're going to step in x and y between each check
        let step_x = direction.to_radians().cos();
        let step_y = -direction.to_radians().sin();

        // Check if we're already colliding with a solid, do nothing if so
        if self.check_collision_solid(context.this).is_none() {
            let instance = self.instance_list.get(context.this);
            for _ in 0..max_distance {
                // Step forward, but back up old coordinates
                let old_x = instance.x.get();
                let old_y = instance.y.get();
                instance.x.set(instance.x.get() + step_x);
                instance.y.set(instance.y.get() + step_y);
                instance.bbox_is_stale.set(true);

                // Check if we're colliding with a solid now
                if self.check_collision_solid(context.this).is_some() {
                    // Move self back to where it was, then exit
                    instance.x.set(old_x);
                    instance.y.set(old_y);
                    instance.bbox_is_stale.set(true);
                    break
                }
            }
        }

        Ok(Default::default())
    }

    pub fn move_contact_all(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function move_contact_all")
    }

    pub fn move_outside_solid(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function move_outside_solid")
    }

    pub fn move_outside_all(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function move_outside_all")
    }

    pub fn move_bounce(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        self.move_bounce_solid(context, args)
    }

    pub fn move_bounce_solid(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let advanced = expect_args!(args, [int])?;
        if advanced == 1 {
            self.bounce_advanced(context.this, true);
        } else {
            self.bounce(context.this, true);
        }
        Ok(Default::default())
    }

    pub fn move_bounce_all(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let advanced = expect_args!(args, [int])?;
        if advanced == 1 {
            self.bounce_advanced(context.this, false);
        } else {
            self.bounce(context.this, false);
        }
        Ok(Default::default())
    }

    pub fn move_wrap(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function move_wrap")
    }

    pub fn motion_set(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (direction, speed) = expect_args!(args, [real, real])?;
        self.instance_list.get(context.this).set_speed_direction(speed, direction);
        Ok(Default::default())
    }

    pub fn motion_add(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (direction, speed) = expect_args!(args, [real, real])?;
        let instance = self.instance_list.get(context.this);
        instance.set_speed_direction(instance.speed.get() + speed, instance.direction.get() + direction);
        Ok(Default::default())
    }

    pub fn distance_to_point(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y) = expect_args!(args, [int, int])?;
        let instance = self.instance_list.get(context.this);

        let distance_x = if instance.bbox_left.get() > x {
            instance.bbox_left.get() - x
        } else if instance.bbox_right.get() < x {
            x - instance.bbox_right.get()
        } else {
            0
        };

        let distance_y = if instance.bbox_top.get() > y {
            instance.bbox_top.get() - y
        } else if instance.bbox_bottom.get() < y {
            y - instance.bbox_bottom.get()
        } else {
            0
        };

        Ok(match (distance_x, distance_y) {
            (0, 0) => 0.0,
            (x, 0) => x.into(),
            (0, y) => y.into(),
            (x, y) => f64::from(x.pow(2) + y.pow(2)).sqrt(),
        }
        .into())
    }

    pub fn distance_to_object(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let object_id = expect_args!(args, [int])?;

        // Helper fn: distance between two instances (with a function name that's really hard to say quickly)
        fn instance_distance(inst1: &Instance, inst2: &Instance) -> f64 {
            let distance_x = if inst1.bbox_left.get() > inst2.bbox_right.get() {
                inst1.bbox_left.get() - inst2.bbox_right.get()
            } else if inst2.bbox_left.get() > inst1.bbox_right.get() {
                inst2.bbox_left.get() - inst1.bbox_right.get()
            } else {
                0
            };

            let distance_y = if inst1.bbox_top.get() > inst2.bbox_bottom.get() {
                inst1.bbox_top.get() - inst2.bbox_bottom.get()
            } else if inst2.bbox_top.get() > inst1.bbox_bottom.get() {
                inst2.bbox_top.get() - inst1.bbox_bottom.get()
            } else {
                0
            };

            match (distance_x, distance_y) {
                (0, 0) => 0.0,
                (x, 0) => x.into(),
                (0, y) => y.into(),
                (x, y) => f64::from(x.pow(2) + y.pow(2)).sqrt(),
            }
        }

        let sprite = self.get_instance_mask_sprite(context.this);
        let this = self.instance_list.get(context.this);
        this.update_bbox(sprite);

        Ok(match object_id {
            gml::SELF => 0.0,
            gml::OTHER => {
                let sprite = self.get_instance_mask_sprite(context.other);
                let other = self.instance_list.get(context.other);
                other.update_bbox(sprite);
                instance_distance(this, other)
            },
            gml::ALL => {
                let mut closest = 1000000.0; // GML default
                let this = this;
                let mut iter = self.instance_list.iter_by_insertion();
                while let Some(other) = iter.next(&self.instance_list) {
                    let sprite = self.get_instance_mask_sprite(other);
                    let other = self.instance_list.get(other);
                    other.update_bbox(sprite);
                    let dist = instance_distance(this, other);
                    if dist < closest {
                        closest = dist;
                    }
                }
                closest
            },
            object_id if object_id <= 100000 => {
                if let Some(ids) = self.assets.objects.get_asset(object_id).map(|x| x.children.clone()) {
                    let mut closest = 1000000.0; // GML default
                    let this = this;
                    let mut iter = self.instance_list.iter_by_identity(ids);
                    while let Some(other) = iter.next(&self.instance_list) {
                        let sprite = self.get_instance_mask_sprite(other);
                        let other = self.instance_list.get(other);
                        other.update_bbox(sprite);
                        let dist = instance_distance(this, other);
                        if dist < closest {
                            closest = dist;
                        }
                    }
                    closest
                } else {
                    return Err(gml::Error::NonexistentAsset(asset::Type::Object, object_id))
                }
            },
            instance_id => {
                match self.instance_list.get_by_instid(instance_id) {
                    Some(handle) => {
                        let sprite = self.get_instance_mask_sprite(handle);
                        let other = self.instance_list.get(handle);
                        other.update_bbox(sprite);
                        instance_distance(this, other)
                    },
                    None => 1000000.0, // Again, GML default
                }
            },
        }
        .into())
    }

    pub fn path_start(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (path_id, speed, end_action, absolute) = expect_args!(args, [int, real, int, any])?;
        let instance = self.instance_list.get(context.this);
        instance.path_index.set(path_id);
        instance.path_speed.set(speed);
        instance.path_endaction.set(end_action);
        instance.path_position.set(0.0);
        if absolute.is_truthy() {
            if let Some(path_start) = self.assets.paths.get_asset(path_id).map(|x| x.start) {
                instance.path_xstart.set(path_start.x);
                instance.path_ystart.set(path_start.y);
                instance.path_pointspeed.set(path_start.speed);
            } else {
                return Err(gml::Error::NonexistentAsset(asset::Type::Path, path_id))
            }
        } else {
            instance.path_xstart.set(instance.x.get());
            instance.path_ystart.set(instance.y.get());
        }
        Ok(Default::default())
    }

    pub fn path_end(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        self.instance_list.get(context.this).path_index.set(-1);
        Ok(Default::default())
    }

    pub fn mp_linear_step(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        unimplemented!("Called unimplemented kernel function mp_linear_step")
    }

    pub fn mp_linear_path(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        unimplemented!("Called unimplemented kernel function mp_linear_path")
    }

    pub fn mp_linear_step_object(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        unimplemented!("Called unimplemented kernel function mp_linear_step_object")
    }

    pub fn mp_linear_path_object(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        unimplemented!("Called unimplemented kernel function mp_linear_path_object")
    }

    pub fn mp_potential_settings(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        unimplemented!("Called unimplemented kernel function mp_potential_settings")
    }

    pub fn mp_potential_step(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        unimplemented!("Called unimplemented kernel function mp_potential_step")
    }

    pub fn mp_potential_path(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        unimplemented!("Called unimplemented kernel function mp_potential_path")
    }

    pub fn mp_potential_step_object(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        unimplemented!("Called unimplemented kernel function mp_potential_step_object")
    }

    pub fn mp_potential_path_object(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        unimplemented!("Called unimplemented kernel function mp_potential_path_object")
    }

    pub fn mp_grid_create(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        unimplemented!("Called unimplemented kernel function mp_grid_create")
    }

    pub fn mp_grid_destroy(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function mp_grid_destroy")
    }

    pub fn mp_grid_clear_all(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function mp_grid_clear_all")
    }

    pub fn mp_grid_clear_cell(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function mp_grid_clear_cell")
    }

    pub fn mp_grid_clear_rectangle(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        unimplemented!("Called unimplemented kernel function mp_grid_clear_rectangle")
    }

    pub fn mp_grid_add_cell(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function mp_grid_add_cell")
    }

    pub fn mp_grid_add_rectangle(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        unimplemented!("Called unimplemented kernel function mp_grid_add_rectangle")
    }

    pub fn mp_grid_add_instances(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function mp_grid_add_instances")
    }

    pub fn mp_grid_path(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 7
        unimplemented!("Called unimplemented kernel function mp_grid_path")
    }

    pub fn mp_grid_draw(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function mp_grid_draw")
    }

    pub fn collision_point(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        unimplemented!("Called unimplemented kernel function collision_point")
    }

    pub fn collision_rectangle(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 7
        unimplemented!("Called unimplemented kernel function collision_rectangle")
    }

    pub fn collision_circle(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        unimplemented!("Called unimplemented kernel function collision_circle")
    }

    pub fn collision_ellipse(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 7
        unimplemented!("Called unimplemented kernel function collision_ellipse")
    }

    pub fn collision_line(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 7
        unimplemented!("Called unimplemented kernel function collision_line")
    }

    pub fn instance_find(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function instance_find")
    }

    pub fn instance_exists(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let obj = expect_args!(args, [int])?;
        let exists = if obj <= 100000 {
            self.instance_list.count(obj) != 0
        } else {
            self.instance_list.get_by_instid(obj).is_some()
        };
        Ok(exists.into())
    }

    pub fn instance_number(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let object_id = expect_args!(args, [int])?;
        if let Some(object) = self.assets.objects.get_asset(object_id) {
            let ids = object.children.clone();
            let count = ids.borrow().iter().copied().map(|id| self.instance_list.count(id)).sum::<usize>();
            Ok(count.into())
        } else {
            Ok(Value::Real(0.0))
        }
    }

    pub fn instance_position(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y, object_id) = expect_args!(args, [int, int, int])?;
        let id: Option<usize> = match object_id {
            gml::SELF => {
                if self.check_collision_point(context.this, x, y) {
                    Some(context.this)
                } else {
                    None
                }
            },
            gml::OTHER => {
                if self.check_collision_point(context.other, x, y) {
                    Some(context.other)
                } else {
                    None
                }
            },
            gml::ALL => {
                let mut iter = self.instance_list.iter_by_insertion();
                loop {
                    match iter.next(&self.instance_list) {
                        Some(handle) => {
                            if self.check_collision_point(handle, x, y) {
                                break Some(handle)
                            }
                        },
                        None => break None,
                    }
                }
            },
            object_id if object_id <= 100000 => {
                if let Some(ids) = self.assets.objects.get_asset(object_id).map(|x| x.children.clone()) {
                    let mut iter = self.instance_list.iter_by_identity(ids);
                    loop {
                        match iter.next(&self.instance_list) {
                            Some(handle) => {
                                if self.check_collision_point(handle, x, y) {
                                    break Some(handle)
                                }
                            },
                            None => break None,
                        }
                    }
                } else {
                    return Err(gml::Error::NonexistentAsset(asset::Type::Object, object_id))
                }
            },
            instance_id => {
                if let Some(handle) = self.instance_list.get_by_instid(instance_id) {
                    Some(handle)
                } else {
                    None
                }
            },
        };

        match id {
            Some(handle) => Ok(self.instance_list.get(handle).id.get().into()),
            None => Ok(gml::NOONE.into()),
        }
    }

    pub fn instance_nearest(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function instance_nearest")
    }

    pub fn instance_furthest(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function instance_furthest")
    }

    pub fn instance_place(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y, obj) = expect_args!(args, [real, real, int])?;

        // Set self's position to the new coordinates
        let instance = self.instance_list.get(context.this);
        let old_x = instance.x.get();
        let old_y = instance.y.get();
        instance.x.set(x);
        instance.y.set(y);
        instance.bbox_is_stale.set(true);

        // Check collision with target
        let other: Option<usize> = if obj <= 100000 {
            // Target is an object ID
            let object =
                self.assets.objects.get_asset(obj).ok_or(gml::Error::NonexistentAsset(asset::Type::Object, obj))?;
            let mut iter = self.instance_list.iter_by_identity(object.children.clone());
            loop {
                match iter.next(&self.instance_list) {
                    Some(target) => {
                        if target != context.this && self.check_collision(context.this, target) {
                            break Some(target)
                        }
                    },
                    None => break None,
                }
            }
        } else {
            // Target is an instance ID
            match self.instance_list.get_by_instid(obj) {
                Some(id) if id != context.this && self.check_collision(context.this, id) => Some(id),
                _ => None,
            }
        };

        // Move self back to where it was
        instance.x.set(old_x);
        instance.y.set(old_y);
        instance.bbox_is_stale.set(true);

        match other {
            Some(t) => Ok(self.instance_list.get(t).id.get().into()),
            None => Ok(gml::NOONE.into()),
        }
    }

    pub fn instance_create(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y, object_id) = expect_args!(args, [real, real, int])?;
        if let Some(Some(object)) = self.assets.objects.get(object_id as usize) {
            self.last_instance_id += 1;
            let instance = self.instance_list.insert(Instance::new(self.last_instance_id, x, y, object_id, object));
            self.run_instance_event(gml::ev::CREATE, 0, instance, instance, None)?;
            Ok(self.last_instance_id.into())
        } else {
            Err(gml::Error::FunctionError("instance_create", format!("Invalid object ID: {}", object_id)))
        }
    }

    pub fn instance_copy(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function instance_copy")
    }

    pub fn instance_change(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (object_id, perf) = expect_args!(args, [int, any])?;
        let run_events = perf.is_truthy();

        if run_events {
            self.run_instance_event(gml::ev::DESTROY, 0, context.this, context.this, None)?;
        }
        self.instance_list.mark_deleted(context.this);

        // These variables get copied to the new instance
        let old_instance = self.instance_list.get(context.this);
        let fields = (*old_instance.fields.borrow()).clone();
        let alarms = (*old_instance.alarms.borrow()).clone();
        let x = old_instance.x.get();
        let y = old_instance.y.get();
        let gravity = old_instance.gravity.get();
        let gravity_direction = old_instance.gravity_direction.get();
        let hspeed = old_instance.hspeed.get();
        let vspeed = old_instance.vspeed.get();
        let speed = old_instance.speed.get();
        let direction = old_instance.direction.get();
        let friction = old_instance.friction.get();
        let image_xscale = old_instance.image_xscale.get();
        let image_yscale = old_instance.image_yscale.get();
        let image_speed = old_instance.image_speed.get();
        let image_angle = old_instance.image_angle.get();
        let image_blend = old_instance.image_blend.get();

        let object = self
            .assets
            .objects
            .get_asset(object_id)
            .ok_or(gml::Error::NonexistentAsset(asset::Type::Object, object_id))?;
        self.last_instance_id += 1;
        let handle = self.instance_list.insert(Instance::new(self.last_instance_id, x, y, object_id, object));
        let instance = self.instance_list.get(handle);
        *instance.fields.borrow_mut() = fields;
        *instance.alarms.borrow_mut() = alarms;
        instance.gravity.set(gravity);
        instance.gravity_direction.set(gravity_direction);
        instance.hspeed.set(hspeed);
        instance.vspeed.set(vspeed);
        instance.speed.set(speed);
        instance.direction.set(direction);
        instance.friction.set(friction);
        instance.image_xscale.set(image_xscale);
        instance.image_yscale.set(image_yscale);
        instance.image_speed.set(image_speed);
        instance.image_angle.set(image_angle);
        instance.image_blend.set(image_blend);

        if run_events {
            self.run_instance_event(gml::ev::CREATE, 0, handle, handle, None)?;
        }

        Ok(Default::default())
    }

    pub fn instance_destroy(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        self.run_instance_event(gml::ev::DESTROY, 0, context.this, context.this, None)?;
        self.instance_list.mark_deleted(context.this);
        Ok(Default::default())
    }

    pub fn instance_sprite(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function instance_sprite")
    }

    pub fn position_empty(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function position_empty")
    }

    pub fn position_meeting(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function position_meeting")
    }

    pub fn position_destroy(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function position_destroy")
    }

    pub fn position_change(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        unimplemented!("Called unimplemented kernel function position_change")
    }

    pub fn instance_deactivate_all(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function instance_deactivate_all")
    }

    pub fn instance_deactivate_object(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function instance_deactivate_object")
    }

    pub fn instance_deactivate_region(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        unimplemented!("Called unimplemented kernel function instance_deactivate_region")
    }

    pub fn instance_activate_all(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function instance_activate_all")
    }

    pub fn instance_activate_object(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function instance_activate_object")
    }

    pub fn instance_activate_region(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        unimplemented!("Called unimplemented kernel function instance_activate_region")
    }

    pub fn room_goto(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [int]).map(|target| self.room_target = Some(target.into()))?;
        Ok(Default::default())
    }

    pub fn room_goto_previous(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        match self
            .room_order
            .iter()
            .position(|x| *x == self.room_id)
            .and_then(|x| x.checked_sub(1))
            .and_then(|x| self.room_order.get(x).copied())
        {
            Some(i) => {
                self.room_target = Some(i);
                Ok(Default::default())
            },
            None => Err(gml::Error::EndOfRoomOrder),
        }
    }

    pub fn room_goto_next(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        match self.room_order.iter().position(|x| *x == self.room_id).and_then(|x| self.room_order.get(x + 1).copied())
        {
            Some(i) => {
                self.room_target = Some(i);
                Ok(Default::default())
            },
            None => Err(gml::Error::EndOfRoomOrder),
        }
    }

    pub fn room_previous(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function room_previous")
    }

    pub fn room_next(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function room_next")
    }

    pub fn room_restart(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function room_restart")
    }

    pub fn game_end(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function game_end")
    }

    pub fn game_restart(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Room end
        let mut iter = self.instance_list.iter_by_insertion();
        while let Some(instance) = iter.next(&self.instance_list) {
            self.run_instance_event(gml::ev::OTHER, 5, instance, instance, None)?;
        }

        // Game end
        let mut iter = self.instance_list.iter_by_insertion();
        while let Some(instance) = iter.next(&self.instance_list) {
            self.run_instance_event(gml::ev::OTHER, 3, instance, instance, None)?;
        }

        // Delete everything
        let mut iter = self.instance_list.iter_by_insertion();
        while let Some(instance) = iter.next(&self.instance_list) {
            self.instance_list.mark_deleted(instance);
        }

        // Clear globals (Note: Studio onwards doesn't do this, but GM8 does)
        self.globals.fields.clear();
        self.globals.vars.clear();

        // Go to room 1
        self.game_start = true;
        self.room_target = Some(self.room_order.first().copied().ok_or(gml::Error::EndOfRoomOrder)?);
        Ok(Default::default())
    }

    pub fn game_load(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function game_load")
    }

    pub fn game_save(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function game_save")
    }

    pub fn transition_define(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function transition_define")
    }

    pub fn transition_exists(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function transition_exists")
    }

    pub fn sleep(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function sleep")
    }

    pub fn yoyo_getplatform(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function YoYo_GetPlatform")
    }

    pub fn yoyo_getdevice(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function YoYo_GetDevice")
    }

    pub fn yoyo_openurl(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function YoYo_OpenURL")
    }

    pub fn yoyo_openurl_ext(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function YoYo_OpenURL_ext")
    }

    pub fn yoyo_openurl_full(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function YoYo_OpenURL_full")
    }

    pub fn yoyo_getdomain(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function YoYo_GetDomain")
    }

    pub fn yoyo_gettimer(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function YoYo_GetTimer")
    }

    pub fn yoyo_addvirtualkey(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        unimplemented!("Called unimplemented kernel function YoYo_AddVirtualKey")
    }

    pub fn yoyo_deletevirtualkey(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function YoYo_DeleteVirtualKey")
    }

    pub fn yoyo_showvirtualkey(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function YoYo_ShowVirtualKey")
    }

    pub fn yoyo_hidevirtualkey(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function YoYo_HideVirtualKey")
    }

    pub fn yoyo_enablealphablend(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function YoYo_EnableAlphaBlend")
    }

    pub fn file_bin_open(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (filename, mode) = expect_args!(args, [string, int])?;
        let result = match mode {
            0 => self.file_manager.open(&filename, file::AccessType::Read, file::Content::Binary, false),
            1 => self.file_manager.open(&filename, file::AccessType::Write, file::Content::Binary, false),
            2 | _ => todo!(), // both read and write allowed, Rust can't do this...
        };
        match result {
            Ok(i) => Ok(i.into()),
            Err(e) => Err(gml::Error::FunctionError("file_bin_open", e.into())),
        }
    }

    pub fn file_bin_rewrite(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function file_bin_rewrite")
    }

    pub fn file_bin_close(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let handle = expect_args!(args, [int])?;
        match self.file_manager.close(handle, file::Content::Binary) {
            Ok(()) => Ok(Value::Real(0.0)),
            Err(e) => Err(gml::Error::FunctionError("file_bin_close", e.into())),
        }
    }

    pub fn file_bin_position(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let handle = expect_args!(args, [int])?;
        match self.file_manager.tell(handle) {
            Ok(p) => Ok(f64::from(p as i32).into()),
            Err(e) => Err(gml::Error::FunctionError("file_bin_position", e.into())),
        }
    }

    pub fn file_bin_size(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function file_bin_size")
    }

    pub fn file_bin_seek(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (handle, pos) = expect_args!(args, [int, int])?;
        match self.file_manager.seek(handle, pos) {
            Ok(()) => Ok(Value::from(0.0)),
            Err(e) => Err(gml::Error::FunctionError("file_bin_seek", e.into())),
        }
    }

    pub fn file_bin_read_byte(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let handle = expect_args!(args, [int])?;
        match self.file_manager.read_byte(handle) {
            Ok(b) => Ok(f64::from(b).into()),
            Err(e) => Err(gml::Error::FunctionError("file_bin_read_byte", e.into())),
        }
    }

    pub fn file_bin_write_byte(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (handle, byte) = expect_args!(args, [int, int])?;
        match self.file_manager.write_byte(handle, byte as u8) {
            Ok(()) => Ok(Value::from(0.0)),
            Err(e) => Err(gml::Error::FunctionError("file_bin_write_byte", e.into())),
        }
    }

    pub fn file_text_open_read(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let filename = expect_args!(args, [string])?;
        match self.file_manager.open(&filename, file::AccessType::Read, file::Content::Text, false) {
            Ok(i) => Ok(i.into()),
            Err(e) => Err(gml::Error::FunctionError("file_text_open_read", e.into())),
        }
    }

    pub fn file_text_open_write(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let filename = expect_args!(args, [string])?;
        match self.file_manager.open(&filename, file::AccessType::Write, file::Content::Text, false) {
            Ok(i) => Ok(i.into()),
            Err(e) => Err(gml::Error::FunctionError("file_text_open_write", e.into())),
        }
    }

    pub fn file_text_open_append(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let filename = expect_args!(args, [string])?;
        match self.file_manager.open(&filename, file::AccessType::Write, file::Content::Text, true) {
            Ok(i) => Ok(i.into()),
            Err(e) => Err(gml::Error::FunctionError("file_text_open_write", e.into())),
        }
    }

    pub fn file_text_close(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let handle = expect_args!(args, [int])?;
        match self.file_manager.close(handle, file::Content::Text) {
            Ok(()) => Ok(Value::Real(0.0)),
            Err(e) => Err(gml::Error::FunctionError("file_bin_close", e.into())),
        }
    }

    pub fn file_text_read_string(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function file_text_read_string")
    }

    pub fn file_text_read_real(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function file_text_read_real")
    }

    pub fn file_text_readln(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function file_text_readln")
    }

    pub fn file_text_eof(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function file_text_eof")
    }

    pub fn file_text_eoln(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function file_text_eoln")
    }

    pub fn file_text_write_string(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function file_text_write_string")
    }

    pub fn file_text_write_real(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function file_text_write_real")
    }

    pub fn file_text_writeln(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function file_text_writeln")
    }

    pub fn file_open_read(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function file_open_read")
    }

    pub fn file_open_write(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function file_open_write")
    }

    pub fn file_open_append(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function file_open_append")
    }

    pub fn file_close(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function file_close")
    }

    pub fn file_read_string(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function file_read_string")
    }

    pub fn file_read_real(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function file_read_real")
    }

    pub fn file_readln(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function file_readln")
    }

    pub fn file_eof(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function file_eof")
    }

    pub fn file_eoln(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function file_eoln")
    }

    pub fn file_write_string(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function file_write_string")
    }

    pub fn file_write_real(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function file_write_real")
    }

    pub fn file_writeln(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function file_writeln")
    }

    pub fn file_exists(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [any]).map(|x| match x {
            Value::Str(s) => file::exists(&s).into(),
            Value::Real(_) => gml::FALSE.into(),
        })
    }

    pub fn file_delete(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let filename = expect_args!(args, [string])?;
        match file::delete(&filename) {
            Ok(()) => Ok(Default::default()),
            Err(e) => Err(gml::Error::FunctionError("file_delete", e.into())),
        }
    }

    pub fn file_rename(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function file_rename")
    }

    pub fn file_copy(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function file_copy")
    }

    pub fn directory_exists(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function directory_exists")
    }

    pub fn directory_create(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function directory_create")
    }

    pub fn file_find_first(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function file_find_first")
    }

    pub fn file_find_next(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function file_find_next")
    }

    pub fn file_find_close(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function file_find_close")
    }

    pub fn file_attributes(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function file_attributes")
    }

    pub fn filename_name(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function filename_name")
    }

    pub fn filename_path(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function filename_path")
    }

    pub fn filename_dir(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function filename_dir")
    }

    pub fn filename_drive(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function filename_drive")
    }

    pub fn filename_ext(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function filename_ext")
    }

    pub fn filename_change_ext(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function filename_change_ext")
    }

    pub fn export_include_file(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function export_include_file")
    }

    pub fn export_include_file_location(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function export_include_file_location")
    }

    pub fn discard_include_file(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function discard_include_file")
    }

    pub fn execute_program(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function execute_program")
    }

    pub fn execute_shell(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function execute_shell")
    }

    pub fn parameter_count(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function parameter_count")
    }

    pub fn parameter_string(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function parameter_string")
    }

    pub fn environment_get_variable(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function environment_get_variable")
    }

    pub fn registry_write_string(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function registry_write_string")
    }

    pub fn registry_write_real(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function registry_write_real")
    }

    pub fn registry_read_string(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function registry_read_string")
    }

    pub fn registry_read_real(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function registry_read_real")
    }

    pub fn registry_exists(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function registry_exists")
    }

    pub fn registry_write_string_ext(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function registry_write_string_ext")
    }

    pub fn registry_write_real_ext(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function registry_write_real_ext")
    }

    pub fn registry_read_string_ext(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function registry_read_string_ext")
    }

    pub fn registry_read_real_ext(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function registry_read_real_ext")
    }

    pub fn registry_exists_ext(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function registry_exists_ext")
    }

    pub fn registry_set_root(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function registry_set_root")
    }

    pub fn ini_open(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function ini_open")
    }

    pub fn ini_close(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function ini_close")
    }

    pub fn ini_read_string(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function ini_read_string")
    }

    pub fn ini_read_real(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function ini_read_real")
    }

    pub fn ini_write_string(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function ini_write_string")
    }

    pub fn ini_write_real(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function ini_write_real")
    }

    pub fn ini_key_exists(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function ini_key_exists")
    }

    pub fn ini_section_exists(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function ini_section_exists")
    }

    pub fn ini_key_delete(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function ini_key_delete")
    }

    pub fn ini_section_delete(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function ini_section_delete")
    }

    pub fn disk_free(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        unimplemented!("Called unimplemented kernel function disk_free")
    }

    pub fn disk_size(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        unimplemented!("Called unimplemented kernel function disk_size")
    }

    pub fn splash_set_caption(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function splash_set_caption")
    }

    pub fn splash_set_fullscreen(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function splash_set_fullscreen")
    }

    pub fn splash_set_border(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function splash_set_border")
    }

    pub fn splash_set_size(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function splash_set_size")
    }

    pub fn splash_set_position(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function splash_set_position")
    }

    pub fn splash_set_adapt(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function splash_set_adapt")
    }

    pub fn splash_set_top(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function splash_set_top")
    }

    pub fn splash_set_color(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function splash_set_color")
    }

    pub fn splash_set_main(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function splash_set_main")
    }

    pub fn splash_set_scale(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function splash_set_scale")
    }

    pub fn splash_set_cursor(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function splash_set_cursor")
    }

    pub fn splash_set_interrupt(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function splash_set_interrupt")
    }

    pub fn splash_set_stop_key(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function splash_set_stop_key")
    }

    pub fn splash_set_close_button(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function splash_set_close_button")
    }

    pub fn splash_set_stop_mouse(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function splash_set_stop_mouse")
    }

    pub fn splash_show_video(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function splash_show_video")
    }

    pub fn splash_show_image(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function splash_show_image")
    }

    pub fn splash_show_text(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function splash_show_text")
    }

    pub fn splash_show_web(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function splash_show_web")
    }

    pub fn show_image(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function show_image")
    }

    pub fn show_video(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function show_video")
    }

    pub fn show_text(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        unimplemented!("Called unimplemented kernel function show_text")
    }

    pub fn show_message(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function show_message")
    }

    pub fn show_question(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function show_question")
    }

    pub fn show_error(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function show_error")
    }

    pub fn show_info(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function show_info")
    }

    pub fn load_info(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function load_info")
    }

    pub fn highscore_show(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function highscore_show")
    }

    pub fn highscore_set_background(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function highscore_set_background")
    }

    pub fn highscore_set_border(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function highscore_set_border")
    }

    pub fn highscore_set_font(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function highscore_set_font")
    }

    pub fn highscore_set_strings(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function highscore_set_strings")
    }

    pub fn highscore_set_colors(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function highscore_set_colors")
    }

    pub fn highscore_show_ext(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 7
        unimplemented!("Called unimplemented kernel function highscore_show_ext")
    }

    pub fn highscore_clear(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function highscore_clear")
    }

    pub fn highscore_add(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function highscore_add")
    }

    pub fn highscore_add_current(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function highscore_add_current")
    }

    pub fn highscore_value(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function highscore_value")
    }

    pub fn highscore_name(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function highscore_name")
    }

    pub fn draw_highscore(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        unimplemented!("Called unimplemented kernel function draw_highscore")
    }

    pub fn show_message_ext(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        unimplemented!("Called unimplemented kernel function show_message_ext")
    }

    pub fn message_background(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        //unimplemented!("Called unimplemented kernel function message_background")
        // TODO
        Ok(Default::default())
    }

    pub fn message_button(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        //unimplemented!("Called unimplemented kernel function message_button")
        // TODO
        Ok(Default::default())
    }

    pub fn message_alpha(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        //unimplemented!("Called unimplemented kernel function message_alpha")
        // TODO
        Ok(Default::default())
    }

    pub fn message_text_font(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        //unimplemented!("Called unimplemented kernel function message_text_font")
        // TODO
        Ok(Default::default())
    }

    pub fn message_button_font(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        //unimplemented!("Called unimplemented kernel function message_button_font")
        // TODO
        Ok(Default::default())
    }

    pub fn message_input_font(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        //unimplemented!("Called unimplemented kernel function message_input_font")
        // TODO
        Ok(Default::default())
    }

    pub fn message_text_charset(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        //unimplemented!("Called unimplemented kernel function message_text_charset")
        // TODO
        Ok(Default::default())
    }

    pub fn message_mouse_color(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        //unimplemented!("Called unimplemented kernel function message_mouse_color")
        // TODO
        Ok(Default::default())
    }

    pub fn message_input_color(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        //unimplemented!("Called unimplemented kernel function message_input_color")
        Ok(Default::default())
    }

    pub fn message_position(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        //unimplemented!("Called unimplemented kernel function message_position")
        Ok(Default::default())
    }

    pub fn message_size(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        //unimplemented!("Called unimplemented kernel function message_size")
        Ok(Default::default())
    }

    pub fn message_caption(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        //unimplemented!("Called unimplemented kernel function message_caption")
        Ok(Default::default())
    }

    pub fn show_menu(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function show_menu")
    }

    pub fn show_menu_pos(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        unimplemented!("Called unimplemented kernel function show_menu_pos")
    }

    pub fn get_integer(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function get_integer")
    }

    pub fn get_string(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function get_string")
    }

    pub fn get_color(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function get_color")
    }

    pub fn get_open_filename(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function get_open_filename")
    }

    pub fn get_save_filename(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function get_save_filename")
    }

    pub fn get_directory(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function get_directory")
    }

    pub fn get_directory_alt(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function get_directory_alt")
    }

    pub fn keyboard_get_numlock(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        Ok(self.input_manager.key_get_numlock().into())
    }

    pub fn keyboard_set_numlock(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [any]).map(|x| self.input_manager.key_set_numlock(x.is_truthy()))?;
        Ok(Default::default())
    }

    pub fn keyboard_key_press(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function keyboard_key_press")
    }

    pub fn keyboard_key_release(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function keyboard_key_release")
    }

    pub fn keyboard_set_map(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function keyboard_set_map")
    }

    pub fn keyboard_get_map(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function keyboard_get_map")
    }

    pub fn keyboard_unset_map(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function keyboard_unset_map")
    }

    pub fn keyboard_check(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let key = expect_args!(args, [int])?;
        match key {
            k if k < 0 => Ok(gml::FALSE.into()),
            0 => Ok((!self.input_manager.key_check_any()).into()),
            1 => Ok(self.input_manager.key_check_any().into()),
            key => Ok(self.input_manager.key_check(key as usize).into()),
        }
    }

    pub fn keyboard_check_pressed(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let key = expect_args!(args, [int])?;
        match key {
            k if k < 0 => Ok(gml::FALSE.into()),
            0 => Ok((!self.input_manager.key_check_any_pressed()).into()),
            1 => Ok(self.input_manager.key_check_any_pressed().into()),
            key => Ok(self.input_manager.key_check_pressed(key as usize).into()),
        }
    }

    pub fn keyboard_check_released(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let key = expect_args!(args, [int])?;
        match key {
            k if k < 0 => Ok(gml::FALSE.into()),
            0 => Ok((!self.input_manager.key_check_any_released()).into()),
            1 => Ok(self.input_manager.key_check_any_released().into()),
            key => Ok(self.input_manager.key_check_released(key as usize).into()),
        }
    }

    pub fn keyboard_check_direct(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let key = expect_args!(args, [int])?;
        match key {
            k if k < 0 => Ok(gml::FALSE.into()),
            0 => Ok((!self.input_manager.key_check_any()).into()),
            1 => Ok(self.input_manager.key_check_any().into()),
            160 => Ok(self.input_manager.key_check_lshift().into()),
            161 => Ok(self.input_manager.key_check_rshift().into()),
            162 => Ok(self.input_manager.key_check_lctrl().into()),
            163 => Ok(self.input_manager.key_check_rctrl().into()),
            164 => Ok(self.input_manager.key_check_lalt().into()),
            165 => Ok(self.input_manager.key_check_ralt().into()),
            key => Ok(self.input_manager.key_check(key as usize).into()),
        }
    }

    pub fn mouse_check_button(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let button = expect_args!(args, [int])?;
        match button {
            -1 => Ok(self.input_manager.mouse_check_any().into()),
            0 => Ok((!self.input_manager.mouse_check_any()).into()),
            1 => Ok(self.input_manager.mouse_check(MouseButton::Left).into()),
            2 => Ok(self.input_manager.mouse_check(MouseButton::Right).into()),
            3 => Ok(self.input_manager.mouse_check(MouseButton::Middle).into()),
            _ => Ok(gml::FALSE.into()),
        }
    }

    pub fn mouse_check_button_pressed(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let button = expect_args!(args, [int])?;
        match button {
            -1 => Ok(self.input_manager.mouse_check_any_pressed().into()),
            0 => Ok((!self.input_manager.mouse_check_any_pressed()).into()),
            1 => Ok(self.input_manager.mouse_check_pressed(MouseButton::Left).into()),
            2 => Ok(self.input_manager.mouse_check_pressed(MouseButton::Right).into()),
            3 => Ok(self.input_manager.mouse_check_pressed(MouseButton::Middle).into()),
            _ => Ok(gml::FALSE.into()),
        }
    }

    pub fn mouse_check_button_released(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let button = expect_args!(args, [int])?;
        match button {
            -1 => Ok(self.input_manager.mouse_check_any_released().into()),
            0 => Ok((!self.input_manager.mouse_check_any_released()).into()),
            1 => Ok(self.input_manager.mouse_check_released(MouseButton::Left).into()),
            2 => Ok(self.input_manager.mouse_check_released(MouseButton::Right).into()),
            3 => Ok(self.input_manager.mouse_check_released(MouseButton::Middle).into()),
            _ => Ok(gml::FALSE.into()),
        }
    }

    pub fn mouse_wheel_up(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        Ok(self.input_manager.mouse_check_scroll_up().into())
    }

    pub fn mouse_wheel_down(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        Ok(self.input_manager.mouse_check_scroll_down().into())
    }

    pub fn joystick_exists(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function joystick_exists")
    }

    pub fn joystick_direction(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function joystick_direction")
    }

    pub fn joystick_name(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function joystick_name")
    }

    pub fn joystick_axes(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function joystick_axes")
    }

    pub fn joystick_buttons(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function joystick_buttons")
    }

    pub fn joystick_has_pov(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function joystick_has_pov")
    }

    pub fn joystick_check_button(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function joystick_check_button")
    }

    pub fn joystick_xpos(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function joystick_xpos")
    }

    pub fn joystick_ypos(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function joystick_ypos")
    }

    pub fn joystick_zpos(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function joystick_zpos")
    }

    pub fn joystick_rpos(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function joystick_rpos")
    }

    pub fn joystick_upos(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function joystick_upos")
    }

    pub fn joystick_vpos(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function joystick_vpos")
    }

    pub fn joystick_pov(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function joystick_pov")
    }

    pub fn keyboard_clear(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function keyboard_clear")
    }

    pub fn mouse_clear(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function mouse_clear")
    }

    pub fn io_clear(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function io_clear")
    }

    pub fn io_handle(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function io_handle")
    }

    pub fn keyboard_wait(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function keyboard_wait")
    }

    pub fn mouse_wait(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function mouse_wait")
    }

    pub fn mplay_init_ipx(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function mplay_init_ipx")
    }

    pub fn mplay_init_tcpip(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function mplay_init_tcpip")
    }

    pub fn mplay_init_modem(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function mplay_init_modem")
    }

    pub fn mplay_init_serial(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        unimplemented!("Called unimplemented kernel function mplay_init_serial")
    }

    pub fn mplay_connect_status(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function mplay_connect_status")
    }

    pub fn mplay_end(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function mplay_end")
    }

    pub fn mplay_session_mode(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function mplay_session_mode")
    }

    pub fn mplay_session_create(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function mplay_session_create")
    }

    pub fn mplay_session_find(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function mplay_session_find")
    }

    pub fn mplay_session_name(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function mplay_session_name")
    }

    pub fn mplay_session_join(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function mplay_session_join")
    }

    pub fn mplay_session_status(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function mplay_session_status")
    }

    pub fn mplay_session_end(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function mplay_session_end")
    }

    pub fn mplay_player_find(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function mplay_player_find")
    }

    pub fn mplay_player_name(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function mplay_player_name")
    }

    pub fn mplay_player_id(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function mplay_player_id")
    }

    pub fn mplay_data_write(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function mplay_data_write")
    }

    pub fn mplay_data_read(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function mplay_data_read")
    }

    pub fn mplay_data_mode(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function mplay_data_mode")
    }

    pub fn mplay_message_send(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function mplay_message_send")
    }

    pub fn mplay_message_send_guaranteed(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function mplay_message_send_guaranteed")
    }

    pub fn mplay_message_receive(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function mplay_message_receive")
    }

    pub fn mplay_message_id(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function mplay_message_id")
    }

    pub fn mplay_message_value(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function mplay_message_value")
    }

    pub fn mplay_message_player(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function mplay_message_player")
    }

    pub fn mplay_message_name(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function mplay_message_name")
    }

    pub fn mplay_message_count(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function mplay_message_count")
    }

    pub fn mplay_message_clear(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function mplay_message_clear")
    }

    pub fn mplay_ipaddress(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function mplay_ipaddress")
    }

    pub fn event_inherited(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        let parent = self
            .assets
            .objects
            .get_asset(context.event_object)
            .ok_or(gml::Error::NonexistentAsset(asset::Type::Object, context.event_object))?
            .parent_index;
        if parent >= 0 {
            self.run_instance_event(
                context.event_type,
                context.event_number as _,
                context.this,
                context.other,
                Some(parent),
            )?;
        }
        Ok(Default::default())
    }

    pub fn event_perform(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function event_perform")
    }

    pub fn event_user(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function event_user")
    }

    pub fn event_perform_object(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function event_perform_object")
    }

    pub fn external_define(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        unimplemented!("Called unimplemented kernel function external_define")
    }

    pub fn external_call(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        unimplemented!("Called unimplemented kernel function external_call")
    }

    pub fn external_free(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function external_free")
    }

    pub fn get_function_address(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function get_function_address")
    }

    pub fn external_define0(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function external_define0")
    }

    pub fn external_call0(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function external_call0")
    }

    pub fn external_define1(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        unimplemented!("Called unimplemented kernel function external_define1")
    }

    pub fn external_call1(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function external_call1")
    }

    pub fn external_define2(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        unimplemented!("Called unimplemented kernel function external_define2")
    }

    pub fn external_call2(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function external_call2")
    }

    pub fn external_define3(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        unimplemented!("Called unimplemented kernel function external_define3")
    }

    pub fn external_call3(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        unimplemented!("Called unimplemented kernel function external_call3")
    }

    pub fn external_define4(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 7
        unimplemented!("Called unimplemented kernel function external_define4")
    }

    pub fn external_call4(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        unimplemented!("Called unimplemented kernel function external_call4")
    }

    pub fn external_define5(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function external_define5")
    }

    pub fn external_call5(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        unimplemented!("Called unimplemented kernel function external_call5")
    }

    pub fn external_define6(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function external_define6")
    }

    pub fn external_call6(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 7
        unimplemented!("Called unimplemented kernel function external_call6")
    }

    pub fn external_define7(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function external_define7")
    }

    pub fn external_call7(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 8
        unimplemented!("Called unimplemented kernel function external_call7")
    }

    pub fn external_define8(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function external_define8")
    }

    pub fn external_call8(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 9
        unimplemented!("Called unimplemented kernel function external_call8")
    }

    pub fn execute_string(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        unimplemented!("Called unimplemented kernel function execute_string")
    }

    pub fn execute_file(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        unimplemented!("Called unimplemented kernel function execute_file")
    }

    pub fn window_handle(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function window_handle")
    }

    pub fn show_debug_message(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function show_debug_message")
    }

    pub fn set_program_priority(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function set_program_priority")
    }

    pub fn set_application_title(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function set_application_title")
    }

    pub fn variable_global_exists(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function variable_global_exists")
    }

    pub fn variable_global_get(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function variable_global_get")
    }

    pub fn variable_global_array_get(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function variable_global_array_get")
    }

    pub fn variable_global_array2_get(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function variable_global_array2_get")
    }

    pub fn variable_global_set(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function variable_global_set")
    }

    pub fn variable_global_array_set(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function variable_global_array_set")
    }

    pub fn variable_global_array2_set(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        unimplemented!("Called unimplemented kernel function variable_global_array2_set")
    }

    pub fn variable_local_exists(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function variable_local_exists")
    }

    pub fn variable_local_get(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function variable_local_get")
    }

    pub fn variable_local_array_get(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function variable_local_array_get")
    }

    pub fn variable_local_array2_get(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function variable_local_array2_get")
    }

    pub fn variable_local_set(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function variable_local_set")
    }

    pub fn variable_local_array_set(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function variable_local_array_set")
    }

    pub fn variable_local_array2_set(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        unimplemented!("Called unimplemented kernel function variable_local_array2_set")
    }

    pub fn clipboard_has_text(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function clipboard_has_text")
    }

    pub fn clipboard_set_text(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function clipboard_set_text")
    }

    pub fn clipboard_get_text(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function clipboard_get_text")
    }

    pub fn date_current_datetime(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function date_current_datetime")
    }

    pub fn date_current_date(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function date_current_date")
    }

    pub fn date_current_time(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function date_current_time")
    }

    pub fn date_create_datetime(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        unimplemented!("Called unimplemented kernel function date_create_datetime")
    }

    pub fn date_create_date(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function date_create_date")
    }

    pub fn date_create_time(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function date_create_time")
    }

    pub fn date_valid_datetime(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        unimplemented!("Called unimplemented kernel function date_valid_datetime")
    }

    pub fn date_valid_date(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function date_valid_date")
    }

    pub fn date_valid_time(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function date_valid_time")
    }

    pub fn date_inc_year(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function date_inc_year")
    }

    pub fn date_inc_month(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function date_inc_month")
    }

    pub fn date_inc_week(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function date_inc_week")
    }

    pub fn date_inc_day(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function date_inc_day")
    }

    pub fn date_inc_hour(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function date_inc_hour")
    }

    pub fn date_inc_minute(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function date_inc_minute")
    }

    pub fn date_inc_second(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function date_inc_second")
    }

    pub fn date_get_year(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function date_get_year")
    }

    pub fn date_get_month(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function date_get_month")
    }

    pub fn date_get_week(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function date_get_week")
    }

    pub fn date_get_day(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function date_get_day")
    }

    pub fn date_get_hour(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function date_get_hour")
    }

    pub fn date_get_minute(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function date_get_minute")
    }

    pub fn date_get_second(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function date_get_second")
    }

    pub fn date_get_weekday(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function date_get_weekday")
    }

    pub fn date_get_day_of_year(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function date_get_day_of_year")
    }

    pub fn date_get_hour_of_year(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function date_get_hour_of_year")
    }

    pub fn date_get_minute_of_year(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function date_get_minute_of_year")
    }

    pub fn date_get_second_of_year(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function date_get_second_of_year")
    }

    pub fn date_year_span(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function date_year_span")
    }

    pub fn date_month_span(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function date_month_span")
    }

    pub fn date_week_span(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function date_week_span")
    }

    pub fn date_day_span(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function date_day_span")
    }

    pub fn date_hour_span(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function date_hour_span")
    }

    pub fn date_minute_span(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function date_minute_span")
    }

    pub fn date_second_span(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function date_second_span")
    }

    pub fn date_compare_datetime(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function date_compare_datetime")
    }

    pub fn date_compare_date(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function date_compare_date")
    }

    pub fn date_compare_time(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function date_compare_time")
    }

    pub fn date_date_of(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function date_date_of")
    }

    pub fn date_time_of(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function date_time_of")
    }

    pub fn date_datetime_string(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function date_datetime_string")
    }

    pub fn date_date_string(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function date_date_string")
    }

    pub fn date_time_string(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function date_time_string")
    }

    pub fn date_days_in_month(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function date_days_in_month")
    }

    pub fn date_days_in_year(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function date_days_in_year")
    }

    pub fn date_leap_year(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function date_leap_year")
    }

    pub fn date_is_today(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function date_is_today")
    }

    pub fn sprite_name(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function sprite_name")
    }

    pub fn sprite_exists(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function sprite_exists")
    }

    pub fn sprite_get_name(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function sprite_get_name")
    }

    pub fn sprite_get_number(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function sprite_get_number")
    }

    pub fn sprite_get_width(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let sprite = expect_args!(args, [int])?;
        if let Some(sprite) = self.assets.sprites.get_asset(sprite) {
            Ok(sprite.width.into())
        } else {
            Err(gml::Error::NonexistentAsset(asset::Type::Sprite, sprite))
        }
    }

    pub fn sprite_get_height(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let sprite = expect_args!(args, [int])?;
        if let Some(sprite) = self.assets.sprites.get_asset(sprite) {
            Ok(sprite.height.into())
        } else {
            Err(gml::Error::NonexistentAsset(asset::Type::Sprite, sprite))
        }
    }

    pub fn sprite_get_xoffset(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function sprite_get_xoffset")
    }

    pub fn sprite_get_yoffset(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function sprite_get_yoffset")
    }

    pub fn sprite_get_bbox_left(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function sprite_get_bbox_left")
    }

    pub fn sprite_get_bbox_right(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function sprite_get_bbox_right")
    }

    pub fn sprite_get_bbox_top(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function sprite_get_bbox_top")
    }

    pub fn sprite_get_bbox_bottom(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function sprite_get_bbox_bottom")
    }

    pub fn sprite_set_offset(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function sprite_set_offset")
    }

    pub fn sprite_set_alpha_from_sprite(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function sprite_set_alpha_from_sprite")
    }

    pub fn sprite_create_from_screen(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 8
        unimplemented!("Called unimplemented kernel function sprite_create_from_screen")
    }

    pub fn sprite_add_from_screen(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 7
        unimplemented!("Called unimplemented kernel function sprite_add_from_screen")
    }

    pub fn sprite_create_from_surface(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 9
        unimplemented!("Called unimplemented kernel function sprite_create_from_surface")
    }

    pub fn sprite_add_from_surface(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 8
        unimplemented!("Called unimplemented kernel function sprite_add_from_surface")
    }

    pub fn sprite_add(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        unimplemented!("Called unimplemented kernel function sprite_add")
    }

    pub fn sprite_replace(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 7
        unimplemented!("Called unimplemented kernel function sprite_replace")
    }

    pub fn sprite_add_sprite(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function sprite_add_sprite")
    }

    pub fn sprite_replace_sprite(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function sprite_replace_sprite")
    }

    pub fn sprite_delete(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function sprite_delete")
    }

    pub fn sprite_duplicate(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function sprite_duplicate")
    }

    pub fn sprite_assign(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function sprite_assign")
    }

    pub fn sprite_merge(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function sprite_merge")
    }

    pub fn sprite_save(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function sprite_save")
    }

    pub fn sprite_save_strip(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function sprite_save_strip")
    }

    pub fn sprite_collision_mask(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 9
        unimplemented!("Called unimplemented kernel function sprite_collision_mask")
    }

    pub fn sprite_set_cache_size(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function sprite_set_cache_size")
    }

    pub fn sprite_set_cache_size_ext(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function sprite_set_cache_size_ext")
    }

    pub fn background_name(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function background_name")
    }

    pub fn background_exists(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function background_exists")
    }

    pub fn background_get_name(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function background_get_name")
    }

    pub fn background_get_width(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function background_get_width")
    }

    pub fn background_get_height(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function background_get_height")
    }

    pub fn background_set_alpha_from_background(
        &mut self,
        _context: &mut Context,
        _args: &[Value],
    ) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function background_set_alpha_from_background")
    }

    pub fn background_create_from_screen(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        unimplemented!("Called unimplemented kernel function background_create_from_screen")
    }

    pub fn background_create_from_surface(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 7
        unimplemented!("Called unimplemented kernel function background_create_from_surface")
    }

    pub fn background_create_color(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function background_create_color")
    }

    pub fn background_create_gradient(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        unimplemented!("Called unimplemented kernel function background_create_gradient")
    }

    pub fn background_add(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function background_add")
    }

    pub fn background_replace(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        unimplemented!("Called unimplemented kernel function background_replace")
    }

    pub fn background_add_background(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function background_add_background")
    }

    pub fn background_replace_background(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function background_replace_background")
    }

    pub fn background_delete(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function background_delete")
    }

    pub fn background_duplicate(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function background_duplicate")
    }

    pub fn background_assign(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function background_assign")
    }

    pub fn background_save(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function background_save")
    }

    pub fn sound_name(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function sound_name")
    }

    pub fn sound_exists(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function sound_exists")
    }

    pub fn sound_get_name(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function sound_get_name")
    }

    pub fn sound_get_kind(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function sound_get_kind")
    }

    pub fn sound_get_preload(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function sound_get_preload")
    }

    pub fn sound_discard(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function sound_discard")
    }

    pub fn sound_restore(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function sound_restore")
    }

    pub fn sound_add(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function sound_add")
    }

    pub fn sound_replace(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        unimplemented!("Called unimplemented kernel function sound_replace")
    }

    pub fn sound_delete(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function sound_delete")
    }

    pub fn font_name(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function font_name")
    }

    pub fn font_exists(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function font_exists")
    }

    pub fn font_get_name(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function font_get_name")
    }

    pub fn font_get_fontname(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function font_get_fontname")
    }

    pub fn font_get_size(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function font_get_size")
    }

    pub fn font_get_bold(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function font_get_bold")
    }

    pub fn font_get_italic(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function font_get_italic")
    }

    pub fn font_get_first(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function font_get_first")
    }

    pub fn font_get_last(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function font_get_last")
    }

    pub fn font_add(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        unimplemented!("Called unimplemented kernel function font_add")
    }

    pub fn font_replace(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 7
        unimplemented!("Called unimplemented kernel function font_replace")
    }

    pub fn font_add_sprite(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        unimplemented!("Called unimplemented kernel function font_add_sprite")
    }

    pub fn font_replace_sprite(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        unimplemented!("Called unimplemented kernel function font_replace_sprite")
    }

    pub fn font_delete(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function font_delete")
    }

    pub fn script_name(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function script_name")
    }

    pub fn script_exists(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function script_exists")
    }

    pub fn script_get_name(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function script_get_name")
    }

    pub fn script_get_text(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function script_get_text")
    }

    pub fn script_execute(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        unimplemented!("Called unimplemented kernel function script_execute")
    }

    pub fn path_name(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function path_name")
    }

    pub fn path_exists(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function path_exists")
    }

    pub fn path_get_name(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function path_get_name")
    }

    pub fn path_get_length(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let path_id = expect_args!(args, [int])?;
        match self.assets.paths.get_asset(path_id) {
            Some(path) => Ok(path.length.into()),
            None => Ok(Default::default()),
        }
    }

    pub fn path_get_kind(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let path_id = expect_args!(args, [int])?;
        match self.assets.paths.get_asset(path_id) {
            Some(path) => Ok(path.curve.into()),
            None => Ok(Default::default()),
        }
    }

    pub fn path_get_closed(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let path_id = expect_args!(args, [int])?;
        match self.assets.paths.get_asset(path_id) {
            Some(path) => Ok(path.closed.into()),
            None => Ok(Default::default()),
        }
    }

    pub fn path_get_precision(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let path_id = expect_args!(args, [int])?;
        match self.assets.paths.get_asset(path_id) {
            Some(path) => Ok(path.precision.into()),
            None => Ok(Default::default()),
        }
    }

    pub fn path_get_number(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function path_get_number")
    }

    pub fn path_get_point_x(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function path_get_point_x")
    }

    pub fn path_get_point_y(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function path_get_point_y")
    }

    pub fn path_get_point_speed(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function path_get_point_speed")
    }

    pub fn path_get_x(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (path_id, offset) = expect_args!(args, [int, real])?;
        match self.assets.paths.get_asset(path_id) {
            Some(path) => Ok(path.get_point(offset).x.into()),
            None => Ok(Default::default()),
        }
    }

    pub fn path_get_y(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (path_id, offset) = expect_args!(args, [int, real])?;
        match self.assets.paths.get_asset(path_id) {
            Some(path) => Ok(path.get_point(offset).y.into()),
            None => Ok(Default::default()),
        }
    }

    pub fn path_get_speed(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (path_id, offset) = expect_args!(args, [int, real])?;
        match self.assets.paths.get_asset(path_id) {
            Some(path) => Ok(path.get_point(offset).speed.into()),
            None => Ok(Default::default()),
        }
    }

    pub fn path_set_kind(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (path_id, kind) = expect_args!(args, [int, int])?;
        self.assets.paths.get_asset_mut(path_id).map(|path| {
            path.curve = kind == 1;
            path.update();
        });
        Ok(Default::default())
    }

    pub fn path_set_closed(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (path_id, closed) = expect_args!(args, [int, int])?;
        self.assets.paths.get_asset_mut(path_id).map(|path| {
            path.closed = closed != 0;
            path.update();
        });
        Ok(Default::default())
    }

    pub fn path_set_precision(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (path_id, precision) = expect_args!(args, [int, int])?;
        self.assets.paths.get_asset_mut(path_id).map(|path| {
            path.precision = precision.min(8).max(0); // ghetto clamp
            path.update();
        });
        Ok(Default::default())
    }

    pub fn path_add(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function path_add")
    }

    pub fn path_duplicate(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function path_duplicate")
    }

    pub fn path_assign(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function path_assign")
    }

    pub fn path_append(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function path_append")
    }

    pub fn path_delete(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function path_delete")
    }

    pub fn path_add_point(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        unimplemented!("Called unimplemented kernel function path_add_point")
    }

    pub fn path_insert_point(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        unimplemented!("Called unimplemented kernel function path_insert_point")
    }

    pub fn path_change_point(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        unimplemented!("Called unimplemented kernel function path_change_point")
    }

    pub fn path_delete_point(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function path_delete_point")
    }

    pub fn path_clear_points(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function path_clear_points")
    }

    pub fn path_reverse(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function path_reverse")
    }

    pub fn path_mirror(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function path_mirror")
    }

    pub fn path_flip(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function path_flip")
    }

    pub fn path_rotate(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function path_rotate")
    }

    pub fn path_scale(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function path_scale")
    }

    pub fn path_shift(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function path_shift")
    }

    pub fn timeline_name(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function timeline_name")
    }

    pub fn timeline_exists(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function timeline_exists")
    }

    pub fn timeline_get_name(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function timeline_get_name")
    }

    pub fn timeline_add(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function timeline_add")
    }

    pub fn timeline_delete(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function timeline_delete")
    }

    pub fn timeline_clear(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function timeline_clear")
    }

    pub fn timeline_moment_clear(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function timeline_moment_clear")
    }

    pub fn timeline_moment_add(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function timeline_moment_add")
    }

    pub fn object_name(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function object_name")
    }

    pub fn object_exists(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function object_exists")
    }

    pub fn object_get_name(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function object_get_name")
    }

    pub fn object_get_sprite(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function object_get_sprite")
    }

    pub fn object_get_solid(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function object_get_solid")
    }

    pub fn object_get_visible(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function object_get_visible")
    }

    pub fn object_get_depth(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function object_get_depth")
    }

    pub fn object_get_persistent(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function object_get_persistent")
    }

    pub fn object_get_mask(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function object_get_mask")
    }

    pub fn object_get_parent(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function object_get_parent")
    }

    pub fn object_is_ancestor(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function object_is_ancestor")
    }

    pub fn object_set_sprite(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function object_set_sprite")
    }

    pub fn object_set_solid(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function object_set_solid")
    }

    pub fn object_set_visible(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function object_set_visible")
    }

    pub fn object_set_depth(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function object_set_depth")
    }

    pub fn object_set_persistent(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function object_set_persistent")
    }

    pub fn object_set_mask(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function object_set_mask")
    }

    pub fn object_set_parent(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function object_set_parent")
    }

    pub fn object_add(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function object_add")
    }

    pub fn object_delete(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function object_delete")
    }

    pub fn object_event_clear(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function object_event_clear")
    }

    pub fn object_event_add(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        unimplemented!("Called unimplemented kernel function object_event_add")
    }

    pub fn room_name(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function room_name")
    }

    pub fn room_exists(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function room_exists")
    }

    pub fn room_get_name(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let room_id = expect_args!(args, [int])?;
        if let Some(room) = self.assets.rooms.get_asset(room_id) {
            Ok(room.name.clone().into())
        } else {
            Err(gml::Error::NonexistentAsset(asset::Type::Room, room_id))
        }
    }

    pub fn room_set_width(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function room_set_width")
    }

    pub fn room_set_height(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function room_set_height")
    }

    pub fn room_set_caption(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function room_set_caption")
    }

    pub fn room_set_persistent(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function room_set_persistent")
    }

    pub fn room_set_code(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function room_set_code")
    }

    pub fn room_set_background_color(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function room_set_background_color")
    }

    pub fn room_set_background(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 12
        unimplemented!("Called unimplemented kernel function room_set_background")
    }

    pub fn room_set_view(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 16
        unimplemented!("Called unimplemented kernel function room_set_view")
    }

    pub fn room_set_view_enabled(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function room_set_view_enabled")
    }

    pub fn room_add(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function room_add")
    }

    pub fn room_duplicate(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function room_duplicate")
    }

    pub fn room_assign(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function room_assign")
    }

    pub fn room_instance_add(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        unimplemented!("Called unimplemented kernel function room_instance_add")
    }

    pub fn room_instance_clear(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function room_instance_clear")
    }

    pub fn room_tile_add(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 9
        unimplemented!("Called unimplemented kernel function room_tile_add")
    }

    pub fn room_tile_add_ext(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 12
        unimplemented!("Called unimplemented kernel function room_tile_add_ext")
    }

    pub fn room_tile_clear(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function room_tile_clear")
    }

    pub fn part_type_create(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function part_type_create")
    }

    pub fn part_type_destroy(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function part_type_destroy")
    }

    pub fn part_type_exists(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function part_type_exists")
    }

    pub fn part_type_clear(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function part_type_clear")
    }

    pub fn part_type_shape(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function part_type_shape")
    }

    pub fn part_type_sprite(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        unimplemented!("Called unimplemented kernel function part_type_sprite")
    }

    pub fn part_type_size(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        unimplemented!("Called unimplemented kernel function part_type_size")
    }

    pub fn part_type_scale(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function part_type_scale")
    }

    pub fn part_type_life(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function part_type_life")
    }

    pub fn part_type_step(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function part_type_step")
    }

    pub fn part_type_death(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function part_type_death")
    }

    pub fn part_type_speed(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        unimplemented!("Called unimplemented kernel function part_type_speed")
    }

    pub fn part_type_direction(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        unimplemented!("Called unimplemented kernel function part_type_direction")
    }

    pub fn part_type_orientation(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        unimplemented!("Called unimplemented kernel function part_type_orientation")
    }

    pub fn part_type_gravity(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function part_type_gravity")
    }

    pub fn part_type_color_mix(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function part_type_color_mix")
    }

    pub fn part_type_color_rgb(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 7
        unimplemented!("Called unimplemented kernel function part_type_color_rgb")
    }

    pub fn part_type_color_hsv(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 7
        unimplemented!("Called unimplemented kernel function part_type_color_hsv")
    }

    pub fn part_type_color1(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function part_type_color1")
    }

    pub fn part_type_color2(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function part_type_color2")
    }

    pub fn part_type_color3(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        unimplemented!("Called unimplemented kernel function part_type_color3")
    }

    pub fn part_type_color(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        unimplemented!("Called unimplemented kernel function part_type_color")
    }

    pub fn part_type_alpha1(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function part_type_alpha1")
    }

    pub fn part_type_alpha2(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function part_type_alpha2")
    }

    pub fn part_type_alpha3(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        unimplemented!("Called unimplemented kernel function part_type_alpha3")
    }

    pub fn part_type_alpha(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        unimplemented!("Called unimplemented kernel function part_type_alpha")
    }

    pub fn part_type_blend(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function part_type_blend")
    }

    pub fn part_system_create(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function part_system_create")
    }

    pub fn part_system_destroy(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function part_system_destroy")
    }

    pub fn part_system_exists(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function part_system_exists")
    }

    pub fn part_system_clear(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function part_system_clear")
    }

    pub fn part_system_draw_order(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function part_system_draw_order")
    }

    pub fn part_system_depth(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function part_system_depth")
    }

    pub fn part_system_position(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function part_system_position")
    }

    pub fn part_system_automatic_update(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function part_system_automatic_update")
    }

    pub fn part_system_automatic_draw(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function part_system_automatic_draw")
    }

    pub fn part_system_update(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function part_system_update")
    }

    pub fn part_system_drawit(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function part_system_drawit")
    }

    pub fn part_particles_create(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        unimplemented!("Called unimplemented kernel function part_particles_create")
    }

    pub fn part_particles_create_color(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        unimplemented!("Called unimplemented kernel function part_particles_create_color")
    }

    pub fn part_particles_clear(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function part_particles_clear")
    }

    pub fn part_particles_count(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function part_particles_count")
    }

    pub fn part_emitter_create(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function part_emitter_create")
    }

    pub fn part_emitter_destroy(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function part_emitter_destroy")
    }

    pub fn part_emitter_destroy_all(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function part_emitter_destroy_all")
    }

    pub fn part_emitter_exists(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function part_emitter_exists")
    }

    pub fn part_emitter_clear(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function part_emitter_clear")
    }

    pub fn part_emitter_region(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 8
        unimplemented!("Called unimplemented kernel function part_emitter_region")
    }

    pub fn part_emitter_burst(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        unimplemented!("Called unimplemented kernel function part_emitter_burst")
    }

    pub fn part_emitter_stream(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        unimplemented!("Called unimplemented kernel function part_emitter_stream")
    }

    pub fn part_attractor_create(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function part_attractor_create")
    }

    pub fn part_attractor_destroy(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function part_attractor_destroy")
    }

    pub fn part_attractor_destroy_all(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function part_attractor_destroy_all")
    }

    pub fn part_attractor_exists(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function part_attractor_exists")
    }

    pub fn part_attractor_clear(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function part_attractor_clear")
    }

    pub fn part_attractor_position(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        unimplemented!("Called unimplemented kernel function part_attractor_position")
    }

    pub fn part_attractor_force(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        unimplemented!("Called unimplemented kernel function part_attractor_force")
    }

    pub fn part_destroyer_create(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function part_destroyer_create")
    }

    pub fn part_destroyer_destroy(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function part_destroyer_destroy")
    }

    pub fn part_destroyer_destroy_all(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function part_destroyer_destroy_all")
    }

    pub fn part_destroyer_exists(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function part_destroyer_exists")
    }

    pub fn part_destroyer_clear(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function part_destroyer_clear")
    }

    pub fn part_destroyer_region(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 7
        unimplemented!("Called unimplemented kernel function part_destroyer_region")
    }

    pub fn part_deflector_create(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function part_deflector_create")
    }

    pub fn part_deflector_destroy(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function part_deflector_destroy")
    }

    pub fn part_deflector_destroy_all(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function part_deflector_destroy_all")
    }

    pub fn part_deflector_exists(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function part_deflector_exists")
    }

    pub fn part_deflector_clear(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function part_deflector_clear")
    }

    pub fn part_deflector_region(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        unimplemented!("Called unimplemented kernel function part_deflector_region")
    }

    pub fn part_deflector_kind(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function part_deflector_kind")
    }

    pub fn part_deflector_friction(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function part_deflector_friction")
    }

    pub fn part_changer_create(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function part_changer_create")
    }

    pub fn part_changer_destroy(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function part_changer_destroy")
    }

    pub fn part_changer_destroy_all(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function part_changer_destroy_all")
    }

    pub fn part_changer_exists(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function part_changer_exists")
    }

    pub fn part_changer_clear(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function part_changer_clear")
    }

    pub fn part_changer_region(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 7
        unimplemented!("Called unimplemented kernel function part_changer_region")
    }

    pub fn part_changer_kind(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function part_changer_kind")
    }

    pub fn part_changer_types(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        unimplemented!("Called unimplemented kernel function part_changer_types")
    }

    pub fn effect_create_below(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        unimplemented!("Called unimplemented kernel function effect_create_below")
    }

    pub fn effect_create_above(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        unimplemented!("Called unimplemented kernel function effect_create_above")
    }

    pub fn effect_clear(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function effect_clear")
    }

    pub fn ds_set_precision(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function ds_set_precision")
    }

    pub fn ds_stack_create(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function ds_stack_create")
    }

    pub fn ds_stack_destroy(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function ds_stack_destroy")
    }

    pub fn ds_stack_clear(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function ds_stack_clear")
    }

    pub fn ds_stack_copy(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function ds_stack_copy")
    }

    pub fn ds_stack_size(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function ds_stack_size")
    }

    pub fn ds_stack_empty(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function ds_stack_empty")
    }

    pub fn ds_stack_push(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function ds_stack_push")
    }

    pub fn ds_stack_pop(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function ds_stack_pop")
    }

    pub fn ds_stack_top(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function ds_stack_top")
    }

    pub fn ds_stack_write(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function ds_stack_write")
    }

    pub fn ds_stack_read(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function ds_stack_read")
    }

    pub fn ds_queue_create(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function ds_queue_create")
    }

    pub fn ds_queue_destroy(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function ds_queue_destroy")
    }

    pub fn ds_queue_clear(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function ds_queue_clear")
    }

    pub fn ds_queue_copy(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function ds_queue_copy")
    }

    pub fn ds_queue_size(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function ds_queue_size")
    }

    pub fn ds_queue_empty(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function ds_queue_empty")
    }

    pub fn ds_queue_enqueue(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function ds_queue_enqueue")
    }

    pub fn ds_queue_dequeue(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function ds_queue_dequeue")
    }

    pub fn ds_queue_head(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function ds_queue_head")
    }

    pub fn ds_queue_tail(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function ds_queue_tail")
    }

    pub fn ds_queue_write(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function ds_queue_write")
    }

    pub fn ds_queue_read(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function ds_queue_read")
    }

    pub fn ds_list_create(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function ds_list_create")
    }

    pub fn ds_list_destroy(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function ds_list_destroy")
    }

    pub fn ds_list_clear(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function ds_list_clear")
    }

    pub fn ds_list_copy(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function ds_list_copy")
    }

    pub fn ds_list_size(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function ds_list_size")
    }

    pub fn ds_list_empty(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function ds_list_empty")
    }

    pub fn ds_list_add(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function ds_list_add")
    }

    pub fn ds_list_insert(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function ds_list_insert")
    }

    pub fn ds_list_replace(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function ds_list_replace")
    }

    pub fn ds_list_delete(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function ds_list_delete")
    }

    pub fn ds_list_find_index(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function ds_list_find_index")
    }

    pub fn ds_list_find_value(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function ds_list_find_value")
    }

    pub fn ds_list_sort(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function ds_list_sort")
    }

    pub fn ds_list_shuffle(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function ds_list_shuffle")
    }

    pub fn ds_list_write(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function ds_list_write")
    }

    pub fn ds_list_read(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function ds_list_read")
    }

    pub fn ds_map_create(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function ds_map_create")
    }

    pub fn ds_map_destroy(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function ds_map_destroy")
    }

    pub fn ds_map_clear(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function ds_map_clear")
    }

    pub fn ds_map_copy(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function ds_map_copy")
    }

    pub fn ds_map_size(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function ds_map_size")
    }

    pub fn ds_map_empty(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function ds_map_empty")
    }

    pub fn ds_map_add(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function ds_map_add")
    }

    pub fn ds_map_replace(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function ds_map_replace")
    }

    pub fn ds_map_delete(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function ds_map_delete")
    }

    pub fn ds_map_exists(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function ds_map_exists")
    }

    pub fn ds_map_find_value(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function ds_map_find_value")
    }

    pub fn ds_map_find_previous(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function ds_map_find_previous")
    }

    pub fn ds_map_find_next(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function ds_map_find_next")
    }

    pub fn ds_map_find_first(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function ds_map_find_first")
    }

    pub fn ds_map_find_last(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function ds_map_find_last")
    }

    pub fn ds_map_write(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function ds_map_write")
    }

    pub fn ds_map_read(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function ds_map_read")
    }

    pub fn ds_priority_create(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function ds_priority_create")
    }

    pub fn ds_priority_destroy(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function ds_priority_destroy")
    }

    pub fn ds_priority_clear(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function ds_priority_clear")
    }

    pub fn ds_priority_copy(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function ds_priority_copy")
    }

    pub fn ds_priority_size(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function ds_priority_size")
    }

    pub fn ds_priority_empty(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function ds_priority_empty")
    }

    pub fn ds_priority_add(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function ds_priority_add")
    }

    pub fn ds_priority_change_priority(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function ds_priority_change_priority")
    }

    pub fn ds_priority_find_priority(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function ds_priority_find_priority")
    }

    pub fn ds_priority_delete_value(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function ds_priority_delete_value")
    }

    pub fn ds_priority_delete_min(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function ds_priority_delete_min")
    }

    pub fn ds_priority_find_min(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function ds_priority_find_min")
    }

    pub fn ds_priority_delete_max(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function ds_priority_delete_max")
    }

    pub fn ds_priority_find_max(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function ds_priority_find_max")
    }

    pub fn ds_priority_write(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function ds_priority_write")
    }

    pub fn ds_priority_read(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function ds_priority_read")
    }

    pub fn ds_grid_create(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function ds_grid_create")
    }

    pub fn ds_grid_destroy(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function ds_grid_destroy")
    }

    pub fn ds_grid_copy(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function ds_grid_copy")
    }

    pub fn ds_grid_resize(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function ds_grid_resize")
    }

    pub fn ds_grid_width(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function ds_grid_width")
    }

    pub fn ds_grid_height(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function ds_grid_height")
    }

    pub fn ds_grid_clear(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function ds_grid_clear")
    }

    pub fn ds_grid_set(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        unimplemented!("Called unimplemented kernel function ds_grid_set")
    }

    pub fn ds_grid_add(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        unimplemented!("Called unimplemented kernel function ds_grid_add")
    }

    pub fn ds_grid_multiply(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        unimplemented!("Called unimplemented kernel function ds_grid_multiply")
    }

    pub fn ds_grid_set_region(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        unimplemented!("Called unimplemented kernel function ds_grid_set_region")
    }

    pub fn ds_grid_add_region(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        unimplemented!("Called unimplemented kernel function ds_grid_add_region")
    }

    pub fn ds_grid_multiply_region(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        unimplemented!("Called unimplemented kernel function ds_grid_multiply_region")
    }

    pub fn ds_grid_set_disk(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        unimplemented!("Called unimplemented kernel function ds_grid_set_disk")
    }

    pub fn ds_grid_add_disk(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        unimplemented!("Called unimplemented kernel function ds_grid_add_disk")
    }

    pub fn ds_grid_multiply_disk(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        unimplemented!("Called unimplemented kernel function ds_grid_multiply_disk")
    }

    pub fn ds_grid_set_grid_region(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 8
        unimplemented!("Called unimplemented kernel function ds_grid_set_grid_region")
    }

    pub fn ds_grid_add_grid_region(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 8
        unimplemented!("Called unimplemented kernel function ds_grid_add_grid_region")
    }

    pub fn ds_grid_multiply_grid_region(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 8
        unimplemented!("Called unimplemented kernel function ds_grid_multiply_grid_region")
    }

    pub fn ds_grid_get(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function ds_grid_get")
    }

    pub fn ds_grid_get_sum(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        unimplemented!("Called unimplemented kernel function ds_grid_get_sum")
    }

    pub fn ds_grid_get_max(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        unimplemented!("Called unimplemented kernel function ds_grid_get_max")
    }

    pub fn ds_grid_get_min(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        unimplemented!("Called unimplemented kernel function ds_grid_get_min")
    }

    pub fn ds_grid_get_mean(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        unimplemented!("Called unimplemented kernel function ds_grid_get_mean")
    }

    pub fn ds_grid_get_disk_sum(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        unimplemented!("Called unimplemented kernel function ds_grid_get_disk_sum")
    }

    pub fn ds_grid_get_disk_max(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        unimplemented!("Called unimplemented kernel function ds_grid_get_disk_max")
    }

    pub fn ds_grid_get_disk_min(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        unimplemented!("Called unimplemented kernel function ds_grid_get_disk_min")
    }

    pub fn ds_grid_get_disk_mean(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        unimplemented!("Called unimplemented kernel function ds_grid_get_disk_mean")
    }

    pub fn ds_grid_value_exists(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        unimplemented!("Called unimplemented kernel function ds_grid_value_exists")
    }

    pub fn ds_grid_value_x(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        unimplemented!("Called unimplemented kernel function ds_grid_value_x")
    }

    pub fn ds_grid_value_y(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        unimplemented!("Called unimplemented kernel function ds_grid_value_y")
    }

    pub fn ds_grid_value_disk_exists(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        unimplemented!("Called unimplemented kernel function ds_grid_value_disk_exists")
    }

    pub fn ds_grid_value_disk_x(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        unimplemented!("Called unimplemented kernel function ds_grid_value_disk_x")
    }

    pub fn ds_grid_value_disk_y(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        unimplemented!("Called unimplemented kernel function ds_grid_value_disk_y")
    }

    pub fn ds_grid_shuffle(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function ds_grid_shuffle")
    }

    pub fn ds_grid_write(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function ds_grid_write")
    }

    pub fn ds_grid_read(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function ds_grid_read")
    }

    pub fn sound_play(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        //unimplemented!("Called unimplemented kernel function sound_play")
        // TODO
        Ok(Default::default())
    }

    pub fn sound_loop(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        //unimplemented!("Called unimplemented kernel function sound_loop")
        // TODO
        Ok(Default::default())
    }

    pub fn sound_stop(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        //unimplemented!("Called unimplemented kernel function sound_stop")
        // TODO
        Ok(Default::default())
    }

    pub fn sound_stop_all(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        //unimplemented!("Called unimplemented kernel function sound_stop_all")
        // TODO
        Ok(Default::default())
    }

    pub fn sound_isplaying(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        //unimplemented!("Called unimplemented kernel function sound_isplaying")
        // TODO
        Ok(Default::default())
    }

    pub fn sound_volume(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        //unimplemented!("Called unimplemented kernel function sound_volume")
        // TODO
        Ok(Default::default())
    }

    pub fn sound_fade(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function sound_fade")
    }

    pub fn sound_pan(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function sound_pan")
    }

    pub fn sound_background_tempo(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function sound_background_tempo")
    }

    pub fn sound_global_volume(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function sound_global_volume")
    }

    pub fn sound_set_search_directory(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function sound_set_search_directory")
    }

    pub fn sound_effect_set(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function sound_effect_set")
    }

    pub fn sound_effect_chorus(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 8
        unimplemented!("Called unimplemented kernel function sound_effect_chorus")
    }

    pub fn sound_effect_compressor(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 7
        unimplemented!("Called unimplemented kernel function sound_effect_compressor")
    }

    pub fn sound_effect_echo(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        unimplemented!("Called unimplemented kernel function sound_effect_echo")
    }

    pub fn sound_effect_flanger(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 8
        unimplemented!("Called unimplemented kernel function sound_effect_flanger")
    }

    pub fn sound_effect_gargle(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function sound_effect_gargle")
    }

    pub fn sound_effect_equalizer(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        unimplemented!("Called unimplemented kernel function sound_effect_equalizer")
    }

    pub fn sound_effect_reverb(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        unimplemented!("Called unimplemented kernel function sound_effect_reverb")
    }

    pub fn sound_3d_set_sound_position(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        unimplemented!("Called unimplemented kernel function sound_3d_set_sound_position")
    }

    pub fn sound_3d_set_sound_velocity(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        unimplemented!("Called unimplemented kernel function sound_3d_set_sound_velocity")
    }

    pub fn sound_3d_set_sound_distance(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function sound_3d_set_sound_distance")
    }

    pub fn sound_3d_set_sound_cone(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 7
        unimplemented!("Called unimplemented kernel function sound_3d_set_sound_cone")
    }

    pub fn cd_init(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function cd_init")
    }

    pub fn cd_present(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function cd_present")
    }

    pub fn cd_number(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function cd_number")
    }

    pub fn cd_playing(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function cd_playing")
    }

    pub fn cd_paused(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function cd_paused")
    }

    pub fn cd_track(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function cd_track")
    }

    pub fn cd_length(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function cd_length")
    }

    pub fn cd_track_length(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function cd_track_length")
    }

    pub fn cd_position(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function cd_position")
    }

    pub fn cd_track_position(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function cd_track_position")
    }

    pub fn cd_play(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function cd_play")
    }

    pub fn cd_stop(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function cd_stop")
    }

    pub fn cd_pause(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function cd_pause")
    }

    pub fn cd_resume(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function cd_resume")
    }

    pub fn cd_set_position(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function cd_set_position")
    }

    pub fn cd_set_track_position(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function cd_set_track_position")
    }

    pub fn cd_open_door(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function cd_open_door")
    }

    pub fn cd_close_door(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function cd_close_door")
    }

    pub fn mci_command(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function MCI_command")
    }

    pub fn d3d_start(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function d3d_start")
    }

    pub fn d3d_end(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function d3d_end")
    }

    pub fn d3d_set_perspective(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function d3d_set_perspective")
    }

    pub fn d3d_set_hidden(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function d3d_set_hidden")
    }

    pub fn d3d_set_depth(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function d3d_set_depth")
    }

    pub fn d3d_set_zwriteenable(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function d3d_set_zwriteenable")
    }

    pub fn d3d_set_lighting(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function d3d_set_lighting")
    }

    pub fn d3d_set_shading(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function d3d_set_shading")
    }

    pub fn d3d_set_fog(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        unimplemented!("Called unimplemented kernel function d3d_set_fog")
    }

    pub fn d3d_set_culling(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function d3d_set_culling")
    }

    pub fn d3d_primitive_begin(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function d3d_primitive_begin")
    }

    pub fn d3d_primitive_begin_texture(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function d3d_primitive_begin_texture")
    }

    pub fn d3d_primitive_end(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function d3d_primitive_end")
    }

    pub fn d3d_vertex(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function d3d_vertex")
    }

    pub fn d3d_vertex_color(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        unimplemented!("Called unimplemented kernel function d3d_vertex_color")
    }

    pub fn d3d_vertex_texture(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        unimplemented!("Called unimplemented kernel function d3d_vertex_texture")
    }

    pub fn d3d_vertex_texture_color(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 7
        unimplemented!("Called unimplemented kernel function d3d_vertex_texture_color")
    }

    pub fn d3d_vertex_normal(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        unimplemented!("Called unimplemented kernel function d3d_vertex_normal")
    }

    pub fn d3d_vertex_normal_color(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 8
        unimplemented!("Called unimplemented kernel function d3d_vertex_normal_color")
    }

    pub fn d3d_vertex_normal_texture(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 8
        unimplemented!("Called unimplemented kernel function d3d_vertex_normal_texture")
    }

    pub fn d3d_vertex_normal_texture_color(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 10
        unimplemented!("Called unimplemented kernel function d3d_vertex_normal_texture_color")
    }

    pub fn d3d_draw_block(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 9
        unimplemented!("Called unimplemented kernel function d3d_draw_block")
    }

    pub fn d3d_draw_cylinder(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 11
        unimplemented!("Called unimplemented kernel function d3d_draw_cylinder")
    }

    pub fn d3d_draw_cone(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 11
        unimplemented!("Called unimplemented kernel function d3d_draw_cone")
    }

    pub fn d3d_draw_ellipsoid(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 10
        unimplemented!("Called unimplemented kernel function d3d_draw_ellipsoid")
    }

    pub fn d3d_draw_wall(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 9
        unimplemented!("Called unimplemented kernel function d3d_draw_wall")
    }

    pub fn d3d_draw_floor(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 9
        unimplemented!("Called unimplemented kernel function d3d_draw_floor")
    }

    pub fn d3d_set_projection(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 9
        unimplemented!("Called unimplemented kernel function d3d_set_projection")
    }

    pub fn d3d_set_projection_ext(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 13
        unimplemented!("Called unimplemented kernel function d3d_set_projection_ext")
    }

    pub fn d3d_set_projection_ortho(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        unimplemented!("Called unimplemented kernel function d3d_set_projection_ortho")
    }

    pub fn d3d_set_projection_perspective(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        unimplemented!("Called unimplemented kernel function d3d_set_projection_perspective")
    }

    pub fn d3d_transform_set_identity(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function d3d_transform_set_identity")
    }

    pub fn d3d_transform_set_translation(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function d3d_transform_set_translation")
    }

    pub fn d3d_transform_set_scaling(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function d3d_transform_set_scaling")
    }

    pub fn d3d_transform_set_rotation_x(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function d3d_transform_set_rotation_x")
    }

    pub fn d3d_transform_set_rotation_y(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function d3d_transform_set_rotation_y")
    }

    pub fn d3d_transform_set_rotation_z(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function d3d_transform_set_rotation_z")
    }

    pub fn d3d_transform_set_rotation_axis(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        unimplemented!("Called unimplemented kernel function d3d_transform_set_rotation_axis")
    }

    pub fn d3d_transform_add_translation(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function d3d_transform_add_translation")
    }

    pub fn d3d_transform_add_scaling(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        unimplemented!("Called unimplemented kernel function d3d_transform_add_scaling")
    }

    pub fn d3d_transform_add_rotation_x(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function d3d_transform_add_rotation_x")
    }

    pub fn d3d_transform_add_rotation_y(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function d3d_transform_add_rotation_y")
    }

    pub fn d3d_transform_add_rotation_z(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function d3d_transform_add_rotation_z")
    }

    pub fn d3d_transform_add_rotation_axis(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        unimplemented!("Called unimplemented kernel function d3d_transform_add_rotation_axis")
    }

    pub fn d3d_transform_stack_clear(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function d3d_transform_stack_clear")
    }

    pub fn d3d_transform_stack_empty(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function d3d_transform_stack_empty")
    }

    pub fn d3d_transform_stack_push(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function d3d_transform_stack_push")
    }

    pub fn d3d_transform_stack_pop(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function d3d_transform_stack_pop")
    }

    pub fn d3d_transform_stack_top(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function d3d_transform_stack_top")
    }

    pub fn d3d_transform_stack_discard(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function d3d_transform_stack_discard")
    }

    pub fn d3d_light_define_ambient(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function d3d_light_define_ambient")
    }

    pub fn d3d_light_define_direction(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        unimplemented!("Called unimplemented kernel function d3d_light_define_direction")
    }

    pub fn d3d_light_define_point(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        unimplemented!("Called unimplemented kernel function d3d_light_define_point")
    }

    pub fn d3d_light_enable(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function d3d_light_enable")
    }

    pub fn d3d_model_create(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        unimplemented!("Called unimplemented kernel function d3d_model_create")
    }

    pub fn d3d_model_destroy(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function d3d_model_destroy")
    }

    pub fn d3d_model_clear(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function d3d_model_clear")
    }

    pub fn d3d_model_load(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function d3d_model_load")
    }

    pub fn d3d_model_save(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function d3d_model_save")
    }

    pub fn d3d_model_draw(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        unimplemented!("Called unimplemented kernel function d3d_model_draw")
    }

    pub fn d3d_model_primitive_begin(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        unimplemented!("Called unimplemented kernel function d3d_model_primitive_begin")
    }

    pub fn d3d_model_primitive_end(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        unimplemented!("Called unimplemented kernel function d3d_model_primitive_end")
    }

    pub fn d3d_model_vertex(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        unimplemented!("Called unimplemented kernel function d3d_model_vertex")
    }

    pub fn d3d_model_vertex_color(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        unimplemented!("Called unimplemented kernel function d3d_model_vertex_color")
    }

    pub fn d3d_model_vertex_texture(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        unimplemented!("Called unimplemented kernel function d3d_model_vertex_texture")
    }

    pub fn d3d_model_vertex_texture_color(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 8
        unimplemented!("Called unimplemented kernel function d3d_model_vertex_texture_color")
    }

    pub fn d3d_model_vertex_normal(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 7
        unimplemented!("Called unimplemented kernel function d3d_model_vertex_normal")
    }

    pub fn d3d_model_vertex_normal_color(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 9
        unimplemented!("Called unimplemented kernel function d3d_model_vertex_normal_color")
    }

    pub fn d3d_model_vertex_normal_texture(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 9
        unimplemented!("Called unimplemented kernel function d3d_model_vertex_normal_texture")
    }

    pub fn d3d_model_vertex_normal_texture_color(
        &mut self,
        _context: &mut Context,
        _args: &[Value],
    ) -> gml::Result<Value> {
        // Expected arg count: 11
        unimplemented!("Called unimplemented kernel function d3d_model_vertex_normal_texture_color")
    }

    pub fn d3d_model_block(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 9
        unimplemented!("Called unimplemented kernel function d3d_model_block")
    }

    pub fn d3d_model_cylinder(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 11
        unimplemented!("Called unimplemented kernel function d3d_model_cylinder")
    }

    pub fn d3d_model_cone(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 11
        unimplemented!("Called unimplemented kernel function d3d_model_cone")
    }

    pub fn d3d_model_ellipsoid(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 10
        unimplemented!("Called unimplemented kernel function d3d_model_ellipsoid")
    }

    pub fn d3d_model_wall(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 9
        unimplemented!("Called unimplemented kernel function d3d_model_wall")
    }

    pub fn d3d_model_floor(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 9
        unimplemented!("Called unimplemented kernel function d3d_model_floor")
    }
}
