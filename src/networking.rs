use reqwest::header::{HeaderMap, HeaderValue};
use base64::prelude::*;
use serde::Serialize;
use serde_json::Value;
use std::io::Write;
use reqwest;
use std::fs;


#[derive(Serialize, Debug)]
struct RequestJson {
    rate: u32,
    recording: bool,
    text: String,
    voice: String,
    volume: u32,
    with_speechmarks: bool 
}

fn get_headers() -> HeaderMap{
    let mut headermap = HeaderMap::new();
    headermap.insert("Accept",HeaderValue::from_static("*/*"));
    headermap.insert("Accept-Encoding",HeaderValue::from_static("gzip, deflate, br"));
    headermap.insert("Accept-Language",HeaderValue::from_static("en-US,en;q=0.5"));
    headermap.insert("Connection",HeaderValue::from_static("keep-alive"));
    headermap.insert("Content-Length",HeaderValue::from_static("126"));
    headermap.insert("Content-Type",HeaderValue::from_static("application/json"));
    headermap.insert("Host",HeaderValue::from_static("cloudtts.com"));
    headermap.insert("Origin",HeaderValue::from_static("https://cloudtts.com"));
    headermap.insert("Referer",HeaderValue::from_static("https://cloudtts.com/u/index.html"));
    headermap.insert("Sec-Fetch-Dest",HeaderValue::from_static("empty"));
    headermap.insert("Sec-Fetch-Mode",HeaderValue::from_static("cors"));
    headermap.insert("Sec-Fetch-Site",HeaderValue::from_static("same-origin"));
    headermap.insert("Sec-GPC",HeaderValue::from_static("1"));
    headermap.insert("User-Agent",HeaderValue::from_static("Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/115.0"));
    return headermap;
}

pub fn get_response(url: &str, text: &str) -> Value {
    let json_data = RequestJson{
        rate: 1,
        recording: false,
        text: text.to_string(),
        voice: "en-US-AriaNeural".to_string(),
        volume: 1,
        with_speechmarks: true,
    };
    let json_body = serde_json::to_string(&json_data).unwrap();

    let client = reqwest::blocking::Client::new();
    let response = client.post(url)
        .headers(get_headers())
        .body(json_body)
        .send().unwrap()
        .text().unwrap();
    println!("{}", response);

    let response_json:Value = serde_json::from_str(response.as_str()).unwrap();
    return response_json;
}

fn save_audio_data(b64data: &str, filename: &str) {
    let base64_decode = BASE64_STANDARD.decode(b64data).unwrap();

    let mut audio_file = fs::File::create(filename).unwrap();
    audio_file.write_all(&base64_decode).unwrap();
    println!("Audio data written successfully");
}

pub struct ParserResponse {
    pub speechmarks: Vec<Value>,
    pub _filename: String,
}

pub fn parse_response(response_json: Value) -> ParserResponse{
    let filename =  "output/audio_data.mp3";
    let audio_data = response_json["data"]["audio"].as_str().unwrap();
    let speechmarks = response_json["data"]["speechmarks"].as_array().unwrap();
    save_audio_data(audio_data, filename);
    println!("string: {}", audio_data);
    println!("speechmarks: {:?}", speechmarks);
    println!("speechmarks: {:?}, len: {}",speechmarks, speechmarks.len());
    return ParserResponse {speechmarks: speechmarks.to_vec(), _filename: filename.to_string()};
}
