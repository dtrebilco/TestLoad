// Below code converted from:
// Pseudorandom number generator.-- Thatcher Ulrich 2003
// This source code has been donated to the Public Domain.  Do
// whatever you want with it.

// PRNG code adapted from the complimentary-multiply-with-carry
// code in the article: George Marsaglia, "Seeds for Random Number
// Generators", Communications of the ACM, May 2003, Vol 46 No 5,
// pp90-93.
//
// The article says:
//
// "Any one of the choices for seed table size and multiplier will
// provide a RNG that has passed extensive tests of randomness,
// particularly those in [3], yet is simple and fast --
// approximately 30 million random 32-bit integers per second on a
// 850MHz PC.  The period is a*b^n, where a is the multiplier, n
// the size of the seed table and b=2^32-1.  (a is chosen so that
// b is a primitive root of the prime a*b^n + 1.)"
//
// [3] Marsaglia, G., Zaman, A., and Tsang, W.  Toward a universal
// random number generator.  _Statistics and Probability Letters
// 8_ (1990), 35-39.

// const Uint64 a = 18782; // for SEED_COUNT=4096, period approx 2^131104 (from Marsaglia usenet post 2003-05-13)
// const Uint64 a = 123471786; // for SEED_COUNT=1024, period approx 2^32794
// const Uint64 a = 123554632; // for SEED_COUNT=512, period approx 2^16410
// const Uint64 a = 8001634; // for SEED_COUNT=256, period approx 2^8182
// const Uint64 a = 8007626; // for SEED_COUNT=128, period approx 2^4118
// const Uint64 a = 647535442; // for SEED_COUNT=64, period approx 2^2077
// const Uint64 a = 547416522; // for SEED_COUNT=32, period approx 2^1053
// const Uint64 a = 487198574; // for SEED_COUNT=16, period approx  2^540
const SEED_COUNT : u32 = 8;

/// A random number generator that is deterministic and suitable for games.
pub struct GameRand {
	param_q : [u32; SEED_COUNT as usize],
	param_c : u32,
	param_i : u32,    
}

impl GameRand {

	/// Constructor that sets a random seed value on the number generator
	/// * `seed` - The seed value to use
    pub fn new(seed : u32) -> GameRand {
        let mut ret = GameRand {
            param_q : [0; SEED_COUNT as usize],
            param_c : 0,
            param_i : 0,
        };
        ret.seed_random(seed);
        ret
    }

	/// Reset the random seed value on the number generator (not necessary to call)
	/// * `seed` - The seed value to use
    pub fn seed_random(&mut self, seed : u32)
    {
        let mut j = seed;
        if j == 0 {
            j = 12345; // 0 is a terrible seed (probably the only bad choice), substitute something else:
        }
        for param in &mut self.param_q {
            j = j ^ (j << 13);
            j = j ^ (j >> 17);
            j = j ^ (j << 5);
            *param = j;
        }
    
        self.param_c = 362436;
        self.param_i = SEED_COUNT - 1;
    }

	/// Return the next pseudo-random number in the sequence.
    pub fn next_random(&mut self) -> u32 {
        let r : u32 = 0xFFFFFFFE;
        let a : u64 = 716514398; // for SEED_COUNT=8, period approx 2^285
    
        self.param_i = (self.param_i + 1) & (SEED_COUNT - 1);
        
        let q = &mut self.param_q[self.param_i as usize];
        let t : u64 = a * (*q as u64) + (self.param_c as u64);
        self.param_c = (t >> 32) as u32;

        let mut x = (t + self.param_c as u64) as u32;
        if x < self.param_c {
            x += 1;
            self.param_c = self.param_c.wrapping_add(1);
        }

        let val = r.wrapping_sub(x);
        *q = val;
        return val;
    }

	/// Return the next pseudo-random number in the 0..1 range.
    pub fn next_random01(&mut self) -> f32
    {
        const DIV : f32 = 1.0 / (u32::MAX as f32);
        let val = self.next_random();
        let val_f  = (val as f32) * DIV;
    
        return val_f;
    }

	/// Generate a pseudo-random number within a given bounds. Does not guarantee an exact even distribution 
    /// of values in the range, but if the range is small (<10000's) it is close to even.
	/// * `min` - The minimum bound of the random number, inclusive.
	/// * `max` - The maximum bound of the random number, inclusive.
	/// \precondition max >= min.
    pub fn rand_range(&mut self, min : u32, max : u32) -> u32
    {
        let mut val = self.next_random();

        let range_diff : u32 = max - min;
        if range_diff != u32::MAX {
            val %= range_diff + 1;
            val += min;
        }
    
        return val;
    }
}