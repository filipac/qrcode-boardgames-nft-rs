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
    let _last_index = append_each_nft(&wallet, &mut html);

    // let mut manual: Vec<&str> = Vec::new();
    // manual.push("BOARD-25bcd6-40");
    // append_manual(manual, &mut html, &_last_index);


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

fn append_each_nft(wallet: &&String, html: &mut String) -> usize {
    let response_full = get_response(&wallet);
    let mut last_index: usize = 0;
    response_full.unwrap().iter().enumerate().for_each(|x| {
        let (index, x) = x;
        let url_spotlight = format!("https://xspotlight.com/nfts/{}", x.identifier);

        append_url_svg(html, url_spotlight, index);
        
        last_index = index;
    });

    last_index
}

fn append_url_svg(html: &mut String, url_spotlight: String, index: usize) {
    let result: String = qrcode_generator::to_svg_to_string(url_spotlight, QrCodeEcc::Low, 1024, None::<&str>).unwrap();
    let result = result.replace("<?xml version=\"1.0\" encoding=\"utf-8\"?>", "");
    let result = result.replace("width=\"1024\" height=\"1024\"", "viewBox=\"0 0 1024 1024\"");

    let show_index = true;

    let mask = match show_index {
        true => format!("<div class='svgroot'><div class='number'>{}</div><div>{}</div></div>", index + 1, result),
        false => format!("<div class='svgroot'><div>{}</div></div>", result)
    };

    html.push_str(&mask);
}

#[allow(dead_code)]
fn append_manual(identifier: Vec<&str>, html: &mut String, start_index: &usize) {
    identifier.iter().enumerate().for_each(|x| {
        let (index, x) = x;
        let url_spotlight = format!("https://xspotlight.com/nfts/{}", x);
        append_url_svg(html, url_spotlight, index + start_index + 1);
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
    // fs::remove_file("screenshot.html").expect("Unable to remove file");
}


fn append_style(html: &mut String) {
    html.push_str("<style>\
    .svgroot svg { \
        max-width: 100px; \
        max-height: 100px;  \
    }\
    .svgroot {
        display: flex; flex-direction: column; \
        position: relative; \
        max-width: 100px; \
        max-height: 100px;  \
        margin-bottom: 15px; \
    }\
    .svgroot .number { margin-bottom: 2px; }\
     @page { \
        size: A4; \
        margin: 0; \
    }\
    </style>");
}