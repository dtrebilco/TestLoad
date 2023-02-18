
#[repr(u32)]
enum PrimitiveType {
    PRIM_TRIANGLES      = 0,
    PRIM_QUADS          = 1,
    PRIM_TRIANGLE_STRIP = 2,
    PRIM_LINES          = 3,
}

#[repr(u32)]  
enum AttributeType {
    ATT_VERTEX   = 0,
    ATT_NORMAL   = 1,
    ATT_TEXCOORD = 2,
    ATT_COLOR    = 3,
}
  
#[repr(u32)]  
enum AttributeFormat {
    ATT_FLOAT         = 0,
    ATT_UNSIGNED_BYTE = 1,
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
  
fn load_model_from_file(filename: &str) -> Option<Model> {
    None
}

//bool make_model_renderable(Model& ret_model);
//bool get_bounding_box(const Model& model, vec3& min, vec3& max);
//bool transform_model(Model& ret_model, const mat4& mat);

fn main() {
    println!("Hello, world!");
}
