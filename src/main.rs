use std::{fs, io::Cursor, thread, time::Duration};

use chrono::Local;
use directories::UserDirs;
use image::{io::Reader, Rgba, imageops::FilterType};
use imageproc::drawing;
use rand::seq::IteratorRandom;
use rusttype::{Font, Scale};
use windows::{Win32::{UI::Shell::{IDesktopWallpaper, DesktopWallpaper}, System::Com::{CoInitialize, CoCreateInstance, CLSCTX_ALL}}, core::{PCWSTR, HSTRING}};

fn main() {
    let mut rng = rand::thread_rng();
    let font = Font::try_from_bytes(include_bytes!("fonts/Fonarto.ttf")).unwrap();
    let white = Rgba([255, 255, 255, 255]);
    let black = Rgba([0, 0, 0, 255]);
    let desktop: IDesktopWallpaper = unsafe {
        CoInitialize(None).unwrap();
        CoCreateInstance(&DesktopWallpaper, None, CLSCTX_ALL).unwrap()
    };

    // Create/open dirs
    let usr_dirs = UserDirs::new().unwrap();
    let pic_dir = usr_dirs.picture_dir().unwrap().join("turbo-wallpaper");
    let in_dir = pic_dir.join("in");
    fs::create_dir_all(&in_dir).unwrap();
    let out_dir = pic_dir.join("out");
    fs::create_dir_all(&out_dir).unwrap();
    let out_path = out_dir.join("wallpaper.png");

    // Open base image
    let in_dir = fs::read_dir(in_dir).unwrap();
    let img_entry = in_dir.choose(&mut rng).expect("Couldn't find wallpaper").unwrap();
    let img = Reader::new(Cursor::new(fs::read(img_entry.path()).unwrap()))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap()
        // TODO: make this work for all monitors
        .resize_to_fill(1920, 1080, FilterType::Lanczos3);

    let w = img.width() as i32;
    let h = img.height() as i32;
    let clock_scale = Scale{ x: w as f32 / 8.0f32, y: h as f32 / 8.0f32 };
    let date_scale = Scale{ x: w as f32 / 32.0f32, y: h as f32 / 32.0f32 };
    let x = w / 2 - clock_scale.x as i32;
    let y = h / 2 - (clock_scale.y as i32 / 2);
    let mut prev_time = String::new();
    loop {
        // Edit image (only if time has changed)
        let time = Local::now().time().format("%-I:%M").to_string();
        if time != prev_time {
            // Draw time
            let mut out_img = drawing::draw_text(&img, black, x + 10, y + 7, clock_scale, &font, &time);
            drawing::draw_text_mut(&mut out_img, white, x, y, clock_scale, &font, &time);

            // Draw date
            let x = x - (clock_scale.x / 2.0f32) as i32;
            let y = y + clock_scale.y as i32 + 3;
            let date = Local::now().date_naive().format("%A, %B %-d %C%y").to_string();
            drawing::draw_text_mut(&mut out_img, black, x + 5, y + 3, date_scale, &font, &date);
            drawing::draw_text_mut(&mut out_img, white, x, y, date_scale, &font, &date);
            
            out_img.save(&out_path).unwrap();

            // Set as wallpaper
            // Using this work around for now: https://github.com/microsoft/windows-rs/issues/2177
            unsafe { desktop.SetWallpaper(PCWSTR::null(), PCWSTR(HSTRING::from(out_path.as_os_str()).as_ptr())) }
                .expect("Failed to set desktop wallpaper");
        }

        thread::sleep(Duration::from_secs(1));
        prev_time = time;
    }
}