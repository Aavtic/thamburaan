use std::time::Duration;
use serde_json::Value;
use std::io::Write;
use std::fs;



fn offset_to_timestamp(offset: u64) -> String {
    let duration = Duration::from_millis(offset);
    let hours = duration.as_secs() / 3600;
    let minutes = (duration.as_secs() % 3600) / 60;
    let seconds = duration.as_secs() % 60;
    let milliseconds = duration.subsec_millis();

    format!("{:02}:{:02}:{:02},{:03}", hours, minutes, seconds, milliseconds)

}

fn write_to_srt(filename: &str, srt_data: Vec<String>) {
    let mut file = fs::File::create(filename).unwrap();
    for line in srt_data {
        file.write_all(line.as_bytes()).unwrap();
    }
}

pub fn save_srt(speechmarks: Vec<Value>) {
    let mut srt_file_content = Vec::<String>::new();
    for (i, value) in speechmarks.iter().enumerate() {
        let offset = &value["offset"];
        let word = &value["word"].as_str().unwrap();

        let start_timestamp = offset_to_timestamp(offset.as_u64().unwrap());
        let end_timestamp = if i + 1 < speechmarks.len() {
            let end_offset = speechmarks[i+1]["offset"].as_u64().unwrap();
            offset_to_timestamp(end_offset)
        } else {
            let end_offset = offset.as_u64().unwrap() + 3000;
            offset_to_timestamp(end_offset)
        };
        
        srt_file_content.push(i.to_string());
        srt_file_content.push('\n'.to_string());
        srt_file_content.push(format!("{} --> {}\n", start_timestamp, end_timestamp));
        srt_file_content.push(word.to_string());
        if i + 1 < speechmarks.len() {
            srt_file_content.push("\n".to_string());
            srt_file_content.push("\n".to_string());
        } 
    }
    write_to_srt("output/subtitles.srt", srt_file_content);
}
