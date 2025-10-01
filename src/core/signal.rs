use std::ops::{AddAssign, Deref, DerefMut, DivAssign, MulAssign, RemAssign, SubAssign};

use anyhow::Result;
use hound::SampleFormat;
use itertools::Itertools;
use num::{cast::AsPrimitive, Complex, FromPrimitive, Signed};
use rand::distr::uniform::{SampleBorrow, SampleUniform};

pub trait SignalType: num::Float + std::fmt::Debug +  FromPrimitive + Signed + Send + Sync + AsPrimitive<i32> + AsPrimitive<f64> + SampleBorrow<Self> + SampleUniform + Default + AddAssign + SubAssign + DivAssign + MulAssign + RemAssign + 'static {}
impl SignalType for f32 {}
impl SignalType for f64 {}

pub trait FromVec<T, U> {
    fn from_vec(sample_rate: f64, samples: Vec<T>) -> Signal<U>;
}

impl<T: SignalType> FromVec<Complex<T>, T> for Signal<T> {
    fn from_vec(sample_rate: f64, samples: Vec<Complex<T>>) -> Signal<T> {
        Signal {
            time: 0,
            sample_rate,
            samples,
        }
    }
}
impl<T: SignalType> FromVec<T, T> for Signal<T> {
    fn from_vec(sample_rate: f64, samples: Vec<T>) -> Signal<T> {
        Signal {
            time: 0,
            sample_rate,
            samples: samples
                .iter()
                .map(|x| Complex::new(*x, T::zero()))
                .collect_vec(),
        }
    }
}

pub trait FromFunction<T, U> {
    fn from_function(sample_rate: f64, len: usize, func: impl Fn(f64)->T) -> Signal<U>;
}
impl<T: SignalType> FromFunction<Complex<T>, T> for Signal<T> {
    fn from_function(sample_rate: f64, len: usize, func: impl Fn(f64)->Complex<T>) -> Signal<T> {
        Signal {
            time: 0,
            sample_rate,
            samples: (0..len).into_iter().map(|x| func((x as f64)/sample_rate)).collect_vec(),
        }
    }
}
impl<T: SignalType> FromFunction<T, T> for Signal<T> {
    fn from_function(sample_rate: f64, len: usize, func: impl Fn(f64)->T) -> Signal<T> {
        Self::from_function(sample_rate, len, |x| Complex { re: func(x), im: T::zero() })
    }
}


#[derive(Clone)]
pub struct Signal<T> {
    pub sample_rate: f64,
    pub time: i64,
    samples: Vec<Complex<T>>,
}

impl<T: SignalType> Signal<T> {
    pub fn new(sample_rate: f64) -> Signal<T> {
        Signal {
            time: 0,
            sample_rate,
            samples: Vec::new(),
        }
    }
    pub fn write_wav(
        &self,
        path: &str,
        sample_format: SampleFormat,
        normalize: bool,
    ) -> Result<()> {
        // write_wav_real(path, self, sample_format, normalize)
        todo!()
    }
    pub fn read_wav(path: &str) -> Result<Signal<T>> {
        // read_wav_real(path)
        todo!()
    }
}
impl<T> Deref for Signal<T> {
    type Target = Vec<Complex<T>>;

    fn deref(&self) -> &Self::Target {
        &self.samples
    }
}
impl<T> DerefMut for Signal<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.samples
    }
}
