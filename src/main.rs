use nih_plug::nih_export_standalone;
use one_pole_plugin::OnePoleFilterPlugin;

fn main() {
    nih_export_standalone::<OnePoleFilterPlugin>();
}
