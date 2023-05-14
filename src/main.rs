use macroquad::prelude::*;
use macroquad::window::Conf;

struct Particles {
    particles: Option<Vec<Particle>>,
}

impl Particles {
    pub fn movement(&mut self) {
        // Move all the partices in the vector
        self.particles
            .iter_mut()
            .flatten()
            .filter(|particle| particle.active)
            .for_each(|particle| particle.movement());
        self.cleanup();
    }
    pub fn draw(&mut self) {
        // Draw all the partices in the vector
        self.particles
            .iter_mut()
            .flatten()
            .filter(|particle| particle.active)
            .for_each(|particle| particle.draw());
    }
    pub fn cleanup(&mut self) {
        // Remove the inactive particles from the vector using retain
        self.particles
            .as_mut()
            .unwrap()
            .retain_mut(|particle| particle.active);
    }
}

struct Particle {
    speed: f32,
    direction: String,
    active: bool,
    halflife: f32,
    point: (f32, f32), // x,y
    sides: u8,
    radius: f32,
    rotation: f32,
    color: Color,
}

impl Particle {
    pub fn new(x: f32, y: f32, direction: &str, color: Color) -> Self {
        let speed = rand::gen_range(0.0, 40.0);
        let halflife = rand::gen_range(0.0, 5.0);
        let size = rand::gen_range(0.0, 3.0);
        Self {
            point: (x, y),
            speed,
            halflife,
            radius: size,
            rotation: 0.0,
            sides: 5,
            active: true,
            color,
            direction: direction.to_string(),
        }
    }
    pub fn movement(&mut self) {
        match self.direction.as_str() {
            "left" => {
                self.point.0 += self.speed;
            }
            "right" => {
                self.point.0 -= self.speed;
            }
            "down" => {
                self.point.1 += self.speed;
            }
            _ => {}
        }
        let decay = 1.25;
        self.halflife -= decay;
        if self.halflife < 0.0 {
            self.active = false;
        }
    }
    pub fn draw(&mut self) {
        // Only draw active particles
        if self.active {
            draw_poly(
                self.point.0,
                self.point.1,
                self.sides,
                self.radius,
                self.rotation,
                self.color,
            );
        }
    }
}

struct Lander {
    delta: (f32, f32),     // dx,dy
    thrusters: (f32, f32), // tx,ty
    size: (f32, f32),      // w,h
    point: (f32, f32),     // x,y
    color: Color,
    mass: f64, // kg
    fuel: f64, // kg
    particles: Particles,
}

impl Lander {
    pub fn movement(&mut self) {
        let color = match rand::gen_range(0, 4) {
            0 => YELLOW,
            1 => ORANGE,
            2 => RED,
            3 => WHITE,
            4 => GRAY,
            _ => BLACK,
        };
        if is_key_down(KeyCode::K) || is_key_down(KeyCode::Up) {
            let particle = Particle::new(
                self.point.0 + self.size.0 / 2.0,
                self.point.1 + self.size.1,
                "down",
                color,
            );
            self.thrusters("up");
            if self.fuel > 0.0 {
                self.particles.particles.as_mut().unwrap().push(particle);
            }
        }
        if is_key_down(KeyCode::H) || is_key_down(KeyCode::Left) {
            let particle = Particle::new(
                self.point.0 + self.size.1,
                self.point.1 + self.size.1 / 2.0,
                "left",
                color,
            );
            self.thrusters("left");
            if self.fuel > 0.0 {
                self.particles.particles.as_mut().unwrap().push(particle);
            }
        }
        if is_key_down(KeyCode::L) || is_key_down(KeyCode::Right) {
            let particle = Particle::new(
                self.point.0,
                self.point.1 + self.size.1 / 2.0,
                "right",
                color,
            );
            self.thrusters("right");
            if self.fuel > 0.0 {
                self.particles.particles.as_mut().unwrap().push(particle);
            }
        }
        if !is_key_down(KeyCode::K) && !is_key_down(KeyCode::H) && !is_key_down(KeyCode::L) {
            self.thrusters.0 = 0.0;
            self.thrusters.1 = 0.0;
        }
        self.point.0 += self.delta.0;
        self.point.1 += self.delta.1;
        self.particles.movement();
    }
    pub fn thrusters(&mut self, direction: &str) {
        match direction {
            "up" => self.up(),
            "left" => self.left(),
            "right" => self.right(),
            _ => {}
        };
        self.delta.0 += self.thrusters.0;
        self.delta.1 += self.thrusters.1;
    }
    pub fn up(&mut self) {
        if self.fuel > 0.0 {
            let rate = 1.25;
            self.fuel -= rate;
            self.thrusters.1 = -rate as f32;
        }
    }
    pub fn left(&mut self) {
        if self.fuel > 0.0 {
            let rate = 0.0075;
            self.fuel -= rate;
            self.thrusters.0 = -rate as f32;
        }
    }
    pub fn right(&mut self) {
        if self.fuel > 0.0 {
            let rate = 0.0075;
            self.fuel -= rate;
            self.thrusters.0 = rate as f32;
        }
    }
    pub fn draw(&mut self, texture: Texture2D) {
        //let (w, h) = self.size;
        let (x, y) = self.point;
        //draw_rectangle(x, y, w, h, self.color);
        draw_texture(texture, x, y, Color::new(1.0, 1.0, 1.0, 1.0));
        self.particles.draw();
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
        // Do something with moon radius?
        let b: f64 = screen_height() as f64 + self.moon.radius - self.lander.point.1 as f64;
        let d: f64 = a.powi(2) + b.powi(2);
        let d: f64 = d.sqrt();
        let f_grav: f64 = (G * m1 * m2) / d.powi(2);
        println!(
            "fuel: {}, x: {}, y: {}, d: {}, dx: {}, dy: {}, tx: {}, ty: {}",
            self.lander.fuel,
            self.lander.point.0,
            self.lander.point.1,
            d,
            self.lander.delta.0,
            self.lander.delta.1,
            self.lander.thrusters.0,
            self.lander.thrusters.1
        );
        // To make it more realistic
        let modifier = 500000000.0;
        self.lander.delta.1 = (f_grav * modifier) as f32;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "moon-rs".to_owned(),
        fullscreen: false,
        window_width: 1024,
        window_height: 768,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let lander_texture = load_texture("./assets/lander.png").await.unwrap();
    let stars_texture = load_texture("./assets/stars.png").await.unwrap();
    let mut game = Game {
        lander: Lander {
            delta: (0.0, 0.0),
            thrusters: (0.0, 0.0),
            size: (18.0, 15.0),
            point: (screen_width() / 2.0 - 60.0, 100.0),
            color: WHITE,
            mass: 15103.0,
            fuel: 10000.0,
            particles: Particles {
                particles: Some(Vec::<Particle>::new()),
            },
        },
        moon: Moon {
            mass: 7.342 * 1022.0,
            radius: 1737.4,
        },
    };
    loop {
        clear_background(BLACK);
        draw_texture(stars_texture, 0.0, 0.0, Color::new(1.0, 1.0, 1.0, 1.0));
        game.lander.movement();
        game.lander.draw(lander_texture);
        game.lunar_gravity();

        next_frame().await
    }
}
