use rusty_duplication::Capturer;
use rusty_duplication::Monitor;
use rusty_duplication::Scanner;
use rusty_duplication::FrameInfoExt;
use rusty_duplication::Error;

use crate::modules::util::Rect;

pub struct Screen {
    pub capturer: Capturer<Vec<u8>>,
    pub screen_size: Rect,
}

impl Screen {
    pub fn new() -> Result<Screen, Error> {
        let mut scanner = Scanner::new().expect("no monitor");
        let monitor = scanner.next().expect("no monitor");

        let description = monitor.dxgi_outdupl_desc();
        let capturer = Capturer::new(monitor, |size| {
            // what the slarp
            Ok(Vec::with_capacity(size))
        })?;

        let screen_size = Rect {
            x: 0,
            y: 0,
            width: description.ModeDesc.Width,
            height: description.ModeDesc.Height
        };

        let screen = Screen {
            capturer,
            screen_size,
        };

        Ok(screen)
    }

    // write area of currently captured framebuffer to buffer using specified rect
    pub fn crop(&self, crop_area: Rect, buffer: &mut Vec<u8>) {
        buffer.clear();
    
        for row in crop_area.y..(crop_area.y + crop_area.height) {
            let start_index = ((row as usize * self.screen_size.width as usize) + crop_area.x as usize) * 4;
            let end_index = start_index + (crop_area.width as usize * 4);

            buffer.extend_from_slice(&self.capturer.buffer[start_index..end_index]);
        }
    }

    pub fn capture(&mut self) {
        self.capturer.capture().expect("screencap error");
    }
}