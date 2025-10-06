use super::wrapper::{CDist, DistWrapper, RustDist};

// avoid repetition since both versions share the same api
// enables calling nih macros by just specifying the type
pub trait DistBuilder {
    const NAME: &'static str;
    const VST3_CLASS_ID: [u8; 16];
    fn make(n_channels: usize) -> Box<dyn DistWrapper>;
}

pub struct RustBuilder;
impl DistBuilder for RustBuilder {
    const NAME: &'static str = "Rust Distortion";

    const VST3_CLASS_ID: [u8; 16] = *b"*RustDistortion*";

    fn make(n_channels: usize) -> Box<dyn DistWrapper> {
        match n_channels {
            1 => Box::new(RustDist::<1>::new()),
            2 => Box::new(RustDist::<2>::new()),
            _ => panic!("unsupported channels"),
        }
    }
}

pub struct CBuilder;
impl DistBuilder for CBuilder {
    const NAME: &'static str = "C Distortion";
    const VST3_CLASS_ID: [u8; 16] = *b"**C_Distortion**";

    fn make(n_channels: usize) -> Box<dyn DistWrapper> {
        match n_channels {
            1 => Box::new(CDist::<1>::new()),
            2 => Box::new(CDist::<2>::new()),
            _ => panic!("unsupported channels"),
        }
    }
}
