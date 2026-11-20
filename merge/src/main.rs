use macroquad::prelude::*;

fn win_conf() -> Conf {
	Conf {
		window_title: "Flat Space".to_string(),
		fullscreen: true,
		window_resizable: false,
		..Default::default()
	}
}

const START_PART: usize = 40000;
const FAST_GATE: usize = 5041;

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
			vel: vec2(
				rand::gen_range(-50.0, 50.0),
				rand::gen_range(-50.0, 50.0),
			),
			m: 0.5,
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

		for _ in 0..START_PART {
			gal.push( Particle::new(vec2(
				rand::gen_range(0.0, screen_width()),
				rand::gen_range(0.0, screen_height()),
			)));
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
		const SOFT: f32 = 1.0;

		let mut ret: Vec<Vec2> = Vec::new();

		// if self.gal.len() > FAST_GATE {
		// 	let mut left: f32 = f32::MAX;
		// 	let mut right: f32 = f32::MIN;
		// 	let mut top: f32 = f32::MAX;
		// 	let mut bottom: f32 = f32::MIN;
		// 	for particle in &self.gal {
		// 		left = left.min(particle.pos.x);
		// 		right = right.max(particle.pos.x);
		// 		top = top.min(particle.pos.y);
		// 		bottom = bottom.max(particle.pos.y);
		// 	}

		// 	const GRID_NUM: usize = 71;

		// 	let step_x: f32 = (right - left) / GRID_NUM as f32;
		// 	let step_y: f32 = (bottom - top) / GRID_NUM as f32;

		// 	let mut r#box: [[Vec<&Particle>; GRID_NUM]; GRID_NUM] = std::array::from_fn(
		// 		|_| -> [Vec<&Particle>; GRID_NUM] {
		// 			std::array::from_fn(|_| -> Vec<&Particle> { Vec::new() })
		// 		}
		// 	);

		// 	for particle in &self.gal {
		// 		let grid: (usize, usize) = (
		// 			((particle.pos.x + left) / GRID_NUM as f32).floor() as usize,
		// 			((particle.pos.y + top) / GRID_NUM as f32).floor() as usize,
		// 		);

		// 		r#box[grid.1][grid.0].push(particle);
		// 	}

		// 	let mut grid: [[Particle; GRID_NUM]; GRID_NUM] = std::array::from_fn(|y: usize| -> [Particle; GRID_NUM] {
		// 		std::array::from_fn(|x: usize| -> Particle {
		// 			Particle {
		// 				pos: vec2((x * GRID_NUM) as f32 - left, (y * GRID_NUM) as f32 - top),
		// 				vel: Vec2::ZERO,
		// 				m: 0.0,
		// 			}
		// 		})
		// 	});

		// 	for (row_box, row_grid) in r#box.iter().zip(grid.iter_mut()) {
		// 		for (chunk_box, chunk_grid) in row_box.iter().zip(row_grid.iter_mut()) {
		// 			for particle in chunk_box {
		// 				chunk_grid.m += particle.m;
		// 				chunk_grid.pos += particle.m * particle.pos;
		// 			}

		// 			chunk_grid.pos *= 1.0;
		// 			if chunk_grid.m != 0.0 {
		// 				chunk_grid.pos /= chunk_grid.m;
		// 			}
		// 		}
		// 	}

		// 	for (box_y, row_box) in r#box.iter().enumerate() {
		// 		for (box_x, row_chunk) in row_box.iter().enumerate() {
		// 			for particle in row_chunk {
		// 				let mut tot: Vec2 = Vec2::ZERO;

		// 				for (grid_y, row_grid) in grid.iter().enumerate() {
		// 					for (grid_x, influence) in row_grid.iter().enumerate() {
		// 						if grid_x == box_x && grid_y == box_y {
		// 							continue;
		// 						}

		// 						let del_x: f32 = influence.pos.x - particle.pos.x;
		// 						let del_y: f32 = influence.pos.y - particle.pos.y;
		// 						let r: f32 = del_x * del_x + del_y * del_y + SOFT;

		// 						if r == 0.0 {
		// 							continue;
		// 						}
								
		// 						tot += G * particle.m * influence.m * (4.0 / 3.0) / r * vec2(del_x, del_y);
		// 					}
		// 				}

		// 				for influence in row_chunk {
		// 					let del_x: f32 = influence.pos.x - particle.pos.x;
		// 					let del_y: f32 = influence.pos.y - particle.pos.y;
		// 					let r: f32 = del_x * del_x + del_y * del_y + SOFT;

		// 					if r == 0.0 {
		// 						continue;
		// 					}

		// 					let f: f32 = G * particle.m * influence.m * (4.0 / 3.0);
		// 					tot += G * particle.m * influence.m * (4.0 / 3.0) / r * vec2(del_x, del_y);
		// 				}

		// 				let influence: Particle = Particle {
		// 					pos: vec2(screen_width() / 2.0, screen_height() / 2.0),
		// 					vel: Vec2::ZERO,
		// 					m: 1000.0,
		// 				};
		
		// 				let del_x: f32 = influence.pos.x - particle.pos.x;
		// 				let del_y: f32 = influence.pos.y - particle.pos.y;
		// 				let r: f32 = del_x * del_x + del_y * del_y + SOFT;
		
		// 				if r != 0.0 {
		// 					let f: f32 = G * particle.m * influence.m * (4.0 / 3.0);
		// 					tot += f / r * vec2(del_x, del_y);
		// 				}
		
		// 				ret.push(tot);
		// 			}
		// 		}
		// 	}
		// } else {
			for particle in &self.gal {
				let mut tot: Vec2 = Vec2::ZERO;

				for influence in &self.gal {
					let del_x: f32 = influence.pos.x - particle.pos.x;
					let del_y: f32 = influence.pos.y - particle.pos.y;
					let r: f32 = del_x * del_x + del_y * del_y + SOFT;

					if r == 0.0 {
						continue;
					}

					let f: f32 = G * particle.m * influence.m * (4.0 / 3.0);
					tot += G * particle.m * influence.m * (4.0 / 3.0) / r * vec2(del_x, del_y);
				}

				let influence: Particle = Particle {
					pos: vec2(screen_width() / 2.0, screen_height() / 2.0),
					vel: Vec2::ZERO,
					m: 1000.0,
				};

				let del_x: f32 = influence.pos.x - particle.pos.x;
				let del_y: f32 = influence.pos.y - particle.pos.y;
				let r: f32 = del_x * del_x + del_y * del_y + SOFT;

				if r != 0.0 {
					let f: f32 = G * particle.m * influence.m * (4.0 / 3.0);
					tot += f / r * vec2(del_x, del_y);
				}

				ret.push(tot);
			}
		// }

		ret
	}

	fn load_pos(&mut self, accel: Vec<Vec2>) {
		for (particle, accel) in self.gal.iter_mut().zip(accel) {
			particle.vel += accel.normalize();
			particle.pos += particle.vel;
		}
	}

	fn merge(&mut self) {
		let mut rm_pair: Vec<(usize, usize)> = Vec::new();
		let mut rm: Vec<usize> = Vec::new();

		for (i, part_1) in self.gal.iter().enumerate() {
			for (j, part_2) in self.gal.iter().enumerate() {
				if i >= j {
					continue;
				}

				let dist_x: f32 = (part_1.pos.x - part_2.pos.x).abs();
				let dist_y: f32 = (part_1.pos.y - part_2.pos.y).abs();

				const MERGE_STRICT: f32 = 1.0;
				let merge_dist: f32 = (part_1.m + part_2.m) * MERGE_STRICT;
				let dist_squ: f32 = dist_x * dist_x + dist_y * dist_y;

				if merge_dist * merge_dist < dist_squ || rm.contains(&i) || rm.contains(&j) {
					continue;
				}

				rm_pair.push((i, j));
				rm.append(&mut vec![i, j]);
			}
		}

		for (i, j) in rm_pair {
			let part_1: &Particle = &self.gal[i];
			let part_2: &Particle = &self.gal[j];

			let sum_m: f32 = part_1.m + part_2.m;

			self.gal.push(Particle {
				pos: vec2(
					(part_1.pos.x * part_1.m + part_2.pos.x * part_2.m) / sum_m,
					(part_1.pos.y * part_1.m + part_2.pos.y * part_2.m) / sum_m,
				),
				vel: vec2(
					(part_1.vel.x * part_1.m + part_2.vel.x * part_2.m) / sum_m,
					(part_1.vel.y * part_1.m + part_2.vel.y * part_2.m) / sum_m,
				),
				m: (part_1.m * part_1.m + part_2.m * part_2.m).sqrt(),
			});
		}

		rm.sort_unstable_by(|a: &usize, b: &usize| -> std::cmp::Ordering { b.cmp(a) });

		for i in rm {
			self.gal.swap_remove(i);
		}
	}
}

#[macroquad::main(win_conf)]
async fn main() {
	println!("sim starts");

	let mut gal: Gal = Gal::new();
	println!("gal init");

	render(&gal);

	loop {
		if is_key_down(KeyCode::Escape) {
			println!("esc detected, ending");
			break;
		}

		if is_mouse_button_pressed(MouseButton::Left) {
			gal.spawn();
		}
		gal.update();
		gal.merge();
		render(&gal);
		next_frame().await;
	}

	println!("sim ends");
}

fn render(gal: &Gal) {
	for particle in &gal.gal {
		draw_circle(particle.pos.x, particle.pos.y, particle.m, WHITE);
	}

	let gal_len: usize = gal.gal.len();
	draw_text(
		format!("{gal_len}"),
		20.0, 20.0,
		16.0,
		if gal_len > FAST_GATE {
			RED
		} else {
			GREEN
		}
	);
}
