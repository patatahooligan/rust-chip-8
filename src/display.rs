use std::cmp::min;

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

pub struct Display {
    buffer: [[bool; WIDTH]; HEIGHT],
}

impl Display {
    pub fn new() -> Display {
        Display {
            buffer: [[false; crate::display::WIDTH]; crate::display::HEIGHT],
        }
    }

    pub fn clear(&mut self) {
        self.buffer = [[false; crate::display::WIDTH]; crate::display::HEIGHT];
    }

    pub fn draw_sprite(
            &mut self, sprite: &[u8],
            startx: usize, starty: usize) -> bool {
        // Sprites are supposed to clip when going off screen, so figure out the
        // width of the sprite that fits on the screen. When unconstrained they
        // are 8 pixels wide.

        assert!(startx < WIDTH);
        assert!(starty < HEIGHT);

        let sprite_width = min(WIDTH - startx, 8);
        let sprite_height = min(HEIGHT - starty, sprite.len());

        let mut any_switched_off = false;
        for &row in &sprite[..sprite_height] {
            // Each bit in a byte represents a pixel in that row. Use bitmasking
            // to unpack them
            for i in 0..sprite_width {
                // Only flip pixels that are 1 in the sprite
                if row & (128 >> i) == 0 {
                    continue;
                }

                // If any pixel is flipped 1 -> 0, return value should be true
                let target_pixel = &mut self.buffer[starty][startx + i];
                any_switched_off |= *target_pixel;
                *target_pixel = !(*target_pixel);
            }
        }
        return any_switched_off;
    }
}
