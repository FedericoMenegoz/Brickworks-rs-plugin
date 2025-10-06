use crate::{builder::DistBuilder, params::DistParams, wrapper::DistWrapper};
use nih_plug::prelude::*;
use std::{num::NonZeroU32, sync::Arc};

const ERROR_DIST_INIT: &str = "Dist must be initialized when calling process.";
const MAX_CHANNELS: usize = 2;
const MAX_SAMPLES: usize = 2048;

pub struct DistPlugin<D> {
    // parameters given to the host
    params: Arc<DistParams>,
    // actual dist
    dist: Option<Box<dyn DistWrapper>>,
    input: Vec<Vec<f32>>,
    // zero-size placeholder that links either to port or native dist
    _marker: std::marker::PhantomData<D>,
}

// requested Default in order to implement the Plugin trait
impl<D> Default for DistPlugin<D> {
    fn default() -> Self {
        let mut input = Vec::with_capacity(MAX_CHANNELS);
        (0..MAX_CHANNELS).for_each(|_| input.push(Vec::with_capacity(MAX_SAMPLES)));

        Self {
            params: Arc::new(DistParams::default()),
            dist: None,
            input,
            _marker: std::marker::PhantomData,
        }
    }
}

// plugin implementation for both rust and c version
impl<D> Plugin for DistPlugin<D>
where
    D: DistBuilder + 'static + Send,
{
    const NAME: &'static str = D::NAME;
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

        self.dist = Some(D::make(n_channels));

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
            dist.as_mut().reset();
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

        dist.process(
            &mut input_refs[..num_channels],
            buffer.as_slice(),
            num_samples,
        );

        ProcessStatus::Normal
    }
}

impl<D> Vst3Plugin for DistPlugin<D>
where
    D: DistBuilder + 'static + Send,
{
    const VST3_CLASS_ID: [u8; 16] = D::VST3_CLASS_ID;
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[Vst3SubCategory::Fx];
}

impl<D> ClapPlugin for DistPlugin<D>
where
    D: DistBuilder + 'static + Send,
{
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
