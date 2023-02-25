use std::fs::File;
use std::io::BufReader;
use std::io::Read;

macro_rules! enum_load {
    ($(#[$derives:meta])* $vis:vis enum $name:ident { $($variant:ident = $value:expr),+ $(,)? }) => {
        $(#[$derives])*        
        $vis enum $name {
            $($variant = $value),+
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
    let mut file = File::open(filename)?;
    let mut buf_reader = BufReader::with_capacity(64 * 1024, file);

    //buf_reader.bytes().array_chunks();//.chunks(4);
    buf_reader.buffer();

    //buf_reader.read_vectored(bufs)

    //buf_reader.read_buf_exact(cursor)

    let mut buf = [0; 4];
    buf_reader.read_exact(&mut buf)?;

    let a = u32::from_le_bytes(buf);

    let test3 = PrimitiveType::Triangles as u32;
    //let test4 = test3 as PrimitiveType;

    let test : u32 = 4;
    let test2 : PrimitiveType = PrimitiveType::try_from(test).unwrap();

    let mut outModel : Model = Model { batches : Vec::with_capacity(10) };

    let mut version: u32 = 0;
    let mut n_batches: u32 = 0;

    //file.read_exact(std::slice::from_mut(&mut version))?;

    unsafe fn slice_to_u8_mut<T: Copy>(slice: &mut [T]) -> &mut [u8] {
        use std::mem::size_of;
    
        let len = size_of::<T>() * slice.len();
        std::slice::from_raw_parts_mut(slice.as_mut_ptr() as *mut u8, len)
    }

    let s: &mut [u8] = unsafe {

        std::slice::from_mut(&mut version).as_mut_ptr();

        // get a mut ptr to the start of the slice where data will be copied into
        let ptr = (&mut version as *mut _ as *mut u8);
        // form a u8 slice of the desired length, starting at `ptr`
        std::slice::from_raw_parts_mut(ptr, 4)
    };
    buf_reader.read_exact(s)?;

/*
  FILE* file = fopen(fileName, "rb");
  if (file == NULL) return false;

  uint32_t version;
  fread(&version, sizeof(version), 1, file);
  uint32_t nBatches;
  fread(&nBatches, sizeof(nBatches), 1, file);

  for (unsigned int i = 0; i < nBatches; i++) {
    Batch batch = {};
    read_batch_from_file(file, batch);
    ret_model.batches.push_back(batch);
  }
  fclose(file);
 */

    Ok(outModel)
    //None
}

//bool make_model_renderable(Model& ret_model);
//bool get_bounding_box(const Model& model, vec3& min, vec3& max);
//bool transform_model(Model& ret_model, const mat4& mat);

fn main() {
    println!("Hello, world!");

    let test : u32 = 1;
    let test2 : PrimitiveType = PrimitiveType::try_from(test).unwrap();

    println!("MyEnum: {:?}", test2);
}
