/// Throws around poor `Box`'d, fake versions of the matrix & canvas around to
/// minimally mock how the C++ library works for tests when not running on a
/// raspberry pi.
use crate::c;
use libc::{c_char, c_int};
use std::boxed::Box;

struct FakeCanvas {
    width: usize,
    height: usize,
}

impl Default for FakeCanvas {
    fn default() -> Self {
        Self {
            width: 64,
            height: 32,
        }
    }
}

struct FakeMatrix {
    main_canvas: FakeCanvas,
    offscreen_canvas: Option<FakeCanvas>,
}

impl Default for FakeMatrix {
    fn default() -> Self {
        FakeMatrix {
            main_canvas: Default::default(),
            offscreen_canvas: None,
        }
    }
}

pub(crate) extern "C" fn led_matrix_create_from_options(
    _options: *const c::LedMatrixOptions,
    _argc: *mut c_int,
    _argv: *mut *mut *mut c_char,
) -> *mut c::LedMatrix {
    let fake_matrix = Box::<FakeMatrix>::new(Default::default());
    Box::into_raw(fake_matrix) as *mut c::LedMatrix
}

pub(crate) extern "C" fn led_matrix_delete(matrix: *mut c::LedMatrix) {
    unsafe {
        let _: Box<FakeMatrix> = Box::from_raw(matrix as *mut FakeMatrix);
    }
}

pub(crate) extern "C" fn led_matrix_get_canvas(matrix: *mut c::LedMatrix) -> *mut c::LedCanvas {
    let fm_ptr: *mut FakeMatrix = matrix as *mut FakeMatrix;
    unsafe { &mut (*fm_ptr).main_canvas as *const _ as *mut c::LedCanvas }
}

pub(crate) extern "C" fn led_canvas_get_size(
    canvas: *const c::LedCanvas,
    width: *mut c_int,
    height: *mut c_int,
) {
    let fm_ptr: *mut FakeCanvas = canvas as *mut FakeCanvas;
    unsafe {
        *width = (*fm_ptr).width as c_int;
        *height = (*fm_ptr).height as c_int;
    }
}

pub(crate) extern "C" fn led_canvas_set_pixel(
    _canvas: *mut c::LedCanvas,
    _x: c_int,
    _y: c_int,
    _r: u8,
    _g: u8,
    _b: u8,
) {
}

pub(crate) extern "C" fn led_canvas_clear(_canvas: *mut c::LedCanvas) {}

pub(crate) extern "C" fn led_canvas_fill(_canvas: *mut c::LedCanvas, _r: u8, _g: u8, _b: u8) {}

pub(crate) extern "C" fn led_matrix_create_offscreen_canvas(
    matrix: *mut c::LedMatrix,
) -> *mut c::LedCanvas {
    let fm_ptr: *mut FakeMatrix = matrix as *mut FakeMatrix;
    unsafe {
        (*fm_ptr).offscreen_canvas = Some(Default::default());
        (*fm_ptr).offscreen_canvas.as_mut().unwrap() as *const _ as *mut c::LedCanvas
    }
}

pub(crate) extern "C" fn led_matrix_swap_on_vsync(
    _matrix: *mut c::LedMatrix,
    canvas: *mut c::LedCanvas,
) -> *mut c::LedCanvas {
    canvas
}

pub(crate) extern "C" fn load_font(_bdf_font_file: *const c_char) -> *mut c::LedFont {
    let fake_matrix = Box::<u8>::new(0);
    Box::into_raw(fake_matrix) as *mut c::LedFont
}

pub(crate) extern "C" fn delete_font(_font: *mut c::LedFont) {}

pub(crate) extern "C" fn draw_text(
    _canvas: *mut c::LedCanvas,
    _font: *const c::LedFont,
    _x: c_int,
    _y: c_int,
    _r: u8,
    _g: u8,
    _b: u8,
    _utf8_text: *const c_char,
    _kerning_offset: c_int,
) -> c_int {
    let value: c_int = 0;
    value
}

pub(crate) extern "C" fn vertical_draw_text(
    _canvas: *mut c::LedCanvas,
    _font: *const c::LedFont,
    _x: c_int,
    _y: c_int,
    _r: u8,
    _g: u8,
    _b: u8,
    _utf8_text: *const c_char,
    _kerning_offset: c_int,
) -> c_int {
    let value: c_int = 0;
    value
}

pub(crate) extern "C" fn draw_circle(
    _canvas: *mut c::LedCanvas,
    _x: c_int,
    _y: c_int,
    _radius: c_int,
    _r: u8,
    _g: u8,
    _b: u8,
) {
}

pub(crate) extern "C" fn draw_line(
    _canvas: *mut c::LedCanvas,
    _x0: c_int,
    _y0: c_int,
    _x1: c_int,
    _y1: c_int,
    _r: u8,
    _g: u8,
    _b: u8,
) {
}
