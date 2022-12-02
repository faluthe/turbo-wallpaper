use std::{thread, time::Duration};

use image::{io::Reader, Rgba};
use imageproc::drawing;
use rusttype::{Font, Scale};
use windows::{Win32::{UI::Shell::{IDesktopWallpaper, DesktopWallpaper}, System::Com::{CoInitialize, CoCreateInstance, CLSCTX_ALL}}, w, core::PCWSTR};

fn main() {
    let bot = Reader::open("").unwrap().decode().unwrap();
    let font = include_bytes!("/Windows/Fonts/GOTHICBI.TTF");
    let font = Font::try_from_bytes(font).unwrap();
    let scale = Scale { x: (bot.width() / 13) as f32, y: (bot.height() / 8) as f32 };
    let x = bot.width() / 2 - scale.x as u32;
    let y = bot.height() / 2 - (scale.y / 2.0) as u32;

    let wallpaper: IDesktopWallpaper = unsafe {
        CoInitialize(None).unwrap();
        CoCreateInstance(&DesktopWallpaper, None, CLSCTX_ALL).unwrap()
    };
    
    loop {
        let time = chrono::offset::Local::now().time().format("%I:%M").to_string();
        let mut nwall = drawing::draw_text(&bot, Rgba{ 0: [0, 0, 0, 255] }, x as i32, y as i32, scale, &font, &time);
        drawing::draw_text_mut(&mut nwall, Rgba{ 0: [255, 255, 255, 255] }, x as i32 - 10, y as i32 - 3, scale, &font, &time);
        nwall.save("").unwrap();
        unsafe { wallpaper.SetWallpaper(PCWSTR::null(), w!("")).unwrap(); }
        thread::sleep(Duration::from_secs(10));
    }
}
