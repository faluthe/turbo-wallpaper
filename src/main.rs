use std::{thread, time::Duration, ffi::OsStr, fs, path::Path, io::{self, Write, Cursor}};

use directories::UserDirs;
use image::{io::Reader, Rgba, DynamicImage, imageops};
use imageproc::drawing;
use rand::seq::IteratorRandom;
use reqwest::blocking;
use rusttype::{Font, Scale};
use scraper::{Html, Selector};
use windows::{Win32::{UI::Shell::{IDesktopWallpaper, DesktopWallpaper}, System::Com::{CoInitialize, CoCreateInstance, CLSCTX_ALL}}, core::{PCWSTR, HSTRING}};

fn generate_wallpaper(base: &DynamicImage, font: &Font, scale: Scale, x: u32, y: u32, path: &OsStr) {
    let time = chrono::offset::Local::now().time().format("%I:%M").to_string();
    let mut nwall = drawing::draw_text(base, Rgba{ 0: [0, 0, 0, 255] }, x as i32, y as i32, scale, font, &time);
    drawing::draw_text_mut(&mut nwall, Rgba{ 0: [255, 255, 255, 255] }, x as i32 - 10, y as i32 - 3, scale, font, &time);
    // let nwall = imageops::resize(&nwall, 1920, 1080, imageops::FilterType::Lanczos3);
    nwall.save(path).unwrap();
}

fn download_url(url: &str, outpath: &str) {
    let bytes = blocking::get(url).unwrap().bytes().unwrap();
    let mut f = fs::File::create(outpath).unwrap();
    f.write_all(bytes.as_ref()).unwrap();
}

fn grab_wallpapers() {
    if Path::new("src/wallpapers").exists() {
        println!("Not yet implemented");
    } else {
        fs::create_dir("src/wallpapers").unwrap();
        let mut term = String::new();
        println!("Wallpapers not found\nEnter a search term: ");
        io::stdin().read_line(&mut term).unwrap();

        let search = blocking::get("https://unsplash.com/s/photos/".to_string() + &term.replace(" ", "-"))
            .unwrap()
            .text()
            .unwrap();
        let search = Html::parse_document(&search);
        let selector = Selector::parse("a.rEAWd").unwrap();
        for (i, anchor) in search.select(&selector).enumerate() {
            let href = anchor.value().attr("href").unwrap();
            let photo = blocking::get("https://unsplash.com".to_owned() + href)
                .unwrap()
                .text()
                .unwrap();
            let photo = Html::parse_document(&photo);
            let selector = Selector::parse("img.YVj9w").unwrap();
            let pic = photo.select(&selector)
                .next()
                .unwrap()
                .value()
                .attr("src")
                .unwrap();
            download_url(pic, format!("src/wallpapers/{i}").as_str());
        }
    }
}

fn main() {
    grab_wallpapers();
    let mut rng = rand::thread_rng();
    let files = fs::read_dir("src/wallpapers").unwrap();
    let file = files.choose(&mut rng).unwrap().unwrap();
    let img = fs::read(file.path()).unwrap();
    let img = Reader::new(Cursor::new(img))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();
    let font = include_bytes!("fonts/Fonarto.ttf");
    let font = Font::try_from_bytes(font).unwrap();
    let scale = Scale { x: (img.width() / 13) as f32, y: (img.height() / 8) as f32 };
    let x = img.width() / 2 - scale.x as u32;
    let y = img.height() / 2 - (scale.y / 2.0) as u32;
    let path = UserDirs::new().unwrap();
    let path = path.picture_dir().unwrap().join("turbo-wallpaper");
    fs::create_dir_all(&path).unwrap();
    let path = path.join("paper.png");
    let path = path.as_os_str();
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
