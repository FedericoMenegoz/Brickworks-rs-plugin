use nih_plug::prelude::*;

// parameters exposed to the host
#[derive(Params)]
pub struct DistParams {
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
