extern crate graphics;
extern crate piston;
extern crate piston_window;

mod lib;

use lib::*;
use piston::window::WindowSettings;
use piston_window::*;
use std::env;

fn main() {
    // Command line args
    let args: Vec<String> = env::args().collect();
    let window_x: u32 = args[1].parse().unwrap();
    let window_y: u32 = args[2].parse().unwrap();
    let iterations: u32 = args[3].parse().unwrap();
    let opacity: f32 = args[4].parse().unwrap();

    // Create window
    let mut window: PistonWindow = WindowSettings::new("MLH", [window_x, window_y + 30]).exit_on_esc(true).build().unwrap();   
    let slot_bar_width = window_x as f64 * 0.6;
    let slot_width = slot_bar_width / 10.0;
    let jump_bar_width = window_x as f64 * 0.2;

    // The 10 fractals we can use
    let mut fractals: Vec<Fractal> = Vec::with_capacity(10);

    for _ in 0..10 {
        fractals.push(Fractal::blank())
    }

    // Tracking variables
    let mut shape_index = 0;
    let mut cursor = Point(0.0, 0.0); 

    while let Some(e) = window.next() {
        // Change the selected shape
        let mut shape = &mut fractals[shape_index];

        // Update the known cursor position
        if let Some(a) = get_cursor(&e) { cursor = a }

        // Mouse controls
        if let Some(Button::Mouse(button)) = e.press_args() {
            match button {
                MouseButton::Left => { 
                    if cursor.1 > 30.0 {
                        shape.add_vertex(cursor) 
                    } else {
                        for i in 0..=10 {
                            let slot_min = slot_width as f32 * (i as f32 - 1.0) + 5.0;
                            let slot_max = slot_width as f32 * i as f32 + 5.0;

                            if cursor.0 >= slot_min  && cursor.0 < slot_max && i != 0 {
                                shape_index = i - 1;
                            }
                        }

                        let jump_min = window_x as f32 - jump_bar_width as f32;
                        let jump_max = window_x as f32;

                        if cursor.0 >= jump_min && cursor.0 < jump_max  {
                            shape.jump_size = (cursor.0 + 5.0 - jump_min) / (jump_max - jump_min);
                            shape.jump_size = round_to(shape.jump_size, 2.0);
                            shape.constrain_jump();
                        }
                    }   
                },
                MouseButton::Right => { shape.filter_dist(cursor, 25.0) }
                _ => { },
            }
        }

        // Keyboard controls
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::D1     => { shape_index = 0 }
                Key::D2     => { shape_index = 1 }
                Key::D3     => { shape_index = 2 }
                Key::D4     => { shape_index = 3 }
                Key::D5     => { shape_index = 4 }
                Key::D6     => { shape_index = 5 }
                Key::D7     => { shape_index = 6 }
                Key::D8     => { shape_index = 7 }
                Key::D9     => { shape_index = 8 }
                Key::D0     => { shape_index = 9 }
                Key::Return => { shape.gen_points(iterations) },
                Key::Equals => { shape.add_jump(0.01) },
                Key::Minus  => { shape.add_jump(-0.01) },
                Key::Space  => { shape.set_optimal_jump() },
                Key::C      => { fractals[shape_index] = Fractal::blank() },
                Key::Comma  => { shape.swap_render_vertices() }
                Key::Period => { shape.swap_render_points() }
                _ => { },
            }
        };

        window.draw_2d(&e, |b, g, _| {
            clear([0.1, 0.1, 0.1, 1.0], g);

            // Bar background
            rectangle(
                [0.2, 0.2, 0.2, 1.0],
                math::margin_rectangle([0.0, 0.0, window_x as f64, 30.0], 0.0),
                b.transform,
                g
            );

            // Jump bar background
            rectangle(
                [0.3, 0.3, 0.3, 1.0],
                math::margin_rectangle([window_x as f64 - jump_bar_width - 5.0, 5.0, jump_bar_width, 20.0], 0.0),
                b.transform,
                g
            );

            // Jump bar
            rectangle(
                [0.4, 0.4, 0.4, 1.0],
                math::margin_rectangle([window_x as f64 - jump_bar_width - 5.0, 5.0, (fractals[shape_index].jump_size * jump_bar_width as f32) as f64, 20.0], 0.0),
                b.transform,
                g
            );

            // Slot bar background
            rectangle(
                [0.3, 0.3, 0.3, 1.0],
                math::margin_rectangle([5.0, 5.0, slot_bar_width, 20.0], 0.0),
                b.transform,
                g
            );

            // Go through all 10 fractals, according to their rendering settings, draw their respective points, vertices, and slots
            for (i, frac) in fractals.iter().enumerate() {
                let colour = RGB::from_hsv(&HSV(36.0 * i as f32, 1.0, 1.0)).to_arr(opacity);

                if frac.render_points {
                    for spot in &frac.path {
                        rectangle(
                            colour,
                            rectangle::square(spot.0.0 as f64, spot.0.1 as f64, 1.0),
                            b.transform,
                            g
                        );
                    }
                }

                if frac.render_vertices {
                    for vertex in &frac.vertices {
                        rectangle(
                            colour,
                            rectangle::square(vertex.0 as f64 - 5.0, vertex.1 as f64 - 5.0, 10.0),
                            b.transform,
                            g
                        );
                    }
                }

                // Slot bar
                rectangle(
                    RGB::from_hsv(&HSV(36.0 * i as f32, 1.0, 1.0)).to_arr(if i == shape_index { 0.7 } else { 0.1 }),
                    math::margin_rectangle([slot_bar_width / 10.0 * i as f64 + 5.0, 5.0, slot_bar_width / 10.0, 20.0], 0.0),
                    b.transform,
                    g
                );
            }
        });
    }
}
