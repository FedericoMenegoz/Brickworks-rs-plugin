use std::sync::Arc;

use nih_plug::prelude::*;
use nih_plug_egui::EguiState;
// parameters exposed to the host
#[derive(Params)]
pub(crate) struct DistParams {
    #[persist = "editor-state"]
    pub(crate) editor_state: Arc<EguiState>,
    #[id = "distortion"]
    pub distortion: FloatParam,
    #[id = "tone"]
    pub tone: FloatParam,
    #[id = "volume"]
    pub volume: FloatParam,
}

impl Default for DistParams {
    fn default() -> Self {
        Self {
            editor_state: EguiState::from_size(400, 300),
            distortion: FloatParam::new(
                "distortion",
                40.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: 100.0,
                },
            )
            .with_smoother(SmoothingStyle::None)
            .with_step_size(0.01)
            .with_unit("%"),
            tone: FloatParam::new(
                "tone",
                70.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: 100.0,
                },
            )
            .with_smoother(SmoothingStyle::None)
            .with_step_size(0.01)
            .with_unit("%"),
            volume: FloatParam::new(
                "volume",
                60.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: 100.0,
                },
            )
            .with_smoother(SmoothingStyle::None)
            .with_step_size(0.01)
            .with_unit("%"),
        }
    }
}
