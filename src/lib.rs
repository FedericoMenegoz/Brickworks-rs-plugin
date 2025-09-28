use std::{num::NonZeroU32, sync::Arc};

use brickworks_rs::native::dist::Dist;
use nih_plug::prelude::*;

const ERROR_CHANNELS: &str = "Channels size does not match.";
const ERROR_DIST_INIT: &str = "Dist must be initializated when calling process.";
const MAX_CHANNELS: usize = 2;
const MAX_SAMPLES: usize = 2048;

pub struct DistPlugin {
    params: Arc<DistParams>,
    dist: Option<Box<dyn DistWrapper>>,
    input: Vec<Vec<f32>>,
}

impl Default for DistPlugin {
    fn default() -> Self {
        let mut input = Vec::with_capacity(MAX_CHANNELS);
        (0..MAX_CHANNELS).for_each(|_| input.push(Vec::with_capacity(MAX_SAMPLES)));

        Self {
            params: Arc::new(DistParams::default()),
            dist: None,
            input,
        }
    }
}

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

trait DistWrapper: Send {
    fn set_sample_rate(&mut self, sample_rate: f32);
    fn reset(&mut self, x0: Option<f32>, y0: Option<&mut [f32]>);
    fn process(&mut self, x: &[&[f32]], y: &mut [&mut [f32]], n_samples: usize);
    fn set_distortion(&mut self, value: f32);
    fn set_tone(&mut self, value: f32);
    fn set_volume(&mut self, value: f32);
}

impl<const N_CHANNELS: usize> DistWrapper for Dist<N_CHANNELS> {
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

impl Plugin for DistPlugin {
    const NAME: &'static str = "Rodent Distortion Pedal";
    const VENDOR: &'static str = "CIMIL Thesis";
    const URL: &'static str = "https://github.com/FedericoMenegoz/Brickworks-rs-plugin";
    const EMAIL: &'static str = "fede.mene@icloud.com";
    const VERSION: &'static str = "0.1";
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),
            aux_input_ports: &[],
            aux_output_ports: &[],
            names: PortNames::const_default(),
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(1),
            main_output_channels: NonZeroU32::new(1),
            ..AudioIOLayout::const_default()
        },
    ];
    type SysExMessage = ();

    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn initialize(
        &mut self,
        audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        let n_channels = audio_io_layout
            .main_input_channels
            .map(|c| c.get())
            .expect("Must have some channels!") as usize;

        self.dist = match n_channels {
            1 => Some(Box::new(Dist::<1>::new()) as Box<dyn DistWrapper>),
            2 => Some(Box::new(Dist::<2>::new()) as Box<dyn DistWrapper>),
            _ => panic!("Unsupported channel count"),
        };

        if let Some(dist) = &mut self.dist {
            dist.set_sample_rate(buffer_config.sample_rate);
        };
        for channel in 0..n_channels {
            if channel >= self.input.len() {
                self.input
                    .push(vec![0.0; buffer_config.max_buffer_size as usize]);
            } else {
                self.input[channel].resize(buffer_config.max_buffer_size as usize, 0.0);
            }
        }
        true
    }

    fn reset(&mut self) {
        if let Some(dist) = &mut self.dist {
            dist.as_mut().reset(None, None);
        }
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let num_samples = buffer.samples();
        let num_channels = buffer.channels();

        let dist = self.dist.as_mut().expect(ERROR_DIST_INIT);
        dist.set_distortion(self.params.distortion.value());
        dist.set_tone(self.params.tone.value());
        dist.set_volume(self.params.volume.value());

        for (sample_index, samples_channel) in buffer.iter_samples().enumerate() {
            for (channel_index, sample) in samples_channel.into_iter().enumerate() {
                self.input[channel_index][sample_index] = *sample;
            }
        }

        let mut input_refs: [&[f32]; MAX_CHANNELS] = [&[]; MAX_CHANNELS];
        for (ch, item) in input_refs.iter_mut().enumerate().take(num_channels) {
            *item = &self.input[ch][..num_samples];
        }

        dist.process(&input_refs[..num_channels], buffer.as_slice(), num_samples);

        ProcessStatus::Normal
    }
}

impl Vst3Plugin for DistPlugin {
    const VST3_CLASS_ID: [u8; 16] = *b"DistortionPlugin";

    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[Vst3SubCategory::Fx];
}

impl ClapPlugin for DistPlugin {
    const CLAP_ID: &'static str = "com.cimil-thesis.dist";
    const CLAP_DESCRIPTION: Option<&'static str> =
        Some("Distortion effect. Loosely inspired to the 'rodent' distortion pedal.");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::AudioEffect,
        ClapFeature::Stereo,
        ClapFeature::Mono,
        ClapFeature::Utility,
    ];
}

nih_export_vst3!(DistPlugin);
nih_export_clap!(DistPlugin);
