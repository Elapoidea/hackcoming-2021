extern crate rand;
extern crate piston_window;

use self::rand::Rng;
use piston_window::*;

#[derive(Copy, Clone)]
pub struct Point(pub f32, pub f32);

impl Point {
    // Converts an array of 2 elements into a point
    pub fn from_arr(arr: [f64; 2]) -> Self {
        Point(arr[0] as f32, arr[1] as f32)
    }

    // Gets the distance between two points
    pub fn dist(&self, p: Point) -> f32 {
        ((p.0 - self.0).powf(2.0) + (p.1 - self.1).powf(2.0)).sqrt()
    }

    // Moves somewhere inbetween two points. How close you are to the second point is controlled by `r`
    pub fn jump(&mut self, p: Point, r: f32) {
        let dx = p.0 - self.0;
        let dy = p.1 - self.1;

        self.0 = self.0 + dx * r;
        self.1 = self.1 + dy * r;
    }
}

#[derive(Clone)]
pub struct Fractal {
    pub vertices: Vec<Point>,
    pub path: Vec<(Point, Point)>,
    pub jump_size: f32,
    pub render_vertices: bool,
    pub render_points: bool,
}

impl Fractal {
    // Makes a blank fractal
    pub fn blank() -> Self {
        Fractal {
            vertices: vec![],
            path: vec![],
            jump_size: 0.5,
            render_vertices: true,
            render_points: true,
        }
    }

    // Adds a vertex to the fractal shape
    pub fn add_vertex(&mut self, p: Point) {
        self.vertices.push(p);
    }

    // Calculates the jump size that would generate the best looking fractals, most of the time
    pub fn set_optimal_jump(&mut self) {
        let n = self.vertices.len() as f32;
        self.jump_size = n / (n + 3.0);
    }

    // Increases or decreases the jump size
    pub fn add_jump(&mut self, change: f32) {
        self.jump_size = round_to(self.jump_size + change, 2.0);
        self.constrain_jump();
    }

    // Constrains the jump to a reasonable amount
    pub fn constrain_jump(&mut self) {
        if self.jump_size < 0.0 { self.jump_size = 0.0 }
        if self.jump_size > 1.0 { self.jump_size = 1.0 }
    }

    // Generates `iters` points in the fractal
    pub fn gen_points(&mut self, iters: u32) {
        self.path = vec![];

        let mut rng = rand::thread_rng();
        let mut tracer = Point(500.0, 500.0);
    
        for _ in 0..iters {
            let target = &self.vertices[rng.gen_range(0..self.vertices.len())];
            tracer.jump(*target, self.jump_size);
            self.path.push((tracer, *target));
        }
    }

    // Both of these invert their respective bool
    // True  -> False
    // False -> True
    pub fn swap_render_vertices(&mut self) {
        self.render_vertices = !self.render_vertices;
    }

    pub fn swap_render_points(&mut self) {
        self.render_points = !self.render_points;
    }

    // Removes all vertices that are `d` or less away from a point, `p`
    pub fn filter_dist(&mut self, p: Point, d: f32) {
        let mut new = vec![];
        
        self.vertices.iter().filter(|x| p.dist(**x) > d).for_each(|x| new.push(*x));

        self.vertices = new;
    }
}

#[derive(Copy, Clone)]
pub struct RGB(pub f32, pub f32, pub f32);

impl RGB {
    // Converts a HSV value into RGB
    pub fn from_hsv(hsv: &HSV) -> Self {
        let h = hsv.0;
        let c = hsv.1 * hsv.2;
        let x = c * (1.0 - ((h/60.0) % 2.0 - 1.0).abs());
        let m = hsv.2 - c;

        if h < 60.0 { RGB(c, x, 0.0) }
        else if h < 120.0 { RGB(x, c, 0.0) }
        else if h < 180.0 { RGB(0.0, c, x) }
        else if h < 240.0 { RGB(0.0, x, c) }
        else if h < 300.0 { RGB(x, 0.0, c) }
        else if h < 360.0 { RGB(c, 0.0, x) }
        else { RGB(0.0, 0.0, 0.0) }.add(m)
    }

    // Adds a value to R, G, and B
    pub fn add(self, x: f32) -> Self {
        RGB(self.0 + x, self.1 + x, self.2 + x)
    }

    // Converts the RGB value into an array with 4 elements.
    pub fn to_arr(self, opacity: f32) -> [f32; 4] {
        [self.0, self.1, self.2, opacity]
    }
}

pub struct HSV(pub f32, pub f32, pub f32);

// Rounds a float, `x` to `p` decimal places
pub fn round_to(x: f32, p: f32) -> f32 {
    (x * 10.0_f32.powf(p)).round() / 10.0_f32.powf(p)
}

// Gets the position of the cursor on the screen as a point
pub fn get_cursor(e: &Event) -> Option<Point> {
    e.mouse_cursor(|pos| { Point::from_arr(pos) })
}




