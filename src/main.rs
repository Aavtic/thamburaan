use std::env;
mod srt_func;
mod networking;
mod process_video;
mod utils;


fn argument_parser() -> String{
    let arguments = ["--text", "-t"];
    let args = env::args().collect::<Vec<String>>();
    let mut text = String::new();

    if args.len() > 2 {
        if arguments.contains(&args[1].as_str()) {
            let text_iter = args.iter().skip(2);

            for arg in text_iter {
                text.push_str(format!(" {}", arg).as_str());
            }
        } else {
            eprintln!("ERROR: Expected flag --text or -t\nCheck thamburaan --help for more information.");
            std::process::exit(-1);
        }
    } else {
        eprintln!("ERROR: Not enough arguments.\nCheck thamburaan --help for more information.");
        std::process::exit(-1);
    }

    return text;

}

fn main(){
    let text = argument_parser();

    let output_directory = "./output/";
    if !utils::dir_exists(output_directory) {
        match utils::create_dir(output_directory) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Could not create output directory!\nERROR: {}", e);
                std::process::exit(-1);
            }
        }
    }

    let response = networking::get_response("https://cloudtts.com:443/api/get_audio", &text); let parsed_values = networking::parse_response(response);
    let speechmarks = parsed_values.speechmarks;
    srt_func::save_srt(speechmarks);

    process_video::convert_audio_to_video("output/audio_data.mp3", "output/output.mp4");
    process_video::burn_subtitles("output/subtitles.srt", "output/output.mp4", "output/final.mp4");
}
