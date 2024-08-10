<details>
<summary>Table of Contents</summary>

- [Thamburaan](#Thamburaan)
- [Installation](#installation)
    
    -[Modes](#modes)
    -[Compile](#compile)
    -[Note](#note)
    -[Download](#downloads)
        -[Linux](#linux)
        -[windows](#windows)

- [Introduction](#introduction)
- [Usage](#usage)
  - [if you compilee](#if-you-compile)
  - [generated video](#generated-video)
- [APIs Used](#apis-used)
- [Licesnse](#license)

</details>

<!-- <img align="left" src="https://github.com/Aavtic/ena/releases/download/tags/ena-logo.png" alt="drawing" width="200"/> -->

# Thamburaan

Thamburaan is a auto-caption program for generation word by word captioning on a green-screen video. The voice is AI generated and provided by [CloudTTS]
&nbsp;

<br>
</br>

## Installation


### Modes
<details>
<summary>Modes of Installation</summary>
- [Manually Compiling](#Compile)
- [Downloading-executable](#Downloads)
</details>

### Compile
You can clone this repo using git 
```shell
git clone https://github.com/Aavtic/thamburaaan
```
Thamburaan is written in Rust and you will have to install [rustup] to compile this program.

### Note 
You have to be in the same folder to run the next command.

You can install the requiremensts using this command.
```shell
cargo build
```
### Downloads
You can download the compiled program and run it directly.

#### Linux
You can download [ForLinux]

#### Windows
You can download [ForWindows]

## Introduction

Thamburaan is a software which generates generates cool word-by-word captioning for videos using Text To Speech. We use [CloudTTS] to generate audio, which is a free online TTS service.

## Usage

Once you have the program either compiled or downloaded you can run the program by the following commands.

### If you compiled 
    You can find `thamburaan`, the compiled program in the `./target/debug/` folder for the compiled binary.

Once everything is set. You can execute the program by this command
The --text or -t flag can be used to feed in the text to be generated as auto-captions in your video.
```shell
    thamburaan -t Hello Friend how are you!
```
This command will generate word-by-word captioned video in the `output` directory. The filename will be `final.mp4`

### Generated video
The video will be saved in a new folder called `output` and the file name will be prefixed with `final` along with a timestamp.


## APIs Used
- [CloudTTS]


## License

[MIT](./LICENSE)


[rustup]: https://www.rust-lang.org/tools/install
[Git Download]: https://git-scm.com/downloads
[CloudTTS]: https://cloudtts.com/about.html 

