# Hypr-empty
Spawn a runner when switching to an empty workspace on Hyprland


https://user-images.githubusercontent.com/96471299/229786216-96e08d27-ff66-4e55-a3a6-7ad376737817.mp4


## Installation

```sh 
cargo install --git=https://github.com/nate-sys/hypr-empty
```

## Usage
1) Create the file `~/.config/hypr-empty/config.toml`
2) Specify the command you want to be run and the `unique` workspace
```toml
# If you're using wofi
[[components]]
workspace = "2"
command = "wofi"
args = ["-S", "drun"]

# If you're using rofi
[[components]]
workspace = "3"
command = "rofi"
args = ["-show", "drun"]

# If you want open the browser when you arrive the workspace
[[components]]
workspace = "13"
command = "google-chrome-stable"
args = []

# arbitrary program
[[components]]
workspace = "NUMBER"
command = "name_of_program"
args = ["arg1", "arg2"]
```
3) Add `hypr-empty` to your startup apps and make sure that `.cargo/bin` is in your path
