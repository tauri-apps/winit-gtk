use gtk4::gdk;
use gtk4::glib;
use gtk4::prelude::*;
use winit_core::icon::{Icon, RgbaIcon};

pub trait PlatformIconExt {
    fn texture(&self) -> Option<gdk::Texture>;
}

impl PlatformIconExt for Icon {
    fn texture(&self) -> Option<gdk::Texture> {
        let icon = self.cast_ref::<RgbaIcon>()?;

        let width = i32::try_from(icon.width()).ok()?;
        let height = i32::try_from(icon.height()).ok()?;
        let stride = usize::try_from(icon.width()).ok()?.checked_mul(4)?;
        let bytes = glib::Bytes::from_owned(icon.buffer().to_vec());

        let format = gdk::MemoryFormat::R8g8b8a8;
        let texture = gdk::MemoryTexture::new(width, height, format, &bytes, stride);

        Some(texture.upcast())
    }
}
