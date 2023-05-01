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
        false
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
