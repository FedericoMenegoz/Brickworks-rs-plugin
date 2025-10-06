# Distortion Plugin (Brickworks-rs)
This repository provides a minimal working example of how to use the [brickworks-rs](https://github.com/FedericoMenegoz/brickworks-rs) library inside an audio plugin built with [nih-plug](https://github.com/robbert-vdh/nih-plug), targeting VST3 and CLAP formats.

The brickworks-rs library offers both a native Rust port and a direct C binding of the original Brickworks dist implementation by [Orastron](https://www.orastron.com/algorithms/bw_dist).

>Full wiki can be found [here](https://github.com/FedericoMenegoz/brickworks-rs/wiki/Nih-Plug).
## Filter Source Options


## Build and Test the VST3/CLAP in a DAW

1. Run the following command to build the plugin, it will create both the native and the binding version:
   ```bash
   cargo xtask bundle dist_plugin --release
   ```

2. Copy the generated VST3/CLAP to your system's plugin folder so your DAW can detect it.

   *VST3 For macOS:*
   ```bash
   sudo cp -r target/bundled/dist_plugin.vst3 /Library/Audio/Plug-Ins/VST3/
   ```

## Debug
To run the plugin in debug mode, useful for checking issues like memory allocations during processing, use:

```bash
cargo build                # build the standalone
./target/debug/dist_plugin # run the standalone
# On macOS, if you get "Received 558 samples, while the configured buffer size is 512":
./target/debug/dist_plugin -r 44100



## License
Brickworks-rs-plugin is distributed under the [GPLv3](https://www.gnu.org/licenses/gpl-3.0.html) License.