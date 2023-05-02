//#![windows_subsystem = "windows"]

mod base_app;
mod game_rand;
mod model;
mod particle_system;
mod sapp;
mod sgfx;
mod timer;
mod vector;

use base_app::*;
use game_rand::GameRand;
use model::*;
use particle_system::*;
use sapp::*;
use sgfx::*;
use timer::*;
use vector::*;

const MAX_PFX_PARTICLES : u32 = 1200;
const MAX_TOTAL_PARTICLES : u32 = MAX_PFX_PARTICLES * 5;
const PFX_VERTEX_SIZE : u32 = (4 * 3) + (4 * 2) + (4 * 4);

struct Light {
    particles : ParticleSystem,
    position : vec3,
    radius : f32,
    xs :f32,
    ys :f32,
    zs :f32,
}

impl Light {

    fn new(position :vec3, radius : f32, xs : f32, ys : f32, zs: f32) -> Light
    {
        let mut new_light = Light {
            particles: ParticleSystem::new(),
            position,
            radius,
            xs,
            ys,
            zs,
        };

        new_light.particles.spawn_rate = 400.0;
        new_light.particles.speed = 70.0;
        new_light.particles.speed_spread = 20.0;
        new_light.particles.life = 3.0;
        new_light.particles.life_spread = 0.0;
        new_light.particles.directional_force = vec3(0.0, -10.0, 0.0);
        new_light.particles.friction_factor = 0.95;
        //particles.setPosition(pos);
        new_light.particles.size = 15.0;
        new_light.particles.size_spread = 5.0;
    
        for i in 0..6 {
          let i_f = i as f32;  
          new_light.particles.colors[i] = vec4(0.05 * i_f, 0.01 * i_f, 0.0, 0.0);
          new_light.particles.colors[i+6] = vec4(0.05 * 6.0, 0.05 * i_f + 0.06, 0.02 * i_f, 0.0);
        }

        new_light
    }

    fn calc_light_offset(&self, t : f32, j : f32) -> vec3 
    {
        vec3(self.xs * f32::cos(4.23 * t + j), self.ys * f32::sin(2.37 * t) * f32::cos(1.39 * t), self.zs * f32::sin(3.12 * t + j))
    }

}

struct Portal {
    v : [vec3; 4],
    sector: u32,
}
impl Portal {  
  fn new(sector :u32, vc0 : &vec3, vc1: &vec3, vc2 : &vec3) -> Portal {
    Portal {
    sector,
    v : [*vc0, *vc1, *vc1 + *vc2 - *vc0, *vc2],
    }
  }
}

struct Sector {
    room : Model,
    portals : Vec<Portal>,
    lights : Vec<Light>,
    min : vec3,
    max : vec3,
    has_been_drawn: bool,
}

impl Sector {
    fn new() -> Sector {
        Sector {
            room: Model { batches: Vec::with_capacity(0) },
            portals: Vec::with_capacity(1),
            lights: Vec::with_capacity(1),
            min: vec3(0.0,0.0,0.0),
            max: vec3(0.0,0.0,0.0),
            has_been_drawn: false
        }
    }

    fn is_in_bounding_box(&self, pos: &vec3) -> bool {
        return 
        pos.x > self.min.x && pos.x < self.max.x &&
        pos.y > self.min.y && pos.y < self.max.y &&
        pos.z > self.min.z && pos.z < self.max.z;
      }
    
      fn is_sphere_in_sector(&self, pos : &vec3, radius : f32) -> bool {
        self.get_distance_sqr(pos) < (radius * radius)
      }
    
      fn get_distance_sqr(&self, pos : &vec3) -> f32 {
        let mut d = 0.0f32;
        for i in 0..3 {
          if pos[i] < self.min[i] {
            let s = pos[i] - self.min[i];
            d += s * s;
          }
          else if pos[i] > self.max[i] {
            let s = pos[i] - self.max[i];
            d += s * s;
          }
        }
        d
      }
}

struct App {
    timer: Timer,

    sectors : [Sector; 5],
  
    shader : sg_shader, 
    base : [sg_image; 3],
    bump : [sg_image; 3],
    room_pipline : sg_pipeline,
    room_pipline_blend : sg_pipeline,
  
    pfx_shader : sg_shader, 
    pfx_particle : sg_image,
    pfx_pipline : sg_pipeline,
  
    pfx_index : sg_buffer,
    pfx_vertex : sg_buffer, 
}

impl AppI for App {
    fn init(&mut self, _app: &mut BaseData, sapp: &mut SAppData) {
        println!("Startup time {} ms", Timer::ms(self.timer.now()));

        let mut icon = SappIconDesc::new();
        icon.sokol_default = true;
        sapp.set_icon(&icon);

        sapp.set_mouse_cursor(MouseCursor::ResizeAll);

        sapp.set_clipboard_string("test string 😀");
        println!("Clipboard:{}", sapp.get_clipboard_string());
    }

    fn reset_camera(&mut self, app: &mut BaseData, _sapp: &mut SAppData) {
        app.cam_pos = vec3(470.0, 220.0, 210.0);
        app.wx = 0.0;
        app.wy = std::f32::consts::PI / 2.0;
        app.wz = 0.0;
    }

    fn load(&mut self, _app: &mut BaseData, _sapp: &mut SAppData) -> bool {
        {
            let indices = vec![0u16; MAX_TOTAL_PARTICLES as usize * 6];
            for i in 0..MAX_TOTAL_PARTICLES {
              let offset = i as usize * 6;
              let index_offset = i as u16 * 4;
              indices[offset + 0] = index_offset + 0;
              indices[offset + 1] = index_offset + 1;
              indices[offset + 2] = index_offset + 3;
              indices[offset + 3] = index_offset + 2;
              indices[offset + 4] = index_offset + 3;
              indices[offset + 5] = index_offset + 1;
            }
            pfx_index = sg_make_buffer(sg_buffer_desc{
                .type = SG_BUFFERTYPE_INDEXBUFFER,
                .data = sg_range{.ptr = indices.data(), .size = (indices.size() * sizeof(uint16_t)) } ,
              });
          }
          pfx_vertex = sg_make_buffer(sg_buffer_desc{
              .size = MAX_TOTAL_PARTICLES * PFX_VERTEX_SIZE * 4,
              .usage = SG_USAGE_STREAM
            });
        
          // create an image 
          sg_image_desc imageDesc = {
            .min_filter = SG_FILTER_LINEAR_MIPMAP_NEAREST,
            .mag_filter = SG_FILTER_LINEAR,
            .wrap_u = SG_WRAP_REPEAT,
            .wrap_v = SG_WRAP_REPEAT,
          };
        
          {
            sg_shader_desc shaderDesc = {};
            shaderDesc.attrs[0] = { .name = "position" };
            shaderDesc.attrs[1] = { .name = "uv" };
            shaderDesc.attrs[2] = { .name = "mat0" };
            shaderDesc.attrs[3] = { .name = "mat1" };
            shaderDesc.attrs[4] = { .name = "mat2" };
        
            shaderDesc.vs.source = vs_src2;
            shaderDesc.vs.entry = "main";
            shaderDesc.vs.uniform_blocks[0].size = 6 * 16;
            shaderDesc.vs.uniform_blocks[0].layout = SG_UNIFORMLAYOUT_STD140;
            shaderDesc.vs.uniform_blocks[0].uniforms[0].name = "vs_params";
            shaderDesc.vs.uniform_blocks[0].uniforms[0].type = SG_UNIFORMTYPE_FLOAT4;
            shaderDesc.vs.uniform_blocks[0].uniforms[0].array_count = 6;
        
            shaderDesc.fs.source = fs_src2;
            shaderDesc.fs.entry = "main";
            shaderDesc.fs.uniform_blocks[0].size = 1 * 16;
            shaderDesc.fs.uniform_blocks[0].layout = SG_UNIFORMLAYOUT_STD140;
            shaderDesc.fs.uniform_blocks[0].uniforms[0].name = "fs_params";
            shaderDesc.fs.uniform_blocks[0].uniforms[0].type = SG_UNIFORMTYPE_FLOAT4;
            shaderDesc.fs.uniform_blocks[0].uniforms[0].array_count = 1;
            shaderDesc.fs.images[0].name = "Base";
            shaderDesc.fs.images[0].image_type = SG_IMAGETYPE_2D;
            shaderDesc.fs.images[0].sampler_type = SG_SAMPLERTYPE_FLOAT;
            shaderDesc.fs.images[1].name = "Bump";
            shaderDesc.fs.images[1].image_type = SG_IMAGETYPE_2D;
            shaderDesc.fs.images[1].sampler_type = SG_SAMPLERTYPE_FLOAT;
        
            shader = sg_make_shader(shaderDesc);
          }
        
          {
            sg_shader_desc shaderDesc = {};
            shaderDesc.attrs[0] = { .name = "position" };
            shaderDesc.attrs[1] = { .name = "in_uv" };
            shaderDesc.attrs[2] = { .name = "in_color" };
        
            shaderDesc.vs.source = vs_src_pfx;
            shaderDesc.vs.entry = "main";
            shaderDesc.vs.uniform_blocks[0].size = 4 * 16;
            shaderDesc.vs.uniform_blocks[0].layout = SG_UNIFORMLAYOUT_STD140;
            shaderDesc.vs.uniform_blocks[0].uniforms[0].name = "vs_params";
            shaderDesc.vs.uniform_blocks[0].uniforms[0].type = SG_UNIFORMTYPE_FLOAT4;
            shaderDesc.vs.uniform_blocks[0].uniforms[0].array_count = 4;
        
            shaderDesc.fs.source = fs_src_pfx;
            shaderDesc.fs.entry = "main";
            shaderDesc.fs.images[0].name = "tex0";
            shaderDesc.fs.images[0].image_type = SG_IMAGETYPE_2D;
            shaderDesc.fs.images[0].sampler_type = SG_SAMPLERTYPE_FLOAT;
        
            pfx_shader = sg_make_shader(shaderDesc);
          }
        
          base[0] = create_texture("data/Wood.png", imageDesc);
          base[1] = create_texture("data/laying_rock7.png", imageDesc);
          base[2] = create_texture("data/victoria.png", imageDesc);
        
          bump[0] = create_texture("data/Wood_N.png", imageDesc);
          bump[1] = create_texture("data/laying_rock7_N.png", imageDesc);
          bump[2] = create_texture("data/victoria_N.png", imageDesc);
        
          sg_image_desc pfx_imageDesc = {
            .min_filter = SG_FILTER_LINEAR_MIPMAP_NEAREST,
            .mag_filter = SG_FILTER_LINEAR,
            .wrap_u = SG_WRAP_CLAMP_TO_EDGE,
            .wrap_v = SG_WRAP_CLAMP_TO_EDGE,
          };
          pfx_particle = create_texture("data/Particle.png", pfx_imageDesc);
        
          auto load_model = [](const char* filename, Sector& sector, vec3 offset) {
            load_model_from_file(filename, sector.room);
        
            //mat4 mat(1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1);
            //mat.translate(offset);
            mat4 mat(
              vec4(1.0, 0.0, 0.0, 0.0),
              vec4(0.0, 1.0, 0.0, 0.0),
              vec4(0.0, 0.0, 1.0, 0.0),
              vec4(offset, 1.0));
        
            transform_model(sector.room, mat);
        
            // Calculate min/max bounds
            get_bounding_box(sector.room, sector.min, sector.max);
            make_model_renderable(sector.room);
          };
        
          load_model("data/room0.hmdl", sectors[0], vec3(0, 256, 0));
          load_model("data/room1.hmdl", sectors[1], vec3(-384, 256, 3072));
          load_model("data/room2.hmdl", sectors[2], vec3(1536, 256, 2688));
          load_model("data/room3.hmdl", sectors[3], vec3(-1024, -768, 2688));
          load_model("data/room4.hmdl", sectors[4], vec3(-2304, 256, 2688));
        
          // Setup portals
          sectors[0].portals.push_back(Portal(1, vec3(-384, 384, 1024), vec3(-128, 384, 1024), vec3(-384, 0, 1024)));
          sectors[1].portals.push_back(Portal(0, vec3(-384, 384, 1024), vec3(-128, 384, 1024), vec3(-384, 0, 1024)));
        
          sectors[1].portals.push_back(Portal(2, vec3(512, 384, 2816), vec3(512, 384, 3072), vec3(512, 0, 2816)));
          sectors[2].portals.push_back(Portal(1, vec3(512, 384, 2816), vec3(512, 384, 3072), vec3(512, 0, 2816)));
        
          sectors[2].portals.push_back(Portal(3, vec3(512, -256, 2304), vec3(512, -256, 2560), vec3(512, -640, 2304)));
          sectors[3].portals.push_back(Portal(2, vec3(512, -256, 2304), vec3(512, -256, 2560), vec3(512, -640, 2304)));
        
          sectors[1].portals.push_back(Portal(4, vec3(-1280, 384, 1664), vec3(-1280, 384, 1920), vec3(-1280, 128, 1664)));
          sectors[4].portals.push_back(Portal(1, vec3(-1280, 384, 1664), vec3(-1280, 384, 1920), vec3(-1280, 128, 1664)));
        
          sectors[1].portals.push_back(Portal(4, vec3(-1280, 192, 3840), vec3(-1280, 192, 4096), vec3(-1280, -256, 3840)));
          sectors[4].portals.push_back(Portal(1, vec3(-1280, 192, 3840), vec3(-1280, 192, 4096), vec3(-1280, -256, 3840)));
        
          // Setup lights
          sectors[0].lights.push_back(Light(vec3(0, 128, 0), 800, 100, 100, 100));
        
          sectors[1].lights.push_back(Light(vec3(-256, 224, 1800), 650, 100, 80, 100));
          sectors[1].lights.push_back(Light(vec3(-512, 128, 3100), 900, 100, 100, 300));
        
          sectors[2].lights.push_back(Light(vec3(1300, 128, 2700), 800, 100, 100, 200));
        
          sectors[3].lights.push_back(Light(vec3(-100, -700, 2432), 600, 50, 50, 50));
          sectors[3].lights.push_back(Light(vec3(-1450, -700, 2900), 1200, 250, 80, 250));
        
          sectors[4].lights.push_back(Light(vec3(-2200, 256, 2300), 800, 100, 100, 100));
          sectors[4].lights.push_back(Light(vec3(-2000, 0, 4000), 800, 100, 100, 100));
        
          {
            sg_pipeline_desc roomPipDesc = {};
            roomPipDesc.layout.attrs[0] = { .offset = 0, .format = SG_VERTEXFORMAT_FLOAT3 }; // position
            roomPipDesc.layout.attrs[1] = { .offset = 12, .format = SG_VERTEXFORMAT_FLOAT2 }; // uv
            roomPipDesc.layout.attrs[2] = { .offset = 20, .format = SG_VERTEXFORMAT_FLOAT3 }; // mat0
            roomPipDesc.layout.attrs[3] = { .offset = 32, .format = SG_VERTEXFORMAT_FLOAT3 }; // mat1
            roomPipDesc.layout.attrs[4] = { .offset = 44, .format = SG_VERTEXFORMAT_FLOAT3 }; // mat2
            roomPipDesc.shader = shader;
            roomPipDesc.index_type = SG_INDEXTYPE_UINT16;
            roomPipDesc.depth = {
                .compare = SG_COMPAREFUNC_LESS_EQUAL,
                .write_enabled = true,
            };
            roomPipDesc.cull_mode = SG_CULLMODE_BACK;
            //roomPipDesc.face_winding = SG_FACEWINDING_CCW;
            room_pipline = sg_make_pipeline(roomPipDesc);
        
            roomPipDesc.colors[0].blend = {
                .enabled = true,
                .src_factor_rgb = SG_BLENDFACTOR_ONE,
                .dst_factor_rgb = SG_BLENDFACTOR_ONE,
                .src_factor_alpha = SG_BLENDFACTOR_ONE,
                .dst_factor_alpha = SG_BLENDFACTOR_ONE,
            };
            room_pipline_blend = sg_make_pipeline(roomPipDesc);
          }
        
          {
            sg_pipeline_desc pipDesc = {};
            pipDesc.layout.attrs[0] = { .offset = 0, .format = SG_VERTEXFORMAT_FLOAT3 }; // position
            pipDesc.layout.attrs[1] = { .offset = 12, .format = SG_VERTEXFORMAT_FLOAT2 }; // uv
            pipDesc.layout.attrs[2] = { .offset = 20, .format = SG_VERTEXFORMAT_FLOAT4 }; // color
            pipDesc.shader = pfx_shader;
            pipDesc.index_type = SG_INDEXTYPE_UINT16;
            pipDesc.depth = {
                .compare = SG_COMPAREFUNC_LESS_EQUAL,
                .write_enabled = false,
            };
            pipDesc.colors[0].blend = {
                .enabled = true,
                .src_factor_rgb = SG_BLENDFACTOR_ONE,
                .dst_factor_rgb = SG_BLENDFACTOR_ONE,
                .src_factor_alpha = SG_BLENDFACTOR_ONE,
                .dst_factor_alpha = SG_BLENDFACTOR_ONE,
            };
            pipDesc.cull_mode = SG_CULLMODE_BACK;
            //pipDesc.face_winding = SG_FACEWINDING_CCW;
            pfx_pipline = sg_make_pipeline(pipDesc);
          }

        true
    }

    fn on_event(&mut self, _app: &mut BaseData, sapp: &mut SAppData, event: &Event) -> bool {
        if let Event::FilesDropped = event {
            for str in sapp.get_dropped_file_paths() {
                println!("File path {str}");
            }
        }
        if let Event::Key(data) = event {
            if data.pressed && data.key_code == KeyCode::T {
                sapp.toggle_fullscreen();
            }
        }

        false // DT_TODO: Use enum here
    }

    fn draw_frame(&mut self, _app: &mut BaseData, _sapp: &mut SAppData) {}

}

fn main() {
    let mut title: String = "Test window title 😀".to_string();
    let mut desc = sapp::SAppDesc::new();
    desc.window_title = &title;
    desc.enable_clipboard = true;
    desc.clipboard_size = 1024;
    desc.max_dropped_files = 5;
    //desc.sample_count = 16;
    //desc.win32_console_utf8 = true;
    desc.win32_console_create = true;
    //desc.win32_console_attach = true;
    //desc.fullscreen = true;

    let App = App {
        timer: Timer::new(),
        sectors : core::array::from_fn(|_| Sector::new()),
  
        shader : sg_shader::default(), 
        base : [sg_image::default(); 3],
        bump : [sg_image::default(); 3],
        room_pipline : sg_pipeline::default(),
        room_pipline_blend : sg_pipeline::default(),
      
        pfx_shader : sg_shader::default(), 
        pfx_particle : sg_image::default(),
        pfx_pipline : sg_pipeline::default(),
      
        pfx_index : sg_buffer::default(),
        pfx_vertex : sg_buffer::default(),

    };
    base_app::run_app(App, &desc);

    let mut p = ParticleSystem::new();
    p.set_color_scheme(ColorScheme::Rainbow);

    let mut rand = GameRand::new(1235);

    p.update(0.1, &mut rand);
    p.update(0.2, &mut rand);
    p.update(0.5, &mut rand);

    println!("Particle count {}", p.get_particle_count());
    let ia = p.get_index_array();
    println!("Index array size {}", ia.len());
    let va = p.get_vertex_array(vec3(0.0, 1.0, 2.0), vec3(0.0, 1.0, 2.0), true, false);
    println!("Vertex array size {}", va.len());

    p.spawn_rate = 0.0;
    p.update(50.0, &mut rand);
    println!("Particle count {}", p.get_particle_count());
    let ia = p.get_index_array();
    println!("Index array size {}", ia.len());
    let va = p.get_vertex_array(vec3(0.0, 1.0, 2.0), vec3(0.0, 1.0, 2.0), true, false);
    println!("Vertex array size {}", va.len());

    if let Ok(model) = model::Model::new("data/room0.hmdl") {
        for batch in model.batches {
            println!(
                "Vertices {} Indices {}",
                batch.num_vertices, batch.num_indices
            );
        }
    } else {
        println!("Failure to read file!");
    }

    for p in PrimitiveType::iter() {
        println!("{:?}", p);
    }

    let mut v = vec2(0.0, 1.0);
    v += vec2(2.0, 3.0);
    v -= vec2(2.0, 3.0);
    v *= 3.0;
    v /= 3.0;

    /*/
        let mut rand = GameRand::new(12345);

        for _ in 0..1_000_000 {
            println!("{}", rand.next_random());
            println!("{}", rand.next_random01());
            println!("{}", rand.rand_range(5, 1067));
            break;
        }
    */
    //let val = rand.rand_range(&(0u32..=3));

    //println!("MyEnum: {:?} {test3}", test2);
}

static vs_src2 : &str = r"
#version 330
out vec2 texCoord;
out vec3 lightVec;
out vec3 viewVec;

uniform vec4 vs_params[6];

layout(location = 0) in vec4 position;
layout(location = 1) in vec2 uv;
layout(location = 2) in vec3 mat0;
layout(location = 3) in vec3 mat1;
layout(location = 4) in vec3 mat2;

void main() {

    mat4 mvp = mat4(vs_params[0], vs_params[1], vs_params[2], vs_params[3]);
    gl_Position = mvp * position;

    texCoord = uv.xy;

    vec3 lightPos = vs_params[4].xyz;
    vec3 camPos = vs_params[5].xyz;

    vec3 lVec = lightPos - position.xyz;
    lightVec.x = dot(mat0.xyz, lVec);
    lightVec.y = dot(mat1.xyz, lVec);
    lightVec.z = dot(mat2.xyz, lVec);

    vec3 vVec = camPos - position.xyz;
    viewVec.x = dot(mat0.xyz, vVec);
    viewVec.y = dot(mat1.xyz, vVec);
    viewVec.z = dot(mat2.xyz, vVec);
}
";
    
static fs_src2: &str = r"
#version 330
uniform sampler2D Base;
uniform sampler2D Bump;

uniform vec4 fs_params[1];

in vec2 texCoord;
in vec3 lightVec;
in vec3 viewVec;

layout(location = 0) out vec4 frag_color;

void main(){

    float invRadius = fs_params[0].x;
    float ambient = fs_params[0].y;

    vec4 base = texture(Base, texCoord);
    vec3 bump = texture(Bump, texCoord).xyz * 2.0 - 1.0;

    bump = normalize(bump);

    float distSqr = dot(lightVec, lightVec);
    vec3 lVec = lightVec * inversesqrt(distSqr);

    float atten = clamp(1.0 - invRadius * sqrt(distSqr), 0.0, 1.0);
    float diffuse = clamp(dot(lVec, bump), 0.0, 1.0);

    float specular = pow(clamp(dot(reflect(normalize(-viewVec), bump), lVec), 0.0, 1.0), 16.0);
    
    frag_color = ambient * base + (diffuse * base + 0.6 * specular) * atten;
}
";
    
    
static vs_src_pfx: &str = r"
#version 330
uniform vec4 vs_params[4];
layout(location = 0) in vec4 position;
layout(location = 1) in vec2 in_uv;
layout(location = 2) in vec4 in_color;
out vec2 uv;
out vec4 color;
void main() {

    // Position it
    mat4 mvp = mat4(vs_params[0], vs_params[1], vs_params[2], vs_params[3]);
    gl_Position = mvp * position;

    uv = in_uv;
    color = in_color;
}
";
    
static fs_src_pfx : &str = r"
#version 330
layout(location = 0) out vec4 frag_color;
in vec2 uv;
in vec4 color;
uniform sampler2D tex0;
void main() {
    frag_color = texture(tex0, uv);
    frag_color *= color;
}
";
