use std::fs::create_dir_all;

use clap::Parser;
use itertools::Itertools;

use crate::{core::{signal::Amplitude, util::find_direct_path::find_direct_path}, plot::time::plot, prelude::Signal};

/// Impulse response extraction program

#[derive(Debug,Clone,Parser)]
pub struct IrExtractArgs {
    src_signal: String,
    output_dir: String,
}

pub fn ir_extract(args: IrExtractArgs) -> anyhow::Result<()> {
    create_dir_all(args.output_dir)?;
    let signal = Signal::<f32>::read_wav(&args.src_signal)?;
    let mut filt = Signal::<f32>::chirp_complex(signal.sample_rate, (signal.sample_rate*0.02) as usize, -5000.0, 5000.0).conj();
    filt.reverse();
    let ir = signal.fftfilt(&filt, crate::core::block::filter::ConvShape::VALID)?;
    plot("plot/filter_output.png", "filter_output", &ir.abs());
    let peaks = find_direct_path(&ir, Amplitude::Log(6.0), 480)?;
    println!("Top 4: {:#?}", peaks[0..4].iter().collect_vec());
    Ok(())
}