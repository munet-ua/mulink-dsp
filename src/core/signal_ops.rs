use itertools::Itertools;

pub use crate::prelude::*;

// Vector Signal ops

impl<T: SignalType> Signal<T> {
    pub fn abs(&self) -> Vec<T> {
        self.iter().map(|x| x.norm()).collect_vec()
    }
    pub fn re(&self) -> Vec<T> {
        self.iter().map(|x| x.re).collect_vec()
    }
    pub fn im(&self) -> Vec<T> {
        self.iter().map(|x| x.im).collect_vec()
    }
    pub fn conj(mut self) -> Self {
        self.iter_mut().for_each(|x| *x=x.conj());
        self
    }
}
impl<T: SignalType> std::ops::Mul for &Signal<T> {
    type Output = Signal<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut out = Signal::from_vec(self.sample_rate, self.iter().zip(rhs.iter()).map(|(a,b)| a*b).collect_vec());
        out.time = i64::max(self.time, rhs.time);
        out
    }
}
impl<T: SignalType> std::ops::Add for &Signal<T> {
    type Output = Signal<T>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut out = Signal::from_vec(self.sample_rate, self.iter().zip(rhs.iter()).map(|(a,b)| a+b).collect_vec());
        out.time = i64::max(self.time, rhs.time);
        out
    }
}
impl<T: SignalType> std::ops::AddAssign for Signal<T> {
    
    fn add_assign(&mut self, rhs: Self) {
        self.iter_mut().zip(rhs.iter()).for_each(|(a,b)| *a += b );
    }
}

impl<T: SignalType> std::ops::MulAssign for Signal<T> {
    
    fn mul_assign(&mut self, rhs: Self) {
        self.iter_mut().zip(rhs.iter()).for_each(|(a,b)| *a *= b );
    }
}