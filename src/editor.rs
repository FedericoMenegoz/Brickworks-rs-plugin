use nih_plug::editor::Editor;
use nih_plug_egui::{create_egui_editor, egui::Vec2, resizable_window::ResizableWindow, widgets};
use std::sync::Arc;

use crate::params::DistParams;

pub(crate) fn create(params: Arc<DistParams>) -> Option<Box<dyn Editor>> {
    let egui_state = params.editor_state.clone();

    create_egui_editor(
        egui_state.clone(),
        (),
        |_, _| {},
        move |egui_ctx, setter, _state| {
            ResizableWindow::new("res-wind")
                .min_size(Vec2::new(128.0, 128.0))
                .show(egui_ctx, egui_state.as_ref(), |ui| {
                    ui.label("Distortion");
                    ui.add(widgets::ParamSlider::for_param(&params.distortion, setter));

                    ui.label("Tone");
                    ui.add(widgets::ParamSlider::for_param(&params.tone, setter));

                    ui.label("Volume");
                    ui.add(widgets::ParamSlider::for_param(&params.volume, setter));
                });
        },
    )
}
