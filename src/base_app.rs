use crate::sapp::*;
use crate::sgfx::*;
use crate::timer::Timer;
use crate::vector::*;

pub struct BaseData {
    pub sg : sg_state_t,

    pub app_time: f32,
    pub frame_time: f32,

    pub timer: Timer,
    start_ticks: u64,
    time_ticks: u64,

    pub cam_pos: vec3,
    pub wx: f32,
    pub wy: f32,
    pub wz: f32,

    pub key_left: bool,
    pub key_right: bool,
    pub key_down: bool,
    pub key_up: bool,
    pub key_backward: bool,
    pub key_forward: bool,
}

pub trait AppI {
    fn init(&mut self, _app: &mut BaseData, _sapp: &mut SAppData) {}

    fn reset_camera(&mut self, _app: &mut BaseData, _sapp: &mut SAppData) {}

    fn on_event(&mut self, _app: &mut BaseData, _sapp: &mut SAppData, _event: &Event) -> bool {
        false
    }

    fn load(&mut self, _app: &mut BaseData, _sapp: &mut SAppData) -> bool {
        false
    }

    fn draw_frame(&mut self, _app: &mut BaseData, _sapp: &mut SAppData) {}
}

impl BaseData {
    fn new() -> BaseData {
        BaseData {
            sg : sg_state_t::default(),

            app_time: 0.0,
            frame_time: 0.33,

            timer: Timer::new(),
            start_ticks: 0,
            time_ticks: 0,

            cam_pos: vec3(0.0, 0.0, 0.0),
            wx: 0.0,
            wy: 0.0,
            wz: 0.0,

            key_left: false,
            key_right: false,
            key_down: false,
            key_up: false,
            key_backward: false,
            key_forward: false,
        }
    }

    fn controls(&mut self) {
        // Compute directional vectors from euler angles
        let cos_x = self.wx.cos();
        let sin_x = self.wx.sin();
        let cos_y = self.wy.cos();
        let sin_y = self.wy.sin();

        let dx = vec3(cos_y, 0.0, sin_y);
        let dy = vec3(-sin_x * sin_y, cos_x, sin_x * cos_y);
        let dz = vec3(-cos_x * sin_y, -sin_x, cos_x * cos_y);

        let mut dir = vec3(0.0, 0.0, 0.0);
        if self.key_left {
            dir -= dx;
        }
        if self.key_right {
            dir += dx;
        }
        if self.key_down {
            dir -= dy;
        }
        if self.key_up {
            dir += dy;
        }
        if self.key_backward {
            dir -= dz;
        }
        if self.key_forward {
            dir += dz;
        }

        let len_sq = length_squared(&dir);
        if len_sq > 0.0 {
            dir *= 1.0 / len_sq.sqrt();
            let speed = 1000.0f32; // DT_TODO:
            self.cam_pos += dir * (self.frame_time * speed);
        }
    }
}

pub struct BaseApp<T>
where
    T: AppI,
{
    base: BaseData,
    app: T,
}

pub fn run_app<T>(app: T, desc: &SAppDesc)
where
    T: AppI,
{
    let mut b = BaseApp {
        base: BaseData::new(),
        app,
    };

    crate::sapp::run_app(&mut b, &desc);
}

impl<T> SAppI for BaseApp<T>
where
    T: AppI,
{
    fn init(&mut self, data: &mut SAppData) {
        self.app.init(&mut self.base, data);

        self.base.start_ticks = self.base.timer.now(); // DT_TODO: Move this to start and report startup time?
        let desc = sg_desc {
            //color_format = (sg_pixel_format) data.color_format();
            //depth_format = (sg_pixel_format) data.depth_format();
            //sample_count = data.sample_count(),
            ..sg_desc::default()
        };
        sg_setup(&mut self.base.sg, &desc);

        //DT_TODO: Load UI assets
        self.app.load(&mut self.base, data);
        self.app.reset_camera(&mut self.base, data);
    }

    fn on_event(&mut self, sapp: &mut SAppData, event: &Event) {
        if self.app.on_event(&mut self.base, sapp, &event) {
            return;
        }

        match event {
            Event::Mouse(data) => {
                if data.mouse_button == MouseButton::Left {
                    sapp.lock_mouse(data.pressed);
                }
            }
            Event::MouseScroll(_data) => {
                //cam_zoom(cam, ev->scroll_y * 0.5f); //DT_TODO: Adjust speed here?
            }
            Event::MouseMove(data) => {
                if sapp.mouse_locked() {
                    let mouse_sensibility = 0.003f32;
                    self.base.wx -= mouse_sensibility * data.mouse_dy;
                    self.base.wy -= mouse_sensibility * data.mouse_dx;
                }
            }
            Event::Char(data) => {
                let a = data.char_code;
            }

            Event::Key(data) => {
                if data.pressed {
                    if data.key_code == KeyCode::Escape {
                        sapp.request_quit();
                    }
                    if data.key_code == KeyCode::Enter {
                        self.app.reset_camera(&mut self.base, sapp);
                    }
                }
                if !data.key_repeat {
                    match data.key_code {
                        KeyCode::W => self.base.key_forward = data.pressed,
                        KeyCode::S => self.base.key_backward = data.pressed,
                        KeyCode::A => self.base.key_left = data.pressed,
                        KeyCode::D => self.base.key_right = data.pressed,
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }

    fn draw_frame(&mut self, data: &mut SAppData) {
        // Update delta time
        self.base.frame_time =
            Timer::sec(self.base.timer.laptime(&mut self.base.time_ticks)) as f32;
        self.base.app_time =
            Timer::sec(Timer::diff(self.base.time_ticks, self.base.start_ticks)) as f32;

        self.base.controls();
        self.app.draw_frame(&mut self.base, data);

        sg_commit();
    }

    fn shutdown(&mut self, data: &mut SAppData) {
        sg_shutdown();
    }
}

/*

static void init_userdata_cb(void* in_app) {
  BaseApp* app = (BaseApp*)in_app;

  sg_setup(sg_desc{ .context = sapp_sgcontext() });
  stm_setup();
  app->start_ticks = stm_now(); // DT_TODO: Move this to start and report startup time?

  //DT_TODO: Load UI assets
  app->Load();
  app->ResetCamera();
}

static void frame_userdata_cb(void* in_app) {
  BaseApp* app = (BaseApp*)in_app;

  // Update delta time
  app->frame_time = (float)stm_sec(stm_laptime(&app->time_ticks));
  app->app_time   = (float)stm_sec(stm_diff(app->time_ticks, app->start_ticks));

  app->Controls();
  app->DrawFrame();

  //DT_TODO: Draw UI

  sg_commit();
}

sapp_desc sokol_main(int argc, char* argv[]) {

  // Create App
  BaseApp* app = BaseApp::CreateApp();

  return sapp_desc{
      .user_data = app,
      .init_userdata_cb = init_userdata_cb,
      .frame_userdata_cb = frame_userdata_cb,
      .cleanup_userdata_cb = cleanup_userdata_cb,
      .event_userdata_cb = event_userdata_cb,
      .width = 800,  // DT_TODO: Get params from the app
      .height = 600,
      .sample_count = 4,
      .window_title = "Portals",
  };
}




 */
