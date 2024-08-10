use std::process::Command;

// ffmpeg -f lavfi -i color=c=black:s=1280x720:r=5 -i audio.mp3 -crf 0 -c:a copy -shortest output.mp4
pub fn convert_audio_to_video(audio_file: &str, output_file: &str){
    Command::new("ffmpeg")
        .args(["-f", "lavfi", "-i", "color=c=green:s=1280x720:r=5", "-i", audio_file, "-crf" ,"0", "-c:a", "copy", "-shortest", "-y", output_file])
        .output()
        .unwrap();
}

// ffmpeg -i video.avi -vf subtitles=subtitle.srt out.avi
pub fn burn_subtitles(subtitle_file: &str, input_file: &str, output_file: &str) {
    assert_ne!(input_file, output_file);

    Command::new("ffmpeg")
        .args(["-i", input_file, "-vf", format!("subtitles={}", subtitle_file).as_str(), "-y", output_file])
        .output()
        .unwrap();
}

