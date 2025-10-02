use std::{fs::File, io::BufReader};

use anyhow::Result;
use hound::SampleFormat;
use num::{cast::AsPrimitive, Complex, FromPrimitive};

use crate::prelude::*;



pub fn read_wav_complex<T: SignalType>(path: &str) -> Result<Signal<T>> {
    let mut reader = hound::WavReader::open(path)?;
    let spec = reader.spec();
    let len = reader.len() as usize;

    let signal: Vec<Complex<T>> =
        match spec.sample_format {
            hound::SampleFormat::Float => match spec.channels {
                1 => {
                    let samples: anyhow::Result<Vec<Complex<T>>> = reader
                        .samples::<f32>()
                        .try_fold(Vec::<Complex<T>>::with_capacity(len), |mut acc, sample| {
                            acc.push(Complex {
                                re: T::from_f32(sample?).unwrap_or_default(),
                                im: T::from_f32(0.0).unwrap_or_default(),
                            });
                            Ok(acc)
                        });
                    samples?
                }
                2 => {
                    let samples: anyhow::Result<Vec<f32>> = reader.samples::<f32>().try_fold(
                        Vec::<f32>::with_capacity(len),
                        |mut acc, sample| {
                            acc.push(sample?);
                            Ok(acc)
                        },
                    );
                    let samples: Vec<Complex<T>> = samples?.chunks_exact(2).fold(
                        Vec::<Complex<T>>::with_capacity(len),
                        |mut acc, sample| {
                            acc.push(Complex {
                                re: T::from_f32(sample[0]).unwrap_or_default(),
                                im: T::from_f32(sample[1]).unwrap_or_default(),
                            });
                            acc
                        },
                    );
                    samples
                }
                _ => {
                    return Err(anyhow::format_err!(
                        "Unsupported channels count: {}",
                        spec.channels
                    ))
                }
            },
            hound::SampleFormat::Int => match spec.channels {
                1 => {
                    let samples: anyhow::Result<Vec<Complex<T>>> = reader
                        .samples::<i32>()
                        .try_fold(Vec::<Complex<T>>::with_capacity(len), |mut acc, sample| {
                            acc.push(Complex {
                                re: T::from_i32(sample?).unwrap_or_default(),
                                im: T::from_i32(0).unwrap_or_default(),
                            });
                            Ok(acc)
                        });
                    samples?
                }
                2 => {
                    let samples: anyhow::Result<Vec<i32>> = reader.samples::<i32>().try_fold(
                        Vec::<i32>::with_capacity(len),
                        |mut acc, sample| {
                            acc.push(sample?);
                            Ok(acc)
                        },
                    );
                    let samples: Vec<Complex<T>> = samples?.chunks_exact(2).fold(
                        Vec::<Complex<T>>::with_capacity(len),
                        |mut acc, sample| {
                            acc.push(Complex {
                                re: T::from_i32(sample[0]).unwrap_or_default(),
                                im: T::from_i32(sample[1]).unwrap_or_default(),
                            });
                            acc
                        },
                    );
                    samples
                }
                _ => {
                    return Err(anyhow::format_err!(
                        "Unsupported channels count: {}",
                        spec.channels
                    ))
                }
            },
        };
    Ok(Signal::from_vec(spec.sample_rate as f64, signal))
}

pub fn write_wav_complex<T: SignalType>(
    path: &str,
    signal: &Signal<T>,
    sample_format: SampleFormat,
    normalize: bool,
) -> Result<()> {
    // add some error checks here for Int signals of unusually low precision. (and warnings) as well as out of bounds float signals
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: signal.sample_rate as u32,
        bits_per_sample: 32,
        sample_format: sample_format.into(),
    };
    let mut writer = hound::WavWriter::create(path, spec)?;
    let result: Result<()> = match sample_format {
        SampleFormat::Float => signal.iter().try_for_each(|x| {
            writer.write_sample::<f32>(x.re.as_())?;
            writer.write_sample::<f32>(x.im.as_())?;
            Ok(())
        }),
        SampleFormat::Int => signal.iter().try_for_each(|x| {
            writer.write_sample::<i32>(x.re.as_())?;
            writer.write_sample::<i32>(x.im.as_())?;
            Ok(())
        }),
    };
    result?;
    Ok(())
}
