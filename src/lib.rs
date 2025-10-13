pub mod params;
pub mod plugin;
pub mod backend;

use backend::{CDistFactory, RustDistFactory};
use nih_plug::{nih_export_clap, nih_export_vst3};
use plugin::DistPlugin;

nih_export_vst3!(DistPlugin<RustDistFactory>, DistPlugin<CDistFactory>);
nih_export_clap!(DistPlugin<RustDistFactory>, DistPlugin<CDistFactory>);
