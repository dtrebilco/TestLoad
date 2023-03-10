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

    app : dyn App,
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

/*/
  ///  To be implemented by derived class
  static BaseApp* CreateApp();

  BaseApp();
  virtual ~BaseApp();

  virtual void ResetCamera();
  virtual bool OnEvent(const sapp_event* ev);

  virtual bool Load();
  virtual void DrawFrame() = 0;

  void Controls();
*/
