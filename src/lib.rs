use std::{num::NonZeroU32, sync::Arc};

use brickworks_rs::c_wrapper::one_pole_wrapper::OnePoleWrapper;
use nih_plug::prelude::*;

const MAX_CHANNELS: usize = 2;
const MAX_SAMPLES: usize = 2048;

pub struct OnePoleFilterPlugin {
    params: Arc<OnePoleFilterParams>,
    filter: Option<Box<dyn OnePoleWrapperTrait>>,
    input_buffer: Vec<Vec<f32>>,
}

trait OnePoleWrapperTrait: Send {
    fn set_sample_rate(&mut self, rate: f32);
    fn set_cutoff(&mut self, cutoff: f32);
    fn process(&mut self, input: &[Vec<f32>], output: Option<&mut [&mut [f32]]>, n_samples: usize);
    fn reset(&mut self);
}

impl<const N: usize> OnePoleWrapperTrait for OnePoleWrapper<N> {
    fn set_sample_rate(&mut self, rate: f32) {
        self.set_sample_rate(rate);
    }

    fn set_cutoff(&mut self, cutoff: f32) {
        self.set_cutoff(cutoff);
    }

    fn process(&mut self, input: &[Vec<f32>], output: Option<&mut [&mut [f32]]>, n_samples: usize) {
        self.process(input, output, n_samples);
    }

    fn reset(&mut self) {
        self.reset(None, None);
    }
}

#[derive(Params)]
pub struct OnePoleFilterParams {
    #[id = "cutoff"]
    pub cutoff: FloatParam,
}

impl Default for OnePoleFilterPlugin {
    fn default() -> Self {
        let mut input_buffer = Vec::with_capacity(MAX_CHANNELS);
        for _ in 0..MAX_CHANNELS {
            let channel: Vec<f32> = Vec::with_capacity(MAX_SAMPLES);
            input_buffer.push(channel);
        }
        Self {
            params: Arc::new(OnePoleFilterParams::default()),
            filter: None,
            input_buffer,
        }
    }
}

impl Default for OnePoleFilterParams {
    fn default() -> Self {
        Self {
            cutoff: FloatParam::new(
                "cutoff",
                100.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: 1000.0,
                },
            )
            .with_step_size(0.1)
            .with_smoother(SmoothingStyle::Logarithmic(1.0))
            .with_unit(" Hz"),
        }
    }
}

impl Plugin for OnePoleFilterPlugin {
    const NAME: &'static str = "One Pole Low Pass Filter Plugin";
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
    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;
    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn reset(&mut self) {
        if let Some(filter) = &mut self.filter {
            filter.as_mut().reset();
        }
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

        self.filter = match n_channels {
            1 => Some(Box::new(OnePoleWrapper::<1>::new())),
            2 => Some(Box::new(OnePoleWrapper::<2>::new())),
            _ => panic!("Unsupported channel count"),
        };

        if let Some(filter) = &mut self.filter {
            filter.set_sample_rate(buffer_config.sample_rate);
        }

        for i in 0..n_channels {
            if i >= self.input_buffer.len() {
                self.input_buffer
                    .push(vec![0.0; buffer_config.max_buffer_size as usize]);
            } else {
                self.input_buffer[i].resize(buffer_config.max_buffer_size as usize, 0.0);
            }
        }
        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let n_samples = buffer.samples();
        self.filter
            .as_mut()
            .expect("One Pole Wrapper must be initialized by now.")
            .set_cutoff(self.params.cutoff.value());

        for (sample_index, samples_channel) in buffer.iter_samples().enumerate() {
            for (channel_index, sample) in samples_channel.into_iter().enumerate() {
                self.input_buffer[channel_index][sample_index] = *sample
            }
        }

        self.filter
            .as_mut()
            .expect("One Pole Wrapper must be initialized by now.")
            .process(&self.input_buffer, Some(buffer.as_slice()), n_samples);

        ProcessStatus::Normal
    }
}

impl Vst3Plugin for OnePoleFilterPlugin {
    const VST3_CLASS_ID: [u8; 16] = *b"MyFirstPlugin666";

    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[Vst3SubCategory::Fx];
}
impl ClapPlugin for OnePoleFilterPlugin {
    const CLAP_ID: &'static str = "com.cimil-thesis.gain";
    const CLAP_DESCRIPTION: Option<&'static str> =
        Some("A one pole low pass filter example plugin");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::AudioEffect,
        ClapFeature::Stereo,
        ClapFeature::Mono,
        ClapFeature::Utility,
    ];
}
nih_export_vst3!(OnePoleFilterPlugin);
nih_export_clap!(OnePoleFilterPlugin);
