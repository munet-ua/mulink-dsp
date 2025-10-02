use anyhow::Context;
use num::{cast::AsPrimitive, Complex};

use crate::{plot::{spectrum::spectrogram, time::{plot, plot_complex}}, prelude::{FromVec, Signal, SignalType}};

pub fn euler<T: SignalType>(x: impl Into<T>) -> Complex<T> {
    let x = x.into();
    Complex {
        re: T::cos(x),
        im: T::sin(x),
    }
}
/// f_start f_end normalized to nyquist
pub fn chirp_complex<T: SignalType>(len: usize, f_start: impl Into<T>, f_end: impl Into<T>) -> Vec<Complex<T>> {
        let (f_start, f_end) = (f_start.into()/T::from(2.0).unwrap(), f_end.into()/T::from(2.0).unwrap());
        let bw = f_start - f_end;
        let len_t = T::from_usize(len).unwrap();
        let pi = T::from_f64(core::f64::consts::PI).unwrap();

        let offset_samples: i32 = ((f_end.abs() - f_start.abs()) * len_t).as_();

        (0..len)
            .map(|idx| {
                euler::<T>(
                    (pi * bw
                        * T::powi(T::from(idx as i32 - (len as i32 / 2) + offset_samples).unwrap(), 2))
                        / (len_t),
                )
            })
            .collect()
    }

    pub fn chirp_real<T: SignalType>(len: usize, f_start: f64, f_end: f64) -> Vec<T> {
        //let mut signal = vec![0.0_f64; len];
        //let f_step: f64 = (f_start - f_end) / len as f64;
        let (f_start, f_end): (T,T) = (T::from_f64(f_start/2.0).unwrap(),T::from_f64(f_end/2.0).unwrap());
        let c: T = (f_end - f_start) / (T::from_usize(len).unwrap()) / T::from_f64(2.0).unwrap();
        
        
        let c: T = T::from(c).unwrap();
        let pi = T::from_f64(core::f64::consts::PI*2.0).unwrap();
        let two = T::from_f64(2.0).unwrap();//.context("Failed conversion")?;

        (0..len)
            .map(|idx| {
                let idx:T = T::from_usize(idx).unwrap();
                T::sin(
                    ( pi * (c * T::powi(idx , 2) + f_start * (idx))),
                )
            })
            .collect()
    }

impl<T: SignalType> Signal<T> {
    pub fn chirp_complex(sample_rate: f64, len: usize, f_start: f64, f_end: f64) -> Signal<T> {
        Signal::from_vec(sample_rate, chirp_complex(len, T::from_f64(f_start/sample_rate*2.0).unwrap(), T::from_f64(f_end/sample_rate*2.0).unwrap()))
    }
    pub fn chirp_real(sample_rate: f64, len: usize, f_start: f64, f_end: f64) -> Vec<T> {
        chirp_real(len, f_start/sample_rate*2.0, f_end/sample_rate*2.0)
    }
}
#[test]
fn test_chirp() -> anyhow::Result<()> {
    // fwd complex
    let chirp = Signal::from_vec(192000.0, chirp_complex::<f64>(2048, -0.5, 0.5));
    spectrogram("plot/test/test_chirp/chirp_fwd_spect.png", chirp.clone(), 256, 128+64+32+16, true, Some(-40.0));
    plot_complex("plot/test/test_chirp/chirp_fwd.png", "Chirp [-0.5, 0.5]", &chirp);
    let chirp = chirp.fft_fwd()?;
    plot("plot/test/test_chirp/chirp_fwd_fft.png", "Chirp [-0.5, 0.5] [fft]", &chirp.abs());
    // rev complex
     let chirp = Signal::from_vec(192000.0, chirp_complex::<f64>(2048, 0.5, -0.5));
    spectrogram("plot/test/test_chirp/chirp_rev_spect.png", chirp.clone(), 256, 128+64+32+16, true, Some(-40.0));
    plot_complex("plot/test/test_chirp/chirp_rev.png", "Chirp [0.5, -0.5]", &chirp);
    let chirp = chirp.fft_fwd()?;
    plot("plot/test/test_chirp/chirp_rev_fft.png", "Chirp [0.5, -0.5] [fft]", &chirp.abs());

    // fwd small
    let chirp = Signal::from_vec(192000.0, chirp_complex::<f64>(2048, -0.01, 0.01));
    //spectrogram("plot/test/test_chirp/chirp_fwd_spect.png", chirp.clone(), 256, 128+64+32+16, true, Some(-40.0));
    plot_complex("plot/test/test_chirp/chirp_fwd_small.png", "Chirp [-0.1, 0.1]", &chirp);

    // fwd real
    let chirp = Signal::from_vec(192000.0, chirp_real::<f64>(2048, 0.0, 0.01));
    //spectrogram("plot/test/test_chirp/chirp_fwd_spect.png", chirp.clone(), 256, 128+64+32+16, true, Some(-40.0));
    plot_complex("plot/test/test_chirp/real_chirp_fwd_small.png", "Chirp [-0.1, 0.1]", &chirp);
    Ok(())
}