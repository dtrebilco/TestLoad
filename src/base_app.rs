use crate::vector::*;

pub struct BaseApp {
    pub app_time: f32,
    pub frame_time: f32,
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

    app: dyn App,
}

pub trait App {
    fn reset_camera(&mut self, app: &mut BaseApp) {}

    fn on_event(&mut self, app: &mut BaseApp) -> bool {
        false
    }

    fn load(&mut self, app: &mut BaseApp) -> bool {
        false
    }

    fn draw_frame(&mut self, app: &mut BaseApp) {}
}

impl BaseApp {
    fn controls(&mut self) {
        // Compute directional vectors from euler angles
        let cosX = self.wx.cos();
        let sinX = self.wx.sin();
        let cosY = self.wy.cos();
        let sinY = self.wy.sin();

        let dx = vec3(cosY, 0.0, sinY);
        let dy = vec3(-sinX * sinY, cosX, sinX * cosY);
        let dz = vec3(-cosX * sinY, -sinX, cosX * cosY);

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

        let lenSq = length_squared(dir);
        if lenSq > 0.0 {
            dir *= 1.0 / lenSq.sqrt();
            let speed = 1000.0f32; // DT_TODO:
            self.cam_pos += dir * (self.frame_time * speed);
        }
    }
}

/*

bool BaseApp::OnEvent(const sapp_event* ev) {

  switch (ev->type) {
  case SAPP_EVENTTYPE_MOUSE_DOWN:
    if (ev->mouse_button == SAPP_MOUSEBUTTON_LEFT) {
      sapp_lock_mouse(true);
    }
    break;
  case SAPP_EVENTTYPE_MOUSE_UP:
    if (ev->mouse_button == SAPP_MOUSEBUTTON_LEFT) {
      sapp_lock_mouse(false);
    }
    break;
  case SAPP_EVENTTYPE_MOUSE_SCROLL:
    //cam_zoom(cam, ev->scroll_y * 0.5f); //DT_TODO: Adjust speed here?
    break;
  case SAPP_EVENTTYPE_MOUSE_MOVE:
    if (sapp_mouse_locked()) {
      float mouseSensibility = 0.003f;
      wx -= mouseSensibility * ev->mouse_dy;
      wy -= mouseSensibility * ev->mouse_dx;
    }
    break;

  case SAPP_EVENTTYPE_KEY_DOWN:
    if (ev->key_code == SAPP_KEYCODE_ESCAPE)
    {
      sapp_request_quit();
    }
    if (ev->key_code == SAPP_KEYCODE_ENTER)
    {
      ResetCamera();
    }
    break;
  default:
    break;
  }

  if (ev->type == SAPP_EVENTTYPE_KEY_DOWN ||
      ev->type == SAPP_EVENTTYPE_KEY_UP) {
    if (!ev->key_repeat) {
      bool pressed = (ev->type == SAPP_EVENTTYPE_KEY_DOWN);

      switch (ev->key_code) {
      case SAPP_KEYCODE_W: key_forwardKey  = pressed; break;
      case SAPP_KEYCODE_S: key_backwardKey = pressed; break;
      case SAPP_KEYCODE_A: key_leftKey     = pressed; break;
      case SAPP_KEYCODE_D: key_rightKey    = pressed; break;
      }
    }
  }

  return false;
}

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
