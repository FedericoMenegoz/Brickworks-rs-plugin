# One-Pole Filter Plugin (Brickworks-rs)
This repository provides a minimal working example of how to use the [brickworks-rs](https://github.com/FedericoMenegoz/brickworks-rs) library inside an audio plugin built with [nih-plug](https://github.com/robbert-vdh/nih-plug), targeting VST3 and CLAP formats.

The brickworks-rs library offers both a native Rust port and a direct C binding of the original Brickworks one-pole filter implementation by [Orastron](https://www.orastron.com/algorithms/bw_one_pole).

>Full wiki can be found [here](https://github.com/FedericoMenegoz/brickworks-rs/wiki/Nih-Plug).
## Filter Source Options

```rust
// To use the C wrapper version:
use brickworks_rs::c_wrapper::one_pole::OnePole;
// To use the native Rust version:
use brickworks_rs::native::one_pole::OnePole;
```

## Build and Test the VST3/CLAP in a DAW

1. Run the following command to build the plugin:
   ```bash
   cargo xtask bundle one_pole_plugin --release
   ```

2. Copy the generated VST3/CLAP to your system's plugin folder so your DAW can detect it.

   *VST3 For macOS:*
   ```bash
   sudo cp -r target/bundled/one_pole_plugin.vst3 /Library/Audio/Plug-Ins/VST3/
   ```


## License
Brickworks-rs-plugin is distributed under the [GPLv3](https://www.gnu.org/licenses/gpl-3.0.html) License.