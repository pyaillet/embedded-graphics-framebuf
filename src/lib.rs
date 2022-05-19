use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::OriginDimensions,
    prelude::{PixelColor, Size, Point},
    Pixel,
};

#[repr(transparent)]
#[derive(Clone)]
pub struct FrameBuf<C: PixelColor, const X: usize, const Y: usize, const TOTAL: usize>(pub [C; TOTAL]);

pub trait AsWords<const TOTAL: usize> {
    fn as_words(&mut self) -> &mut [u16; TOTAL];
}

impl<C: PixelColor, const X: usize, const Y: usize, const TOTAL: usize> AsWords<TOTAL> for FrameBuf<C, X, Y, TOTAL> {
    fn as_words(&mut self) -> &mut [u16; TOTAL] {
        unsafe { core::mem::transmute(self) }
    }
}

impl<C: PixelColor + Default, const X: usize, const Y: usize, const TOTAL: usize> FrameBuf<C, X, Y, TOTAL> {
    /// Set all pixels to black.
    pub fn clear_black(&mut self) {
        for x in 0..X {
            for y in 0..Y {
                self.0[y * X + x] = C::default();
            }
        }
    }
}

impl<'a, C: PixelColor, const X: usize, const Y: usize, const TOTAL: usize> IntoIterator for &'a FrameBuf<C, X, Y, TOTAL> {
    type Item = C;
    type IntoIter = FrameBufIntoIterator<'a, C, X, Y, TOTAL>;

    fn into_iter(self) -> Self::IntoIter {
        FrameBufIntoIterator {
            fbuf: self,
            index: 0,
        }
    }
}
pub struct FrameBufIntoIterator<'a, C: PixelColor, const X: usize, const Y: usize, const TOTAL: usize> {
    fbuf: &'a FrameBuf<C, X, Y, TOTAL>,
    index: usize,
}

impl<'a, C: PixelColor, const X: usize, const Y: usize, const TOTAL: usize> Iterator
    for FrameBufIntoIterator<'a, C, X, Y, TOTAL>
{
    type Item = C;
    fn next(&mut self) -> Option<C> {
        let y = self.index / X;
        let x = self.index - y * X;

        if self.index >= X * Y {
            return None;
        }
        self.index += 1;
        Some(self.fbuf.0[y * X + x])
    }
}

impl<C: PixelColor, const X: usize, const Y: usize, const TOTAL: usize> OriginDimensions for FrameBuf<C, X, Y, TOTAL> {
    fn size(&self) -> Size {
        Size::new(X as u32, Y as u32)
    }
}

impl<C: PixelColor, const X: usize, const Y: usize, const TOTAL: usize> DrawTarget for FrameBuf<C, X, Y, TOTAL> {
    type Color = C;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            if coord.x >= 0 && coord.x < X as i32 && coord.y >= 0 && coord.y < Y as i32 {
                let Point { x, y } = coord;
                self.0[y as usize * X + x as usize] = color;
            }
        }
        Ok(())
    }

    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        for x in 0..X {
            for y in 0..Y {
                self.0[y * X + x] = color;
            }
        }
        Ok(())
    }
}
