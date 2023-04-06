//#![windows_subsystem = "windows"]

mod base_app;
mod game_rand;
mod model;
mod particle_system;
mod sapp;
mod timer;
mod vector;

use base_app::*;
use game_rand::GameRand;
use model::*;
use particle_system::*;
use sapp::*;
use timer::*;
use vector::*;

struct App {
    timer : Timer,
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
    //desc.win32_console_utf8 = true;
    //desc.win32_console_create = true;
    //desc.win32_console_attach = true;
    //desc.fullscreen = true;

    let App = App {
        timer : Timer::new()
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
