# One-Pole Filter Plugin (C-to-Rust Binding Example)
This is a minimal working example of how to build an audio plugin using [nih-plug](https://github.com/robbert-vdh/nih-plug) to produce a VST3 and CLAP plugin. The plugin integrates a C-based one-pole filter, for which I wrapped the binding to the code from [Orastron](https://www.orastron.com/algorithms/bw_one_pole), using [`bindgen`](https://github.com/rust-lang/rust-bindgen) and exposed through [brickworks-rs](https://github.com/FedericoMenegoz/brickworks-rs).

>Full wiki can be found [here](https://github.com/FedericoMenegoz/brickworks-rs/wiki/Nih-Plug).

## Build and Test the VST3/CLAP in a DAW

1. Run the following command to build the plugin:
   ```bash
   cargo xtask bundle gain_plugin --release
   ```

2. Copy the generated VST3/CLAP to your system's plugin folder so your DAW can detect it.

   *VST3 For macOS:*
   ```bash
   sudo cp -r target/bundled/one_pole_plugin.vst3 /Library/Audio/Plug-Ins/VST3/
   ```


## License
Brickworks-rs-plugin is distributed under the [GPLv3](https://www.gnu.org/licenses/gpl-3.0.html) License.