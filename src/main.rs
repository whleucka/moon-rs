use macroquad::prelude::*;

struct Lander {
    dy: f64,
    size: (f32, f32),
    point: (f32, f32),
    color: Color,
    mass: f64,
}

impl Lander {
    pub fn movement(&mut self) {
        self.point.1 += self.dy as f32;
    }
    pub fn thrusters(&mut self) {
        let (x, y) = self.point;
    }
    pub fn draw(&mut self) {
        let (w, h) = self.size;
        let (x, y) = self.point;
        draw_rectangle(x, y, w, h, self.color);
    }
}

struct Moon {
    mass: f64,
    radius: f64,
}

struct Game {
    lander: Lander,
    moon: Moon,
}

impl Game {
    pub fn lunar_gravity(&mut self) {
        // f_grav = (G*m1*m2)/d^2
        // f_grav = force due to gravity
        // G = universal gravitation constant
        // m1 = mass of the 1st object
        // m2 = mass of the 2nd object
        // d = distance between the centers of two objects
        const G: f64 = 6.673e-11;
        let m1: f64 = self.lander.mass;
        let m2: f64 = self.moon.mass;
        let a: f64 = screen_width() as f64 / 2.0 - self.lander.point.0 as f64;
        let b: f64 = screen_height() as f64 - self.lander.point.1 as f64;
        let d: f64 = a.powi(2) + b.powi(2);
        let d: f64 = d.sqrt();
        let f_grav: f64 = (G * m1 * m2) / d.powi(2);
        println!(
            "x: {}, y: {}, d: {}, dy: {}",
            self.lander.point.0, self.lander.point.1, d, self.lander.dy
        );
        // To make it more realistic
        let modifier = 100000000.0;
        self.lander.dy = f_grav * modifier;
    }
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut game = Game {
        lander: Lander {
            dy: 0.0,
            size: (20.0, 20.0),
            point: (screen_width() / 2.0 - 60.0, 100.0),
            color: WHITE,
            mass: 15103.0, // kg
        },
        moon: Moon {
            mass: 7.342 * 1022.0, // kg
            radius: 1737.4,       // km
        },
    };
    loop {
        clear_background(BLACK);

        game.lander.movement();
        game.lander.thrusters();
        game.lander.draw();
        game.lunar_gravity();

        next_frame().await
    }
}

