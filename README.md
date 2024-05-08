# term_video_cli
### Command line interface for converting videos etc. and playing them as ascii_art videos from Terminal.

* **必ずREADMEを最後まで読んでから、使用してください。**  
* **Be sure to read the README to the end before use.**

## Example
### **[Youtube](https://youtu.be/82GU84CVCsI?si=PxKNbz6ZLI79hvsw)**  
<img src="example/sample01.png" width="600">

## Features
* Ability to download Youtube videos
* Ascii_video conversion of videos and display in Terminal
* Resizing and displaying videos to fit the terminal size
* **Frame rate: 24FPS**

## Usage
### 1. executable file
1. Download the Youtube video you want to convert.

```zsh
 $ term_video_cli -u [Youtube_video_url]
```
2. Pass the path to the video file you just downloaded to the -f option.

```zsh
 $ term_video_cli -f [Path_to_downloaded_video_file]
```

**※ Ascii_video play only after conversion. (will be played back in the same size as when converted)**
```zsh
 $ term_video_cli -p
```
### 2. Build from source code
1. Clone the repository and build or run
```zsh
 $ git clone https://github.com/SL9-1994/term_video_cli.git
```

2. Download the Youtube video you want to convert.
```zsh
 $ cargo run -- -u [Youtube_video_url]
```

3. Pass the path to the video file you just downloaded to the -f option.
```zsh
 $ cargo run -- -f [Path_to_downloaded_video_file]
```

**※ Ascii_video play only after conversion. (will be played back in the same size as when converted)**
```zsh
 $ cargo run -- -p
```
## Critical information

### Japanese
1. このCLIを使用して、ダウンロードした動画を他者に公開・再配布することは絶対にやめてください。

2. 違法アップロードされた動画(アニメ, ドラマ等...)を、ダウンロードすることは絶対にやめてください。

> 本ソフトウェアは素人が作成したものであり、重大なバグやエラーが含まれている可能性があります。  
> 私は本ソフトウェアを使用して発生した、いかなる問題にも責任を負いません。   
> 使用者の自己責任で使用してください。  

### English
1. Please do not use this CLI to publish or redistribute downloaded videos to others.

2. Never download illegally uploaded videos (anime, drama, etc...) 2. please do not download illegally uploaded videos (anime, dramas, etc...)

> This software was created by a novice and may contain bugs and errors.  
> I take no responsibility for any problems that may arise from the use of this software.  
> Use at user's own risk.

## Dependency
* ffmpeg      "4.4.2-0ubuntu0.22.04.1" => apt install ffmpeg
* clap        "4.5.4"
* image       "0.25.1"
* rusty_ytdl  "0.7.1"
* term_size   "0.3.2"
* tokio       "1.37.0"

## License
This project is licensed under the [MIT license](/LICENSE).
