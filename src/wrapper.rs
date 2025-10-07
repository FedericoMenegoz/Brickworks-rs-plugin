use brickworks_rs::{
    c_wrapper::dist::Dist as CDistBW, c_wrapper::src_int::SRCInt as CSRCIntBW,
    native::dist::Dist as RustDistBW, native::src_int::SRCInt as RustSRCIntBW,
};

const BUFFER_SIZE: usize = 32;
const OVERSAMPLE_FACTOR: i32 = 2;
// abstraction over rust port and binding of bw_dist
pub trait DistWrapper: Send {
    fn set_sample_rate(&mut self, sample_rate: f32);
    fn reset(&mut self);
    // fn process(&mut self, x: &[&[f32]], y: &mut [&mut [f32]], n_samples: usize);
    fn process(&mut self, x: &[f32], y: &mut [f32], n_samples: usize, channel: usize);
    fn set_distortion(&mut self, value: f32);
    fn set_tone(&mut self, value: f32);
    fn set_volume(&mut self, value: f32);
}

// macro to avoid repetition since both backends share the same api
// implements DistWrapper for the given type
macro_rules! impl_dist_wrapper {
    ($type:ty) => {
        impl<const N_CHANNELS: usize> DistWrapper for $type {
            
            #[inline(always)]
            fn set_sample_rate(&mut self, sample_rate: f32) {
                self.dist.set_sample_rate(sample_rate);
            }

            #[inline(always)]
            fn reset(&mut self) {
                self.dist.reset(Some(0.0), None);
                self.src_up.reset(Some(0.0), None);
                self.src_down.reset(Some(0.0), None);
            }

            // fn process(&mut self, x: &[&[f32]], y: &mut [&mut [f32]], n_samples: usize) {
            #[inline(always)]
            fn process(&mut self, x: &[f32], y: &mut [f32], n_samples: usize, channel: usize) {
                // (0..N_CHANNELS).for_each(|channel| {
                    let mut i = 0;
                    while i < n_samples {
                        let n =
                            (n_samples as i32 - i as i32).min((BUFFER_SIZE >> 1) as i32) as usize;
                        // upsampling
                        self.src_up.coeffs.process(
                            &mut self.src_up.states[channel],
                            &x[i..],
                            &mut self.buffer_a,
                            n,
                        );
                        // brickworks process the samples one at the time 
                        // it reads it then make the calculation 
                        // and at the end it save it, using only one buffer
                        // should be fine
                        unsafe {
                            let read_ptr = self.buffer_a.as_ptr();
                            let write_ptr = self.buffer_a.as_mut_ptr();
                            // processing
                            self.dist.coeffs.process(
                                &mut self.dist.states[channel],
                                std::slice::from_raw_parts(read_ptr, self.buffer_a.len()),
                                std::slice::from_raw_parts_mut(write_ptr, self.buffer_a.len()),
                                n << 1,
                            );
                        }
                        // downsampling
                        self.src_down.coeffs.process(
                            &mut self.src_down.states[channel],
                            &self.buffer_a,
                            &mut y[i..],
                            n << 1,
                        );
                        i += n;
                    }
                // });
            }

            #[inline(always)]
            fn set_distortion(&mut self, value: f32) {
                self.dist.set_distortion(value);
            }

            #[inline(always)]
            fn set_tone(&mut self, value: f32) {
                self.dist.set_tone(value);
            }

            #[inline(always)]
            fn set_volume(&mut self, value: f32) {
                self.dist.set_volume(value);
            }
        }
    };
}

macro_rules! define_dist_struct {
    ($name:ident, $dist_type:ty, $src_type:ty) => {
        pub struct $name<const N_CHANNELS: usize> {
            pub dist: $dist_type,
            pub src_up: $src_type,
            pub src_down: $src_type,
            pub buffer_a: [f32; BUFFER_SIZE],
            // pub buffer_b: [f32; BUFFER_SIZE],
        }

        impl<const N_CHANNELS: usize> $name<N_CHANNELS> {
            #[inline(always)]
            pub fn new() -> Self {
                Self {
                    dist: <$dist_type>::new(),
                    src_up: <$src_type>::new(OVERSAMPLE_FACTOR),
                    src_down: <$src_type>::new(-OVERSAMPLE_FACTOR),
                    buffer_a: [0.0; BUFFER_SIZE],
                    // buffer_b: [0.0; BUFFER_SIZE],
                }
            }
        }

        impl_dist_wrapper!($name<N_CHANNELS>);
    };
}

define_dist_struct!(RustDist, RustDistBW<N_CHANNELS>, RustSRCIntBW<N_CHANNELS>);
define_dist_struct!(CDist, CDistBW<N_CHANNELS>, CSRCIntBW<N_CHANNELS>);
