pub mod prelude;
pub mod logging;
pub mod core {
    pub mod signal;
    pub mod signal_ops;
    pub mod stream;
    pub mod block {
        pub mod fft;
        pub mod refragment;
        pub mod filter;
    }
    pub mod gen {
        pub mod fir;
        pub mod chirp;
        pub mod noise;
    }
    pub mod util {
        pub mod find_direct_path;
    }
}
pub mod io {
    pub mod wav;
}
pub mod plot {
    pub mod time;
    pub mod spectrum;
}
pub mod fs {
    pub mod init;
}
pub mod programs {
    pub mod ir_extract;
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
