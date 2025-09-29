use brickworks_rs::{c_wrapper::dist::Dist as CDist, native::dist::Dist as RustDist};

const ERROR_CHANNELS: &str = "Channels size does not match.";

pub trait DistWrapper: Send {
    fn set_sample_rate(&mut self, sample_rate: f32);
    fn reset(&mut self, x0: Option<f32>, y0: Option<&mut [f32]>);
    fn process(&mut self, x: &[&[f32]], y: &mut [&mut [f32]], n_samples: usize);
    fn set_distortion(&mut self, value: f32);
    fn set_tone(&mut self, value: f32);
    fn set_volume(&mut self, value: f32);
}

impl<const N_CHANNELS: usize> DistWrapper for RustDist<N_CHANNELS> {
    fn set_sample_rate(&mut self, sample_rate: f32) {
        self.set_sample_rate(sample_rate);
    }

    fn reset(&mut self, x0: Option<f32>, y0: Option<&mut [f32]>) {
        self.reset(x0, y0.map(|slice| slice.try_into().expect(ERROR_CHANNELS)));
    }

    fn process(&mut self, x: &[&[f32]], y: &mut [&mut [f32]], n_samples: usize) {
        self.process(
            x.try_into().expect(ERROR_CHANNELS),
            y.try_into().expect(ERROR_CHANNELS),
            n_samples,
        );
    }

    fn set_distortion(&mut self, value: f32) {
        self.set_distortion(value);
    }

    fn set_tone(&mut self, value: f32) {
        self.set_tone(value);
    }

    fn set_volume(&mut self, value: f32) {
        self.set_volume(value);
    }
}

impl<const N_CHANNELS: usize> DistWrapper for CDist<N_CHANNELS> {
    fn set_sample_rate(&mut self, sample_rate: f32) {
        self.set_sample_rate(sample_rate);
    }

    fn reset(&mut self, x0: Option<f32>, y0: Option<&mut [f32]>) {
        self.reset(x0, y0.map(|slice| slice.try_into().expect(ERROR_CHANNELS)));
    }

    fn process(&mut self, x: &[&[f32]], y: &mut [&mut [f32]], n_samples: usize) {
        self.process(
            x.try_into().expect(ERROR_CHANNELS),
            y.try_into().expect(ERROR_CHANNELS),
            n_samples,
        );
    }

    fn set_distortion(&mut self, value: f32) {
        self.set_distortion(value);
    }

    fn set_tone(&mut self, value: f32) {
        self.set_tone(value);
    }

    fn set_volume(&mut self, value: f32) {
        self.set_volume(value);
    }
}
