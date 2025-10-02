use std::cmp::Ordering;

use itertools::Itertools;
use crate::core::block::filter;
use crate::core::signal::Amplitude;
use crate::plot::time::{plot, plot_complex};
use crate::{io::wav::read_wav_complex, logging::init_logging};

use crate::prelude::*;
use plotters::data::fitting_range;

fn max<T: SignalType>(a: &&T,b: &&T) -> Ordering {
    a.partial_cmp(b).unwrap_or(Ordering::Less)
}

fn max_idx<T:SignalType>(a: &(usize, T),b: &(usize, T)) -> Ordering {
    a.1.partial_cmp(&b.1).unwrap_or(Ordering::Less)
}
fn min_idx<T:SignalType>(a: &(usize, T),b: &(usize, T)) -> Ordering {
    a.1.partial_cmp(&b.1).unwrap_or(Ordering::Less).reverse()
}

pub fn find_direct_path<T: SignalType>(impulse_response: &Signal<T>, min_snr: Amplitude, min_required_spacing: usize) -> anyhow::Result<Vec<(usize, T)>> {
    let ir = impulse_response.abs();
    
    println!("min_snr:{:?} {:?} {}", T::one()/min_snr.linear(), min_snr, min_snr.linear::<f64>());
    let min_snr = T::one()/min_snr.linear();
    let max = ir.iter().max_by(max).unwrap();
    let candidates= ir.iter().enumerate().flat_map( | (idx,x)| if *x > *max*min_snr && idx > min_required_spacing {
        Some(idx)
    } else {None}).collect_vec();

    let mut filtered_candidates = Vec::from([candidates[0]]);
    candidates.windows(2).for_each(|x|if x[1]-x[0] > min_required_spacing {filtered_candidates.push(x[1]);});

    let sorted_peaks = filtered_candidates.iter().map(|x| (*x, ir[*x])).sorted_by(min_idx).collect_vec();
    Ok(sorted_peaks)
}