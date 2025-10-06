use brickworks_rs::{
    c_wrapper::dist::Dist as CDistBW, c_wrapper::src_int::SRCInt as CSRCIntBW,
    native::dist::Dist as RustDistBW, native::src_int::SRCInt as RustSRCIntBW,
};

const ERROR_CHANNELS: &str = "Channels size does not match.";

const MAX_BUFFER_SIZE: usize = 4092;
const OVERSAMPLE_FACTOR: i32 = 2;
// abstraction over rust port and binding of bw_dist
pub trait DistWrapper: Send {
    fn set_sample_rate(&mut self, sample_rate: f32);
    fn reset(&mut self);
    fn process(&mut self, x: &[&[f32]], y: &mut [&mut [f32]], n_samples: usize);
    fn set_distortion(&mut self, value: f32);
    fn set_tone(&mut self, value: f32);
    fn set_volume(&mut self, value: f32);
}

// macro to avoid repetition since both backends share the same api
// implements DistWrapper for the given type
macro_rules! impl_dist_wrapper {
    ($type:ty) => {
        impl<const N_CHANNELS: usize> DistWrapper for $type {
            fn set_sample_rate(&mut self, sample_rate: f32) {
                self.dist.set_sample_rate(sample_rate);
            }

            fn reset(&mut self) {
                self.dist.reset(Some(0.0), None);
                self.src_up.reset(Some(0.0), None);
                self.src_down.reset(Some(0.0), None);
            }

            fn process(&mut self, x: &[&[f32]], y: &mut [&mut [f32]], n_samples: usize) {
                let mut n_out: [usize; N_CHANNELS] = [0; N_CHANNELS];
                let upsampled_size = n_samples << 1;

                let buffer_up_ptrs: [*mut f32; N_CHANNELS] =
                    std::array::from_fn(|i| self.buffer_up[i].as_mut_ptr());

                let buffer_down_ptrs: [*mut f32; N_CHANNELS] =
                    std::array::from_fn(|i| self.buffer_down[i].as_mut_ptr());

                // need unsafe cause of from_raw_part_mut, basically I am creating
                // a slice from a raw pointer
                // todo: explain why it is safe
                unsafe {
                    let mut buffer_up_mut: [&mut [f32]; N_CHANNELS] = std::array::from_fn(|i| {
                        std::slice::from_raw_parts_mut(buffer_up_ptrs[i], upsampled_size)
                    });

                    self.src_up.process(
                        x.try_into().expect(ERROR_CHANNELS),
                        &mut buffer_up_mut,
                        n_samples,
                        Some(&mut n_out),
                    );
                }

                // need unsafe cause of from_raw_part_mut, basically I am creating
                // a slice from a raw pointer
                // todo: explain why it is safe if it is actually safe hehe
                unsafe {
                    let buffer_up_ref: [&[f32]; N_CHANNELS] = std::array::from_fn(|i| {
                        std::slice::from_raw_parts(buffer_up_ptrs[i], upsampled_size)
                    });

                    let mut buffer_down_mut: [&mut [f32]; N_CHANNELS] = std::array::from_fn(|i| {
                        std::slice::from_raw_parts_mut(buffer_down_ptrs[i], upsampled_size)
                    });

                    self.dist
                        .process(&buffer_up_ref, &mut buffer_down_mut, upsampled_size);
                }

                // need unsafe cause of from_raw_part_mut, basically I am creating
                // a slice from a raw pointer
                // todo: explain why it is safe if it is actually safe hehe
                unsafe {
                    let buffer_down_ref: [&[f32]; N_CHANNELS] = std::array::from_fn(|i| {
                        std::slice::from_raw_parts(buffer_down_ptrs[i], upsampled_size)
                    });

                    self.src_down.process(
                        &buffer_down_ref,
                        y.try_into().expect(ERROR_CHANNELS),
                        upsampled_size,
                        Some(&mut n_out),
                    );
                }
            }

            fn set_distortion(&mut self, value: f32) {
                self.dist.set_distortion(value);
            }

            fn set_tone(&mut self, value: f32) {
                self.dist.set_tone(value);
            }

            fn set_volume(&mut self, value: f32) {
                self.dist.set_volume(value);
            }
        }
    };
}

// expand for both versions
impl_dist_wrapper!(RustDist<N_CHANNELS>);
impl_dist_wrapper!(CDist<N_CHANNELS>);

pub struct RustDist<const N_CHANNELS: usize> {
    pub dist: RustDistBW<N_CHANNELS>,
    pub src_up: RustSRCIntBW<N_CHANNELS>,
    pub src_down: RustSRCIntBW<N_CHANNELS>,
    buffer_up: Vec<Vec<f32>>,
    buffer_down: Vec<Vec<f32>>,
}

impl<const N_CHANNELS: usize> RustDist<N_CHANNELS> {
    pub fn new() -> Self {
        Self {
            dist: RustDistBW::new(),
            src_up: RustSRCIntBW::new(OVERSAMPLE_FACTOR),
            src_down: RustSRCIntBW::new(-OVERSAMPLE_FACTOR),
            buffer_up: vec![vec![0.0; MAX_BUFFER_SIZE * OVERSAMPLE_FACTOR as usize]; N_CHANNELS],
            buffer_down: vec![vec![0.0; MAX_BUFFER_SIZE]; N_CHANNELS],
        }
    }
}

pub struct CDist<const N_CHANNELS: usize> {
    pub dist: CDistBW<N_CHANNELS>,
    pub src_up: CSRCIntBW<N_CHANNELS>,
    pub src_down: CSRCIntBW<N_CHANNELS>,
    buffer_up: Vec<Vec<f32>>,
    buffer_down: Vec<Vec<f32>>,
}

impl<const N_CHANNELS: usize> CDist<N_CHANNELS> {
    pub fn new() -> Self {
        Self {
            dist: CDistBW::new(),
            src_up: CSRCIntBW::new(OVERSAMPLE_FACTOR),
            src_down: CSRCIntBW::new(-OVERSAMPLE_FACTOR),
            buffer_up: vec![vec![0.0; MAX_BUFFER_SIZE as usize]; N_CHANNELS],
            buffer_down: vec![vec![0.0; MAX_BUFFER_SIZE as usize]; N_CHANNELS],
        }
    }
}
