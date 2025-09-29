pub mod factory;
pub mod params;
pub mod plugin;
pub mod wrapper;

use factory::{CFactory, RustFactory};
use nih_plug::{nih_export_clap, nih_export_vst3};
use plugin::DistPlugin;

nih_export_vst3!(DistPlugin<RustFactory>, DistPlugin<CFactory>);
nih_export_clap!(DistPlugin<RustFactory>, DistPlugin<CFactory>);
