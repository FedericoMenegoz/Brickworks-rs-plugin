use super::wrapper::DistWrapper;
use brickworks_rs::{c_wrapper::dist::Dist as CDist, native::dist::Dist as RustDist};

pub trait DistFactory {
    const NAME: &'static str;
    fn make(n_channels: usize) -> Box<dyn DistWrapper>;
}

pub struct RustFactory;
impl DistFactory for RustFactory {
    const NAME: &'static str = "Rust Distortion";
    fn make(n_channels: usize) -> Box<dyn DistWrapper> {
        match n_channels {
            1 => Box::new(RustDist::<1>::new()),
            2 => Box::new(RustDist::<2>::new()),
            _ => panic!("unsupported channels"),
        }
    }
}

pub struct CFactory;
impl DistFactory for CFactory {
    const NAME: &'static str = "C Distortion";
    fn make(n_channels: usize) -> Box<dyn DistWrapper> {
        match n_channels {
            1 => Box::new(CDist::<1>::new()),
            2 => Box::new(CDist::<2>::new()),
            _ => panic!("unsupported channels"),
        }
    }
}
