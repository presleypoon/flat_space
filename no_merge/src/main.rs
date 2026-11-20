use macroquad::prelude::*;

fn win_conf() -> Conf {
	Conf {
		window_title: "Flat Space".to_string(),
		fullscreen: true,
		window_resizable: false,
		..Default::default()
	}
}

const MAX_PART: usize = 5000;

#[derive(Debug)]
struct Particle {
	pos: Vec2,
	vel: Vec2,
	m: f32,
}
impl Particle {
	fn new(pos: Vec2) -> Self {
		Particle {
			pos,
			// vel: vec2(rand::gen_range(-1.0, 1.0), rand::gen_range(-1.0, 1.0)),
			vel: Vec2::ZERO,
			m: 1.0,
		}
	}
}

#[derive(Debug)]
struct Gal {
	gal: Vec<Particle>,
}
impl Gal {
	fn new() -> Self {
		rand::srand(miniquad::date::now() as u64);
		let mut gal: Vec<Particle> = Vec::new();
		
		for _ in 0..MAX_PART {
			gal.push(Particle::new(vec2(
				rand::gen_range(0.0, screen_width()),
				rand::gen_range(0.0, screen_height()),
			)))
		}
		
		Gal { gal }
	}
	
	fn spawn(&mut self) {
		let m_pos: (f32, f32) = mouse_position();
		self.gal.push(Particle::new(vec2(m_pos.0, m_pos.1)))
	}
	
	fn update(&mut self) {
		self.load_pos(self.calc_pos());
	}
	
	fn calc_pos(&self) -> Vec<Vec2> {
		const G: f32 = 50.0;

		let mut ret: Vec<Vec2> = Vec::new();

		for particle in &self.gal {
			let mut tot: Vec2 = vec2(0.0, 0.0);

			for influence in &self.gal {
				let del_x: f32 = influence.pos.x - particle.pos.x;
				let del_y: f32 = influence.pos.y - particle.pos.y;
				const SOFT: f32 = 1.0;
				let r: f32 = del_x * del_x + del_y * del_y + SOFT;

				if r == 0.0 {
					continue;
				}

				let mut f: f32 = G * particle.m * influence.m * (4.0 / 3.0);
				const MAX_FORCE: f32 = 10.0;
				if f > MAX_FORCE {
					f = MAX_FORCE;
				}
				tot += f / r * vec2(del_x, del_y);
			}
			let influence: Vec2 = vec2(screen_width() / 2.0, screen_height() / 2.0);
			let del_x: f32 = influence.x - particle.pos.x;
			let del_y: f32 = influence.y - particle.pos.y;
			const SOFT: f32 = 1.0;
			let r: f32 = del_x * del_x + del_y * del_y + SOFT;

			if r == 0.0 {
				continue;
			}

			tot += G * particle.m * 10.0 * (4.0 / 3.0) / r * vec2(del_x, del_y);

			ret.push(tot);
		}

		ret
	}

	fn load_pos(&mut self, accel: Vec<Vec2>) {
		for (particle, accel) in self.gal.iter_mut().zip(accel) {
			particle.vel += accel.normalize();
			particle.pos += particle.vel;
		}
	}
}

#[macroquad::main(win_conf)]
async fn main() {
	println!("sim starts");
	
	let mut gal: Gal = Gal::new();
	println!("gal init");
	
	loop {
		if is_key_down(KeyCode::Escape) {
			println!("esc detected, ending");
			break;
		}
		
		if is_mouse_button_pressed(MouseButton::Left) {
			gal.spawn();
		}
		gal.update();
		render(&gal);
		// println!("{:?}", gal);
		next_frame().await;
	}
	
	println!("sim ends");
}

fn render(gal: &Gal) {
	for particle in &gal.gal {
		draw_circle(particle.pos.x, particle.pos.y, particle.m, WHITE);
	}
}
