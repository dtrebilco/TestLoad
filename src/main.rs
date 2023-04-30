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

  /*
  struct Portal {
    inline Portal(uint32_t sect, const vec3& vc0, const vec3& vc1, const vec3& vc2) {
      sector = sect;
      v[0] = vc0;
      v[1] = vc1;
      v[2] = vc1 + vc2 - vc0;
      v[3] = vc2;
    }
  
    vec3 v[4];
    uint32_t sector = 0;
  };
  
  class Sector {
  public:
  
    inline bool isInBoundingBox(vec3& pos) const {
      return (pos.x > min.x && pos.x < max.x&&
        pos.y > min.y && pos.y < max.y&&
        pos.z > min.z && pos.z < max.z);
    }
  
    inline bool isSphereInSector(const vec3& pos, const float radius) const {
      return (getDistanceSqr(pos) < radius * radius);
    }
  
    inline float getDistanceSqr(const vec3& pos) const {
      float s, d = 0;
      for (int i = 0; i < 3; i++) {
        if (pos[i] < min[i]) {
          s = pos[i] - min[i];
          d += s * s;
        }
        else if (pos[i] > max[i]) {
          s = pos[i] - max[i];
          d += s * s;
        }
      }
      return d;
    }
  
  
    Model room;
    std::vector<Portal> portals;
    std::vector<Light> lights;
  
    vec3 min, max;
    bool hasBeenDrawn = false;
  };
  
  class App : public BaseApp
  {
  public:
  
    void ResetCamera() override;
    bool Load() override;
    void DrawFrame() override;
  
  protected:
  
    Sector sectors[5];
  
    sg_shader shader = {};
    sg_image base[3] = {};
    sg_image bump[3] = {};
    sg_pipeline room_pipline = {};
    sg_pipeline room_pipline_blend = {};
  
    sg_shader pfx_shader = {};
    sg_image pfx_particle = {};
    sg_pipeline pfx_pipline = {};
  
    sg_buffer pfx_index = {};
    sg_buffer pfx_vertex = {};
  
  };
*/

struct App {
    timer: Timer,
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
