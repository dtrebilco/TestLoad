use std::fs::File;
use std::io::{BufReader, Read};

macro_rules! enum_load {
    ($(#[$derives:meta])* $vis:vis enum $name:ident { $($(#[$nested_meta:meta])* $member:ident = $value:expr),+ $(,)? }) => {
        $(#[$derives])*        
        $vis enum $name {
            $($(#[$nested_meta])* $member = $value),+
        }

        impl TryFrom<u32> for $name {
            type Error = ();

            fn try_from(value: u32) -> Result<Self, Self::Error> {
                match value {
                    $($value => Ok($name::$member),)+
                    _ => Err(()),
                }
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
            type Error = ();

            fn try_from(value: u32) -> Result<Self, Self::Error> {
                match value {
                    $(x if x == $name::$member as u32 => Ok($name::$member),)+
                    _ => Err(()),
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

enum_sequential! {
    #[derive(Debug)]
    pub enum PrimitiveType2 {
        Triangles      ,
        Quads          ,
        TriangleStrip  ,
        Lines          ,
    }
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
  
struct Format {
    attType   : AttributeType,
    attFormat : AttributeFormat,
    size   : u32,
    offset : u32,
    index  : u32,
}

struct Batch
{
    vertices : Vec<u8>,
    indices  : Vec<u8>,
  
    nVertices  : u32,
    nIndices   : u32,
    vertexSize : u32,
    indexSize  : u32,
  
    formats : Vec<Format>,
    primitiveType : PrimitiveType, 
  
    //sg_buffer render_index;
    //sg_buffer render_vertex;
}
  
struct Model
{
    batches : Vec<Batch>, 
}

fn load_model_from_file(filename: &str) -> std::io::Result<Model> {
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
    let nBatches = read_u32(&mut buf_reader)?;

    let mut outModel : Model = Model { batches : Vec::with_capacity(nBatches as usize) };
    for  _ in 0..nBatches {
        let nVertices  = read_u32(&mut buf_reader)?;
        let nIndices   = read_u32(&mut buf_reader)?;
        let vertexSize = read_u32(&mut buf_reader)?;
        let indexSize  = read_u32(&mut buf_reader)?;

        let primType  = read_u32(&mut buf_reader)?;
        let nFormats  = read_u32(&mut buf_reader)?;

        let primitiveType = match PrimitiveType::try_from(primType) {
            Ok(e) => e,
            Err(_) => return Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
        };

        let mut newBatch = Batch {
            vertices : Vec::with_capacity(vertexSize as usize * nVertices as usize),
            indices  : Vec::with_capacity(indexSize as usize * nIndices as usize),
          
            nVertices,
            nIndices,
            vertexSize,
            indexSize,
          
            formats : Vec::with_capacity(nFormats as usize),
            primitiveType, 

        };

        // Read formats
        for _ in 0..nFormats {

            let attType = read_u32(&mut buf_reader)?;
            let attType = match AttributeType::try_from(attType) {
                Ok(e) => e,
                Err(_) => return Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
            };
    
            let attFormat = read_u32(&mut buf_reader)?;
            let attFormat = match AttributeFormat::try_from(attFormat) {
                Ok(e) => e,
                Err(_) => return Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
            };

            let mut newFormat = Format {
                attType,
                attFormat,
                size   : read_u32(&mut buf_reader)?,
                offset : read_u32(&mut buf_reader)?,
                index  : read_u32(&mut buf_reader)?,
            };
            newBatch.formats.push(newFormat);
        }

        // Read vertices
        newBatch.vertices.resize(newBatch.vertices.capacity(), 0);
        buf_reader.read_exact(newBatch.vertices.as_mut_slice())?;

        // Read indices
        if newBatch.nIndices > 0 {
            newBatch.indices.resize(newBatch.indices.capacity(), 0);
            buf_reader.read_exact(newBatch.indices.as_mut_slice())?;
        }

        //DT_TODO:  Use non resizable arrays for the data storage    
        outModel.batches.push(newBatch);
    }

    Ok(outModel)

}

fn main() {
    println!("Hello, world!");

    if let Ok(model) = load_model_from_file("data/room0.hmdl") {
        for batch in model.batches {
            println!("Vertices {} Indices {}", batch.nVertices, batch.nIndices);
        }
    }
    else {
        println!("Failure to read file!");
    }

    for p in PrimitiveType2::iter() {
        println!("{:?}", p);
    }

    //println!("MyEnum: {:?} {test3}", test2);
}
