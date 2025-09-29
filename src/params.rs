use nih_plug::prelude::*;

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
                0.4,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_smoother(SmoothingStyle::None)
            .with_step_size(0.01),
            tone: FloatParam::new("tone", 0.7, FloatRange::Linear { min: 0.0, max: 1.0 })
                .with_smoother(SmoothingStyle::None)
                .with_step_size(0.01),
            volume: FloatParam::new("volume", 0.6, FloatRange::Linear { min: 0.0, max: 1.0 })
                .with_smoother(SmoothingStyle::None)
                .with_step_size(0.01),
        }
    }
}
