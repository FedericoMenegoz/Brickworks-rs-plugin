pub mod builder;
pub mod params;
pub mod plugin;
pub mod wrapper;


use builder::{CBuilder, RustBuilder};
use nih_plug::{nih_export_clap, nih_export_vst3};
use plugin::DistPlugin;

nih_export_vst3!(DistPlugin<RustBuilder>, DistPlugin<CBuilder>);
nih_export_clap!(DistPlugin<RustBuilder>, DistPlugin<CBuilder>);
