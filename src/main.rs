mod structs;

use std::collections::HashMap;
use std::{env, fs};
use headless_chrome::{Browser, LaunchOptions};
use headless_chrome::protocol::cdp::Page::CaptureScreenshotFormatOption;
use headless_chrome::types::PrintToPdfOptions;
use reqwest;
use structs::NftsResponse;
use qrcode_generator::QrCodeEcc;

const URL: &str = "https://api.multiversx.com/accounts/erd158lk5s2m3cpjyg5fwgm0pwnt8ugnc29mj4nafkrvcrhfdfpgvp3swpmnrj/nfts?size=300";

fn main() -> Result<(), ()> {
    let args: Vec<_> = env::args().skip(1).collect();
    if args.len() != 1 {
        panic!("Missing argument: output file");
    }
    let output_file = &args[0];

    let client = reqwest::blocking::Client::new();
    let response_full = client.get(URL)
        .send()
        .unwrap().json::<Vec<NftsResponse>>();

    let mut html = String::new();
    html.push_str("<div style='display: grid; grid-template-columns: repeat(5, 1fr); grid-gap: 10px;'>");

    response_full.unwrap().iter().for_each(|x| {
        let urlSpotlight = format!("https://xspotlight.com/nfts/{}", x.identifier);
        let result: String = qrcode_generator::to_svg_to_string(urlSpotlight, QrCodeEcc::Low, 1024, None::<&str>).unwrap();
        // replace <?xml version="1.0" encoding="utf-8"?> with empty string
        let result = result.replace("<?xml version=\"1.0\" encoding=\"utf-8\"?>", "");
        // replace width and height with viewBox
        let result = result.replace("width=\"1024\" height=\"1024\"", "viewBox=\"0 0 1024 1024\"");

        html.push_str(&format!("<div class='svgroot'>{}</div>", &result));
        // let image = client.get(&x.media.get(0).unwrap().thumbnailUrl).send().unwrap().bytes().unwrap();
        // let reader = std::io::Cursor::new(image);
    });

    html.push_str("</div>");

    html.push_str("<style>\
    .svgroot svg { \
        max-width: 100px; \
        max-height: 100px;  \
    }\
    .svgroot {
        position: relative; \
        max-width: 100px; \
        max-height: 100px;  \
        margin-bottom: 15px; \
    }\
     @page { \
        size: A4; \
        margin: 0; \
    }\
    </style>");

    // write html to file
    fs::write("screenshot.html", &html).expect("Unable to write file");

    let options = LaunchOptions::default_builder()
        .build()
        .expect("Couldn't find appropriate Chrome binary.");
    let browser = Browser::new(LaunchOptions {
        headless: true,
        // A4 size
        window_size: Some((595, 842)),
        ..Default::default()
    }).expect("Failed to launch browser");

    let tab = browser.new_tab().expect("TODO: panic message");

    // get path to current directory
    let path = env::current_dir().unwrap();

    let url = format!("file://{}", path.join("screenshot.html").to_str().unwrap());

    tab.navigate_to(&url).expect("TODO: panic message");
    
    tab.wait_until_navigated().expect("TODO: panic message");

    let pdf = tab.print_to_pdf(Some(PrintToPdfOptions {
        // A4
        prefer_css_page_size: Some(true),
        ..Default::default()
    })).expect("Couldn't print to pdf");
    fs::write(output_file, &pdf).expect("Unable to write file");
    //
    fs::remove_file("screenshot.html").expect("Unable to remove file");

    // print!("{}", &html);

    Ok(())
}
