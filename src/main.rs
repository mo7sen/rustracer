extern crate cgmath;
extern crate scoped_threadpool;
extern crate sdl2;

mod tracer;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::{Duration, Instant};

use cgmath::Vector3;

use std::sync::Arc;

use tracer::types::{Camera, Color, Light, Material, Scene, Surface};

use tracer::traceable::shapes;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let window = video_subsystem
        .window("rustracer", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture(
            texture_creator.default_pixel_format(),
            sdl2::render::TextureAccess::Static,
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
        )
        .unwrap();

    let surface = Arc::new(Surface::new(WINDOW_WIDTH as usize, WINDOW_HEIGHT as usize));
    let mut camera = Camera::new(Vector3::new(0.0, 0.0, 0.0), 120_f32);
    camera.set_surface(Arc::clone(&surface));

    let mut event_pump = sdl.event_pump().unwrap();
    let mut i = 0;

    let mut scene: Scene = Scene::new();

    let ivory: Material = Material::new(
        Color::RGB(
            (0.3 * 255.0) as u8,
            (0.2 * 255.0) as u8,
            (0.4 * 255.0) as u8,
        ),
        0.6,
        0.3,
        50.0,
        0.3,
        0.0,
        1.0,
    );
    let rubber: Material = Material::new(
        Color::RGB(
            (0.3 * 255.0) as u8,
            (0.1 * 255.0) as u8,
            (0.3 * 255.0) as u8,
        ),
        0.9,
        0.1,
        10.0,
        0.1,
        0.0,
        1.0,
    );
    let mirror: Material = Material::new(Color::RGB(200, 0, 150), 0.1, 0.0, 1000.0, 0.9, 0.0, 1.0);
    let glass: Material = Material::new(Color::RGB(150, 10, 50), 0.1, 0.2, 100.0, 0.2, 0.7, 1.5);

    #[rustfmt::skip]
    scene
        .add_object(Box::new(shapes::Sphere::new( Vector3::new(-1.0, -1.5, -22.0), 2.0, ivory)))
        .add_object(Box::new(shapes::Sphere::new( Vector3::new(1.5, -0.5, -25.0), 2.0, rubber)))
        // .add_object(Box::new(Sphere::new( Vector3::new(1.5, -0.5, -25.0), 2000.0, rubber)))
        .add_object(Box::new(shapes::Sphere::new( Vector3::new(-6.0, 4.5, -30.0), 4.0, mirror)))
        .add_object(Box::new(shapes::Sphere::new( Vector3::new(2.0, 0.0, -15.0), 1.0, glass)))
        .add_object(Box::new(shapes::Disk::new(  Vector3::new(0.0, 15.0, -30.0), Vector3::new(0.0, -1.0, 1.0), 10.0, mirror)))
        // .add_object(Box::new(Plane::new(  Vector3::new(0.0, 15.0, -100.0), Vector3::new(0.0, -1.0, 1.0), mirror)))

        .add_object(Box::new(shapes::Cube::new( Vector3::new(10.0, -5.0, -9.0), Vector3::new(5.0, 5.0, -24.0), glass)))

        .add_light(Box::new(Light::new(Vector3::new(30.0, 50.0, -25.0), 1.8)))
        .add_light(Box::new(Light::new(Vector3::new(-20.0, 20.0, 20.0), 1.5)))
        .add_light(Box::new(Light::new(Vector3::new(30.0, 20.0, 30.0), 1.7)))
        ;

    'windowpoll: loop {
        let start: Instant = Instant::now();
        i = (i + 1) % 255;
        canvas.set_draw_color(sdl2::pixels::Color::RGB(i, 64, 255 - i));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'windowpoll,
                _ => {}
            }
        }

        camera.render_scene(&scene).expect("Couldn't render scene");

        texture
            .update(
                None,
                surface.pixels.lock().unwrap().pixels.as_slice(),
                WINDOW_WIDTH as usize * 4,
            )
            .unwrap();
        canvas.copy(&texture, None, None).unwrap();

        canvas.present();
        let duration: Duration = start.elapsed();
        println!("FPS: {}", 1.0 / duration.as_secs_f32());
    }
}
