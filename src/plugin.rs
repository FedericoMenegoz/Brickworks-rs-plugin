use crate::{backend::{DistFactory, DistBackend}, params::DistParams};
use nih_plug::prelude::*;
use std::{num::NonZeroU32, sync::Arc};

const ERROR_DIST_INIT: &str = "Dist must be initialized when calling process.";
const N_CHANNELS: u32 = 1; // Mono

pub struct DistPlugin<D> {
    // parameters given to the host
    params: Arc<DistParams>,
    // actual dist
    dist: Option<Box<dyn DistBackend>>,
    // zero-size placeholder that links either to port or native dist
    _marker: std::marker::PhantomData<D>,
}

// requested Default in order to implement the Plugin trait
impl<D> Default for DistPlugin<D> {
    fn default() -> Self {
        Self {
            params: Arc::new(DistParams::default()),
            dist: None,
            _marker: std::marker::PhantomData,
        }
    }
}

// plugin implementation for both rust and c version
impl<D> Plugin for DistPlugin<D>
where
    D: DistFactory + 'static + Send,
{
    const NAME: &'static str = D::NAME;
    const VENDOR: &'static str = "CIMIL Thesis";
    const URL: &'static str = "https://github.com/FedericoMenegoz/Brickworks-rs-plugin";
    const EMAIL: &'static str = "fede.mene@icloud.com";
    const VERSION: &'static str = "0.1";
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(N_CHANNELS),
        main_output_channels: NonZeroU32::new(N_CHANNELS),
        ..AudioIOLayout::const_default()
    }];
    type SysExMessage = ();

    type BackgroundTask = ();

    #[inline(always)]
    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    #[inline(always)]
    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        self.dist = Some(D::make(N_CHANNELS));

        if let Some(dist) = &mut self.dist {
            dist.set_sample_rate(buffer_config.sample_rate);
        };
        true
    }

    #[inline(always)]
    fn reset(&mut self) {
        if let Some(dist) = &mut self.dist {
            dist.as_mut().reset();
        }
    }

    #[inline(always)]
    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        // Warning: AudioIOLayout is MONO
        // To process stereo need to loop through the channels
        const CHANNEL: usize = 0;

        let num_samples = buffer.samples();

        let dist = self.dist.as_mut().expect(ERROR_DIST_INIT);
        dist.set_distortion(self.params.distortion.value()*0.01);
        dist.set_tone(self.params.tone.value()*0.01);
        dist.set_volume(self.params.volume.value()*0.01);

        let read_ptr = buffer.as_slice_immutable()[CHANNEL].as_ptr();
        let write_ptr = buffer.as_slice()[CHANNEL].as_mut_ptr();
        // brickworks process the samples one at the time
        // it reads it then make the calculation
        // and at the end it save it, using only one buffer
        unsafe {
            dist.process(
                std::slice::from_raw_parts(read_ptr, num_samples),
                std::slice::from_raw_parts_mut(write_ptr, num_samples),
                num_samples,
                CHANNEL,
            );
        }
        ProcessStatus::Normal
    }
}

impl<D> Vst3Plugin for DistPlugin<D>
where
    D: DistFactory + 'static + Send,
{
    const VST3_CLASS_ID: [u8; 16] = D::VST3_CLASS_ID;
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[Vst3SubCategory::Fx];
}

impl<D> ClapPlugin for DistPlugin<D>
where
    D: DistFactory + 'static + Send,
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
