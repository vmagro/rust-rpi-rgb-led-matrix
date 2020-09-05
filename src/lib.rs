extern crate libc;
mod c;
mod led_color;
#[cfg(feature = "rgbmatrix-mock")]
pub(crate) mod rgbmatrix_mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "embeddedgraphics")]
use embedded_graphics::{drawable::Pixel, geometry::Size, pixelcolor::PixelColor, DrawTarget};
use libc::{c_char, c_int};
use std::ffi::CString;
use std::path::Path;
use std::ptr::null;

pub use c::LedMatrixOptions;
pub use led_color::LedColor;

pub struct LedCanvas {
    handle: *mut c::LedCanvas,
}

pub struct LedMatrix {
    handle: *mut c::LedMatrix,
    _options: LedMatrixOptions,
}

pub struct LedFont {
    handle: *mut c::LedFont,
}

#[cfg_attr(feature = "rgbmatrix-mock", allow(unused_unsafe))]
impl LedMatrix {
    pub fn new(options: Option<LedMatrixOptions>) -> Result<LedMatrix, &'static str> {
        let options = {
            if let Some(o) = options {
                o
            } else {
                LedMatrixOptions::new()
            }
        };

        let handle = unsafe {
            c::led_matrix_create_from_options(
                &options as *const LedMatrixOptions,
                null::<c_int>() as *mut c_int,
                null::<i64>() as *mut *mut *mut c_char,
            )
        };

        if handle.is_null() {
            Err("Couldn't create LedMatrix")
        } else {
            Ok(LedMatrix {
                handle,
                _options: options,
            })
        }
    }

    pub fn canvas(&self) -> LedCanvas {
        let handle = unsafe { c::led_matrix_get_canvas(self.handle) };

        LedCanvas { handle }
    }

    pub fn offscreen_canvas(&self) -> LedCanvas {
        let handle = unsafe { c::led_matrix_create_offscreen_canvas(self.handle) };

        LedCanvas { handle }
    }

    pub fn swap(&self, canvas: LedCanvas) -> LedCanvas {
        let handle = unsafe { c::led_matrix_swap_on_vsync(self.handle, canvas.handle) };

        LedCanvas { handle }
    }
}

#[cfg_attr(feature = "rgbmatrix-mock", allow(unused_unsafe))]
impl Drop for LedMatrix {
    fn drop(&mut self) {
        unsafe {
            c::led_matrix_delete(self.handle);
        }
    }
}

#[cfg_attr(feature = "rgbmatrix-mock", allow(unused_unsafe))]
impl LedFont {
    pub fn new(bdf_file: &Path) -> Result<LedFont, &'static str> {
        let string = match bdf_file.to_str() {
            Some(s) => s,
            None => return Err("Couldn't convert path to str"),
        };
        let cstring = CString::new(string).unwrap();

        let handle = unsafe { c::load_font(cstring.as_ptr()) };

        if handle.is_null() {
            Err("Couldn't load font")
        } else {
            Ok(LedFont { handle })
        }
    }
}

#[cfg_attr(feature = "rgbmatrix-mock", allow(unused_unsafe))]
impl Drop for LedFont {
    fn drop(&mut self) {
        unsafe { c::delete_font(self.handle) }
    }
}

#[cfg_attr(feature = "rgbmatrix-mock", allow(unused_unsafe))]
impl LedCanvas {
    pub fn size(&self) -> (i32, i32) {
        let (mut width, mut height): (c_int, c_int) = (0, 0);
        unsafe {
            c::led_canvas_get_size(
                self.handle,
                &mut width as *mut c_int,
                &mut height as *mut c_int,
            );
        }
        (width as i32, height as i32)
    }

    pub fn set(&mut self, x: i32, y: i32, color: &LedColor) {
        unsafe {
            c::led_canvas_set_pixel(
                self.handle,
                x as c_int,
                y as c_int,
                color.red,
                color.green,
                color.blue,
            )
        }
    }

    pub fn clear(&mut self) {
        unsafe {
            c::led_canvas_clear(self.handle);
        }
    }

    pub fn fill(&mut self, color: &LedColor) {
        unsafe {
            c::led_canvas_fill(self.handle, color.red, color.green, color.blue);
        }
    }

    pub fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: &LedColor) {
        unsafe {
            c::draw_line(
                self.handle,
                x0,
                y0,
                x1,
                y1,
                color.red,
                color.green,
                color.blue,
            );
        }
    }

    pub fn draw_circle(&mut self, x: i32, y: i32, radius: u32, color: &LedColor) {
        unsafe {
            c::draw_circle(
                self.handle,
                x as c_int,
                y as c_int,
                radius as c_int,
                color.red,
                color.green,
                color.blue,
            );
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn draw_text(
        &mut self,
        font: &LedFont,
        text: &str,
        x: i32,
        y: i32,
        color: &LedColor,
        kerning_offset: i32,
        vertical: bool,
    ) -> i32 {
        let ctext = CString::new(text).unwrap();
        unsafe {
            if vertical {
                c::vertical_draw_text(
                    self.handle,
                    font.handle,
                    x as c_int,
                    y as c_int,
                    color.red,
                    color.green,
                    color.blue,
                    ctext.as_ptr(),
                    kerning_offset as c_int,
                ) as i32
            } else {
                c::draw_text(
                    self.handle,
                    font.handle,
                    x as c_int,
                    y as c_int,
                    color.red,
                    color.green,
                    color.blue,
                    ctext.as_ptr(),
                    kerning_offset as c_int,
                ) as i32
            }
        }
    }
}

#[cfg(feature = "embeddedgraphics")]
impl<C> DrawTarget<C> for LedCanvas
where
    C: Into<LedColor> + PixelColor,
{
    type Error = core::convert::Infallible;

    fn draw_pixel(&mut self, item: Pixel<C>) -> Result<(), Self::Error> {
        let Pixel(point, color) = item;
        self.set(point.x, point.y, &color.into());
        Ok(())
    }

    fn size(&self) -> Size {
        let size = self.size();
        Size::new(size.0 as u32, size.1 as u32)
    }

    fn clear(&mut self, color: C) -> Result<(), Self::Error> {
        self.fill(&color.into());
        Ok(())
    }
}
