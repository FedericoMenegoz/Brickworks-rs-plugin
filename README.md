# One-Pole Filter Plugin (C-to-Rust Binding Example)
This is a minimal working example of how to build an audio plugin using [nih-plug](https://github.com/robbert-vdh/nih-plug) to produce a VST3 and CLAP plugin. The plugin integrates a C-based one-pole filter, for which I wrapped the binding to the code from [Orastron](https://www.orastron.com/algorithms/bw_one_pole), using [`bindgen`](https://github.com/rust-lang/rust-bindgen) and exposed through [brickworks-rs](https://github.com/FedericoMenegoz/brickworks-rs).


## Build and Test the VST3 in a DAW

1. Run the following command to build the plugin:
   ```bash
   cargo xtask bundle gain_plugin --release
   ```

2. Copy the generated VST3/CLAP to your system's plugin folder so your DAW can detect it.

   **For macOS:**
   ```bash
   sudo cp -r target/bundled/one_pole_plugin.vst3 /Library/Audio/Plug-Ins/VST3/
   ```

## Standalone Debugging

Running the plugin standalone is helpful for debugging.

1. Refer to the [NIH Plug documentation](https://nih-plug.robbertvanderhelm.nl/nih_plug/wrapper/standalone/fn.nih_export_standalone.html) to enable standalone mode (already done in this repo).

2. Build the standalone:
   ```bash
   cargo build
   ```

3. Run the standalone:
   ```bash
   ./target/debug/one_pole_filter
   ```

   **Note for macOS:** You might see an error like:
   ```
   Received 558 samples, while the configured buffer size is 512
   ```
   This is usually due to a sample rate mismatch as stated in this [issue](https://github.com/robbert-vdh/nih-plug/issues/147). You can fix it by running:
   ```bash
   ./target/debug/one_pole_filter -r 44100
   ```

4. For additional options, run:
   ```bash
   ./target/debug/one_pole_filter --help
   ```

## License
Brickworks-rs-plugin is distributed under the [GPLv3](https://www.gnu.org/licenses/gpl-3.0.html) License.