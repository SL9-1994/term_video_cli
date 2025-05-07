# term_video_cli

### Command line interface for converting videos etc. and playing them as ascii_art videos from Terminal.

- **必ず README を最後まで読んでから、使用してください。**
- **Be sure to read the README to the end before use.**

## Example

### **[Youtube](https://youtu.be/82GU84CVCsI?si=PxKNbz6ZLI79hvsw)**

<img src="example/sample01.png" width="600">

## Features

- Ascii_video conversion of videos and display in Terminal
- Resizing and displaying videos to fit the terminal size
- **Frame rate: 24FPS**

## Assets

| Asset                                                                                                        | Note                     |
| ------------------------------------------------------------------------------------------------------------ | ------------------------ |
| [term_video_cli.exe](https://github.com/SL9-1994/term_video_cli/releases/latest/download/term_video_cli.exe) | x86_64-pc-windows-gnu    |
| [term_video_cli](https://github.com/SL9-1994/term_video_cli/releases/latest/download/term_video_cli)         | x86_64-unknown-linux-gnu |

## Usage

### **Prerequisite:** Please have FFmpeg installed in your execution environment.

1. Prepare videos and images to be converted and displayed.
2. specify the video path with -f option and convert.
3. start playing the video in the terminal with -p option.

**Example**

```zsh
 $ term_video_cli -f [Path_to_downloaded_video_file]
 $ term_video_cli -p
```

**Help**

```zsh
 $ term_video_cli --help

 This CLI can draw and play arbitrary videos and images to the terminal.

 Usage: term_video_cli [OPTIONS]

 Options:
   -f, --file-path <CONVERT_VIDEO_PATH>  Enter the path of the video you wish to convert. (Supported extensions: mp4, mkv) Note: Since the conversion is based on the terminal size at the time this option is executed, a terminal of a different size will not be drawn correctly
   -p, --play                            Play ascii_video with the converted image already prepared in tmp
   -h, --help                            Print help
   -V, --version                         Print version
```

## Critical information

### Japanese

> 本ソフトウェアは素人が作成したものであり、重大なバグやエラーが含まれている可能性があります。  
> 私は本ソフトウェアを使用して発生した、いかなる問題にも責任を負いません。  
> 使用者の自己責任で使用してください。
> また，このソフトウェアの動作にはシステムに ffmpeg がインストールされている事と'PATH'環境変数が設定されていることが必要です．

### English

> This software was created by a novice and may contain bugs and errors.  
> I take no responsibility for any problems that may arise from the use of this software.  
> Use at user's own risk.
> The software requires ffmpeg to be installed on your system and the 'PATH' environment variable to be set.

## Dependency

- ffmpeg "4.4.2-0ubuntu0.22.04.1" => apt install ffmpeg
- clap "4.5.4"
- image "0.25.1"
- term_size "0.3.2"

## License

This project is licensed under the [GPL-3.0 License](/LICENSE).
