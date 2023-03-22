//#[cfg(windows)]

use windows_sys::Win32::System::Performance::*;

pub struct Timer {
    freq: u64,
    start: u64,
}

// Computes (value*numer)/denom without overflow, as long as both
// (numer*denom) and the overall result fit into u64 (which is the case
// for our time conversions).
fn mul_div_u64(value: u64, numer: u64, denom: u64) -> u64 {
    let q = value / denom;
    let r = value % denom;
    // Decompose value as (value/denom*denom + value%denom),
    // substitute into (value*numer)/denom and simplify.
    // r < denom, so (denom*numer) is the upper bound of (r*numer)
    q * numer + r * numer / denom
}

impl Timer {
    pub fn new() -> Timer {
        let mut freq = 0;
        let mut start = 0;
        unsafe {
            QueryPerformanceFrequency(&mut freq);
            QueryPerformanceCounter(&mut start);
        }
        Timer {
            freq: freq as u64,
            start: start as u64,
        }
    }

    pub fn now(&self) -> u64 {
        let mut qpc = 0;
        unsafe {
            QueryPerformanceCounter(&mut qpc);
        }
        let now = mul_div_u64((qpc as u64) - self.start, 1_000_000_000, self.freq);
        now
    }

    pub fn diff(new_ticks: u64, old_ticks: u64) -> u64 {
        if new_ticks > old_ticks {
            return new_ticks - old_ticks;
        }
        1
    }

    pub fn laptime(&self, last_time : &mut u64) -> u64 {

        let mut dt : u64 = 0;
        let now = self.now();
        if 0 != *last_time {
            dt = Self::diff(now, *last_time);
        }
        *last_time = now;
        dt
    }

    pub fn sec(ticks: u64) -> f64 {
        ticks as f64 / 1_000_000_000.0
    }

    pub fn ms(ticks: u64) -> f64 {
        ticks as f64 / 1_000_000.0
    }

    pub fn us(ticks: u64) -> f64 {
        ticks as f64 / 1000.0
    }

    pub fn ns(ticks: u64) -> f64 {
        ticks as f64
    }
}
