# namefrench - the tool to french your name!!!!
<img src="write-your-name-in-french.jpg" height=250px alt="Image meme showing the lowercase letters a through z being assigned to strings in what is jokingly referred to as french. Additionally, stereotypical french memes are also on the image."></img>
**This is obviously a joke project.**
## Use
Once installed you can just run `namefrench ` and then chain as many arguments as you want!! they will be output without the spaces in between them though, converted to "french" in the stdout!!

## Build
You need to install [cargo and rust](https://doc.rust-lang.org/cargo/getting-started/installation.html) to build it.
Once you've cloned the repository (you can use `git clone https://github.com/moddingforreal/namefrench`) run `cargo build` to build or `cargo run <args>` to immediately run the program. Append the `--release` flag to either of them to optimize for release.

## Installation
Follow the steps in the [Build](#Build) section to install cargo and rust, and to clone and build the project.
You can then install namefrench by running `cargo install --path .`.
If you would like to instead install a debug build, run `cargo install --path . --debug`.
Keep in mind that this will ignore the Cargo.lock file by default. To make sure it is respected you can add `--locked` to the end of your command.

Cargo will by default install rust binaries to `~/.cargo/bin` so add it to your `PATH` environment variable to make sure namefrench is globally discoverable so you can run it from your terminal with `namefrench`.

## Demo
![Terminal showing the output of `namefrench "Avery Solara"` to be `éeVursilitéeEuaoixri grec Jean-pierre974🇱éerée`](demos/namefrench-avery-solara.jpg)
<sup><sub>A normal name in `namefrench`</sub></sup>

![Terminal showing the output of `namefrench "I will use = special ; symbols ! 123´ to be `Iàèìòù 2€Iàèìòù🇱🇱 UeJean-pierreEuaoix = Jean-pierrepetEuaoixcieIàèìòùée🇱 ; Jean-pierrei grecMeu (comme une vache xptdrrr)bou974🇱Jean-pierre ! 123`"](demos/namefrench-special-symbols.jpg)
<sup><sub>While capital letters A-Z will be made lowercase, only letters a-z can be converted. Other symbols will be ignored</sub></sup>
