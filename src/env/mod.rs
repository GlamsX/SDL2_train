use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::space::Object3d;

pub struct Env {
    sdl_context: sdl2::Sdl,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

impl Env {
  pub fn new(title: &str, width: u32, height: u32) -> Self {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window(title, width, height)
      .position_centered()
      .build()
      .unwrap();
    let canvas = window.into_canvas().build().unwrap();

    Env {
      sdl_context,
      canvas,
    }
  }

  pub fn run(&mut self) {
    let mut event_pump = self.sdl_context.event_pump().unwrap();
    let mut cube = Object3d::create_cube();
    
    'running: loop {
      for event in event_pump.poll_iter() {
        match event {
          Event::Quit {..} |
          Event::KeyDown { 
            keycode: Some(Keycode::Escape), .. 
          } => {
              break 'running
          },
          Event::KeyDown { 
            keycode: Some(Keycode::Y), .. 
          } => {
            for face in &mut cube.faces {
              face.p1.rotate_y(0.1);
              face.p2.rotate_y(0.1);
              face.p3.rotate_y(0.1);
            }
          },
          Event::KeyDown { 
            keycode: Some(Keycode::X), .. 
          } => {
            for face in &mut cube.faces {
              face.p1.rotate_x(0.1);
              face.p2.rotate_x(0.1);
              face.p3.rotate_x(0.1);
            }
          },
          Event::KeyDown { 
            keycode: Some(Keycode::Z), .. 
          } => {
            for face in &mut cube.faces {
              face.p1.rotate_z(0.1);
              face.p2.rotate_z(0.1);
              face.p3.rotate_z(0.1);
            }
          },
          _ => {}
        }
      }
      self.canvas.set_draw_color(Color::RGB(0, 0, 0));
      self.canvas.clear();

      // Draw here

      self.canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255)); // white color
      for face in &cube.faces {
        face.draw(&mut self.canvas, 800.0, 600.0);
      }

      self.canvas.present();
      std::thread::sleep(std::time::Duration::from_millis(16)); // roughly 60 FPS

    }
  }
}