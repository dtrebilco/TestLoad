use std::fs::File;
use std::io::BufReader;
use std::io::Read;

macro_rules! enum_load {
    ($(#[$derives:meta])* $vis:vis enum $name:ident { $($(#[$nested_meta:meta])* $variant:ident = $value:expr),+ $(,)? }) => {
        $(#[$derives])*        
        $vis enum $name {
            $($(#[$nested_meta])* $variant = $value),+
        }

        impl TryFrom<u32> for $name {
            type Error = ();

            fn try_from(value: u32) -> Result<Self, Self::Error> {
                match value {
                    $($value => Ok($name::$variant),)+
                    _ => Err(()),
                }
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


/*
impl TryFrom<u32> for PrimitiveType {
    //type Error = &'static str;
    type Error = ();//std::io::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
           0 => Ok(PrimitiveType::PRIM_TRIANGLES),
           1 => Ok(PrimitiveType::PRIM_QUADS),
           2 => Ok(PrimitiveType::PRIM_TRIANGLE_STRIP),
           3 => Ok(PrimitiveType::PRIM_LINES),
           _ => Err(())
           //_ => Err("Bad primitive type conversion")
        }
    }
}
*/

  

  
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


/*
void read_batch_from_file(FILE* file, Batch& batch) {
    fread(&batch.nVertices, sizeof(batch.nVertices), 1, file);
    fread(&batch.nIndices, sizeof(batch.nIndices), 1, file);
    fread(&batch.vertexSize, sizeof(batch.vertexSize), 1, file);
    fread(&batch.indexSize, sizeof(batch.indexSize), 1, file);
  
    fread(&batch.primitiveType, sizeof(batch.primitiveType), 1, file);
  
    unsigned int nFormats;
    fread(&nFormats, sizeof(nFormats), 1, file);
    batch.formats.resize(nFormats);
    fread(batch.formats.data(), nFormats * sizeof(Format), 1, file);
  
    batch.vertices = new char[batch.nVertices * batch.vertexSize];
    fread(batch.vertices, batch.nVertices * batch.vertexSize, 1, file);
  
    if (batch.nIndices > 0) {
      batch.indices = new char[batch.nIndices * batch.indexSize];
      fread(batch.indices, batch.nIndices * batch.indexSize, 1, file);
    }
    else batch.indices = NULL;
}
*/
/*
fn read_enum_from_file<P: AsRef<Path>>(path: P) -> io::Result<MyEnum> {
    let mut file = File::open(path)?;
    let mut buf = [0; 4];
    file.read_exact(&mut buf)?;
    let value = u32::from_le_bytes(buf);
    MyEnum::try_from(value).map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid enum value"))
}

use std::convert::TryFrom;
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::Path;

macro_rules! my_enum {
    // The enum name and possible visibility modifier
    ($vis:vis enum $name:ident { $($variant:ident),+ $(,)? }) => {
        #[derive(Debug)]
        $vis enum $name {
            $($variant),+
        }

        impl TryFrom<u32> for $name {
            type Error = ();

            fn try_from(value: u32) -> Result<Self, Self::Error> {
                match value {
                    $(x if x == $name::$variant as u32 => Ok($name::$variant),)+
                    _ => Err(()),
                }
            }
        }
    };
}

// Using the macro
my_enum! {
    pub enum MyEnum {
        Variant1,
        Variant2,
        Variant3,
    }
}

fn read_enums_from_file<P: AsRef<Path>>(path: P) -> io::Result<Vec<MyEnum>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut values = Vec::new();
    for chunk in reader.bytes().chunks(4) {
        let bytes = chunk?;
        if bytes.len() == 4 {
            let value = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
            match MyEnum::try_from(value) {
                Ok(e) => values.push(e),
                Err(_) => return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid enum value")),
            }
        } else if !bytes.is_empty() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid data length"));
        }
    }
    Ok(values)
}

fn read_enums_from_file<P: AsRef<Path>>(path: P) -> io::Result<Vec<MyEnum>> {
    let mut file = File::open(path)?;
    let mut reader = BufReader::new(&mut file);
    let mut values = Vec::new();
    let mut buffer = [0u8; 4];
    loop {
        match reader.read_exact(&mut buffer) {
            Ok(_) => {
                let value = u32::from_le_bytes(buffer);
                match MyEnum::try_from(value) {
                    Ok(e) => values.push(e),
                    Err(_) => return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid enum value")),
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::UnexpectedEof => break,
            Err(e) => return Err(e),
        }
    }
    Ok(values)
}


/// https://stackoverflow.com/a/64678145/10854888
macro_rules! iterable_enum {
    ($(#[$derives:meta])* $(vis $visibility:vis)? enum $name:ident { $($(#[$nested_meta:meta])* $member:ident),* }) => {
        const count_members:usize = $crate::count!($($member)*);
        $(#[$derives])*
        $($visibility)? enum $name {
            $($(#[$nested_meta])* $member),*
        }
        impl $name {
            pub const fn iter() -> [$name; count_members] {
                [$($name::$member,)*]
            }
        }
    };
}

*/

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

//bool make_model_renderable(Model& ret_model);
//bool get_bounding_box(const Model& model, vec3& min, vec3& max);
//bool transform_model(Model& ret_model, const mat4& mat);

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

    let test : u32 = 1;
    let test2 : PrimitiveType = PrimitiveType::try_from(test).unwrap();

    println!("MyEnum: {:?}", test2);
}
