mod structs;

use std::{env, fs};
use headless_chrome::{Browser, LaunchOptions};
use headless_chrome::types::PrintToPdfOptions;
use reqwest;
use structs::NftsResponse;
use qrcode_generator::QrCodeEcc;

const URL: &str = "https://api.multiversx.com/accounts/{}/nfts?size=300";

fn main() -> Result<(), ()> {
    let args: Vec<_> = env::args().skip(1).collect();
    if args.len() != 2 {
        panic!("Required arguments: output file, wallet");
    }
    let output_file = &args[0];
    let wallet = &args[1];

    let mut html = String::new();

    html.push_str("<div style='display: grid; grid-template-columns: repeat(5, 1fr); grid-gap: 10px;'>");
    append_each_nft(&wallet, &mut html);
    html.push_str("</div>");
    append_style(&mut html);

    fs::write("screenshot.html", &html).expect("Unable to write file");

    generate_write_pdf(output_file);

    Ok(())
}

fn get_response(wallet: &String) -> reqwest::Result<Vec<NftsResponse>> {
    let client = reqwest::blocking::Client::new();
    let new_url = URL.replace("{}", wallet);
    client.get(new_url)
        .send()
        .unwrap().json::<Vec<NftsResponse>>()
}

fn append_each_nft(wallet: &&String, html: &mut String) {
    let response_full = get_response(&wallet);
    response_full.unwrap().iter().for_each(|x| {
        let url_spotlight = format!("https://xspotlight.com/nfts/{}", x.identifier);
        let result: String = qrcode_generator::to_svg_to_string(url_spotlight, QrCodeEcc::Low, 1024, None::<&str>).unwrap();
        let result = result.replace("<?xml version=\"1.0\" encoding=\"utf-8\"?>", "");
        let result = result.replace("width=\"1024\" height=\"1024\"", "viewBox=\"0 0 1024 1024\"");

        html.push_str(&format!("<div class='svgroot'>{}</div>", &result));
    });
}

fn generate_write_pdf(output_file: &String) {
    let browser = Browser::new(LaunchOptions {
        headless: true,
        window_size: Some((595, 842)),
        ..Default::default()
    }).expect("Failed to launch browser");

    let tab = browser.new_tab().expect("TODO: panic message");
    let path = env::current_dir().unwrap();
    let url = format!("file://{}", path.join("screenshot.html").to_str().unwrap());

    tab.navigate_to(&url).expect("TODO: panic message");

    tab.wait_until_navigated().expect("TODO: panic message");

    let pdf = tab.print_to_pdf(Some(PrintToPdfOptions {
        prefer_css_page_size: Some(true),
        ..Default::default()
    })).expect("Couldn't print to pdf");

    fs::write(output_file, &pdf).expect("Unable to write file");
    fs::remove_file("screenshot.html").expect("Unable to remove file");
}


fn append_style(html: &mut String) {
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
}