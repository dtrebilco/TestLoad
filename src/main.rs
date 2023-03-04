mod game_rand;
use game_rand::GameRand;


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



fn main() {
    /*
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
    */
    let mut rand = GameRand::new(12345);

    for _ in 0..1_000_000 {

        println!("{}", rand.next_random());
        println!("{}", rand.rand_range(5, 1067));
    }

    //let val = rand.rand_range(&(0u32..=3));

    //println!("MyEnum: {:?} {test3}", test2);
}
