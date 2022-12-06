use std::{thread, time::Duration, ffi::OsStr};

use directories::UserDirs;
use image::{io::Reader, Rgba, DynamicImage};
use imageproc::drawing;
use rusttype::{Font, Scale};
use windows::{Win32::{UI::Shell::{IDesktopWallpaper, DesktopWallpaper}, System::Com::{CoInitialize, CoCreateInstance, CLSCTX_ALL}}, core::{PCWSTR, HSTRING}};

pub fn generate_wallpaper(base: &DynamicImage, font: &Font, scale: Scale, x: u32, y: u32, path: &OsStr) {
    let time = chrono::offset::Local::now().time().format("%I:%M").to_string();
    let mut nwall = drawing::draw_text(base, Rgba{ 0: [0, 0, 0, 255] }, x as i32, y as i32, scale, font, &time);
    drawing::draw_text_mut(&mut nwall, Rgba{ 0: [255, 255, 255, 255] }, x as i32 - 10, y as i32 - 3, scale, font, &time);
    nwall.save(path).unwrap();
}
fn main() {
    let img = Reader::open("bottom.jpg").unwrap().decode().unwrap();
    let font = include_bytes!("/Windows/Fonts/GOTHICBI.TTF");
    let font = Font::try_from_bytes(font).unwrap();
    let scale = Scale { x: (img.width() / 13) as f32, y: (img.height() / 8) as f32 };
    let x = img.width() / 2 - scale.x as u32;
    let y = img.height() / 2 - (scale.y / 2.0) as u32;
    let path = UserDirs::new().unwrap();
    let path = path.picture_dir().unwrap().join("turbo-wallpaper\\paper.png");
    // Create directory if it doesn't exist
    let path = path.as_os_str();
    println!("path: {:?}", path);
    let wi: IDesktopWallpaper = unsafe {
        CoInitialize(None).unwrap();
        CoCreateInstance(&DesktopWallpaper, None, CLSCTX_ALL).unwrap()
    };

    loop {
        generate_wallpaper(&img, &font, scale, x, y, &path);
        // Using this work around for now: https://github.com/microsoft/windows-rs/issues/2177
        unsafe { wi.SetWallpaper(PCWSTR::null(), PCWSTR(HSTRING::from(path).as_ptr())) }
            .expect("Failed to set desktop wallpaper");
        thread::sleep(Duration::from_secs(10));
    }
}
