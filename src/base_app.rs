use crate::vector::*;

struct BaseApp {
    app_time: f32,
    frame_time: f32,
    start_ticks: u64,
    time_ticks: u64,

    cam_pos: vec3,
    wx: f32,
    wy: f32,
    wz: f32,

    key_left: bool,
    key_right: bool,
    key_down: bool,
    key_up: bool,
    key_backward: bool,
    key_forward: bool,
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
