use std::fs::File;
use std::io::{BufReader, Read};

pub enum EnumLoadError {
    InvalidData,
}

impl From<EnumLoadError> for std::io::Error {
    fn from(_: EnumLoadError) -> std::io::Error {
        std::io::Error::from(std::io::ErrorKind::InvalidData)
    }
}

macro_rules! enum_load {
    ($(#[$derives:meta])* $vis:vis enum $name:ident { $($(#[$nested_meta:meta])* $member:ident = $value:expr),+ $(,)? }) => {
        $(#[$derives])*        
        $vis enum $name {
            $($(#[$nested_meta])* $member = $value),+
        }

        impl TryFrom<u32> for $name {
            type Error = EnumLoadError;

            fn try_from(value: u32) -> Result<Self, Self::Error> {
                match value {
                    $($value => Ok($name::$member),)+
                    _ => Err(EnumLoadError::InvalidData),
                }
            }
        }

        impl $name {
            pub const fn count() -> usize {
                [$($name::$member),+].len()
            }

            pub const fn iter() -> [$name; Self::count()] {
                [$($name::$member),+]
            }            
        }
    };
}

macro_rules! enum_sequential {
    ($(#[$derives:meta])* $vis:vis enum $name:ident { $($(#[$nested_meta:meta])* $member:ident),+ $(,)? }) => {

        $(#[$derives])*
        $vis enum $name {
            $($(#[$nested_meta])* $member),+
        }

        impl TryFrom<u32> for $name {
            type Error = EnumLoadError;

            fn try_from(value: u32) -> Result<Self, Self::Error> {
                match value {
                    $(x if x == $name::$member as u32 => Ok($name::$member),)+
                    _ => Err(EnumLoadError::InvalidData),
                }
            }
        }

        impl $name {
            pub const fn len() -> usize {
                [$($name::$member),+].len()
            }

            pub const fn iter() -> [$name; Self::len()] {
                [$($name::$member),+]
            }            
        }
    };
}

enum_load! {
    #[derive(Debug)]
    pub enum PrimitiveType {
        Triangles      = 0,
        Quads          = 1,
        TriangleStrip  = 2,
        Lines          = 3,
    }
}

enum_load! {
    enum AttributeType {
        Vertex   = 0,
        Normal   = 1,
        Texcoord = 2,
        Color    = 3,
    }
}

enum_load! {
    enum AttributeFormat {
        Float        = 0,
        UnsignedByte = 1,
    }
}
  
pub struct Format {
    attrib_type   : AttributeType,
    attrib_format : AttributeFormat,
    size   : u32,
    offset : u32,
    index  : u32,
}

pub struct Batch
{
    num_vertices  : u32,
    num_indices   : u32,
    vertex_size : u32,
    index_size  : u32,

    primitive_type : PrimitiveType, 

    formats : Vec<Format>,

    vertices : Vec<u8>,
    indices  : Vec<u8>,
    
    //sg_buffer render_index;
    //sg_buffer render_vertex;
}
  
pub struct Model
{
    batches : Vec<Batch>, 
}

fn load_model_from_file(filename: &str) -> std::io::Result<Model> {

    // DT_TODO: Use non resizable arrays for the data storage RawVec / Unique<T> ? use buf_reader.read_buf_exact()
    // DT_TODO: Optimize by accessing the internal buffer directly fo small reads? buf_reader.buffer()

    let file = File::open(filename)?;
    let mut buf_reader = BufReader::with_capacity(64 * 1024, file);

    let mut buf = [0; 4];
    let mut read_u32 = | buf_reader : &mut BufReader<File> | -> std::io::Result<u32> {
        buf_reader.read_exact(&mut buf)?;        
        Ok(u32::from_le_bytes(buf))
    };

    let version = read_u32(&mut buf_reader)?;
    if version != 1 {
        return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
    }
    let num_batches = read_u32(&mut buf_reader)?;

    let mut out_model : Model = Model { batches : Vec::with_capacity(num_batches as usize) };
    for  _ in 0..num_batches {
        let num_vertices = read_u32(&mut buf_reader)?;
        let num_indices  = read_u32(&mut buf_reader)?;
        let vertex_size  = read_u32(&mut buf_reader)?;
        let index_size   = read_u32(&mut buf_reader)?;

        let primitive_type = PrimitiveType::try_from(read_u32(&mut buf_reader)?)?;
        let num_formats  = read_u32(&mut buf_reader)?;

        let mut new_batch = Batch {
            num_vertices,
            num_indices,
            vertex_size,
            index_size,
            primitive_type, 

            formats  : Vec::with_capacity(num_formats as usize),
            vertices : Vec::with_capacity(vertex_size as usize * num_vertices as usize),
            indices  : Vec::with_capacity(index_size as usize * num_indices as usize),
        };

        // Read formats
        for _ in 0..num_formats {
            let new_format = Format {
                attrib_type   : AttributeType::try_from(read_u32(&mut buf_reader)?)?,
                attrib_format : AttributeFormat::try_from(read_u32(&mut buf_reader)?)?,
                size   : read_u32(&mut buf_reader)?,
                offset : read_u32(&mut buf_reader)?,
                index  : read_u32(&mut buf_reader)?,
            };
            new_batch.formats.push(new_format);
        }

        // Read vertices
        new_batch.vertices.resize(new_batch.vertices.capacity(), 0);
        buf_reader.read_exact(new_batch.vertices.as_mut_slice())?;

        // Read indices
        if new_batch.num_indices > 0 {
            new_batch.indices.resize(new_batch.indices.capacity(), 0);
            buf_reader.read_exact(new_batch.indices.as_mut_slice())?;
        }

        out_model.batches.push(new_batch);
    }

    Ok(out_model)

}

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
	ParamQ : [u32; SEED_COUNT as usize],
	ParamC : u32,
	ParamI : u32,    
}

impl GameRand {

	/// Constructor that sets a random seed value on the number generator
	/// * `seed` - The seed value to use
    fn new(seed : u32) -> GameRand {
        let mut ret = GameRand {
            ParamQ : [0; SEED_COUNT as usize],
            ParamC : 0,
            ParamI : 0,
        };
        ret.seed_random(seed);
        ret
    }

	/// Reset the random seed value on the number generator (not necessary to call)
	/// * `seed` - The seed value to use
    fn seed_random(&mut self, seed : u32)
    {
        let mut j = seed;
        if j == 0 {
            j = 12345; // 0 is a terrible seed (probably the only bad choice), substitute something else:
        }
        for param in &mut self.ParamQ {
            j = j ^ (j << 13);
            j = j ^ (j >> 17);
            j = j ^ (j << 5);
            *param = j;
        }
    
        self.ParamC = 362436;
        self.ParamI = SEED_COUNT - 1;
    }

	/// Return the next pseudo-random number in the sequence.
    fn next_random(&mut self) -> u32 {
        let r : u32 = 0xFFFFFFFE;
        let a : u64 = 716514398; // for SEED_COUNT=8, period approx 2^285
    
        self.ParamI = (self.ParamI + 1) & (SEED_COUNT - 1);
        
        let t : u64 = a * (self.ParamQ[self.ParamI as usize] as u64) + (self.ParamC as u64);
        self.ParamC = (t >> 32) as u32;

        let mut x : u32 = (t + self.ParamC as u64) as u32;
        if x < self.ParamC {
            x = x + 1;
            self.ParamC = self.ParamC + 1;
        }
    
        let val : u32 = r - x;
        self.ParamQ[self.ParamI as usize] = val;
        return val;
    }

	/// Generate a pseudo-random number within a given bounds. Does not guarantee an exact even distribution 
    /// of values in the range, but if the range is small (<10000's) it is close to even.
	/// * `min` - The minimum bound of the random number, inclusive.
	/// * `max` - The maximum bound of the random number, inclusive.
	/// \precondition max >= min.
    fn rand_range(&mut self, min : u32, max : u32) -> u32
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


fn main() {
    println!("Hello, world!");

    if let Ok(model) = load_model_from_file("data/room0.hmdl") {
        for batch in model.batches {
            println!("Vertices {} Indices {}", batch.num_vertices, batch.num_indices);
        }
    }
    else {
        println!("Failure to read file!");
    }

    for p in PrimitiveType::iter() {
        println!("{:?}", p);
    }

    //println!("MyEnum: {:?} {test3}", test2);
}
