	pub enum Direction {
		N,
		S,
		E,
		W
	}

	struct Point {
		pub x: i32,
		pub y: i32
	}

	pub struct Rover {
		pub position: Point,
		direction: Direction
	}

	impl Rover {
		pub fn new(x: i32, y: i32, d: Direction) -> Rover {
			println!("     Constructing a rover... ({0},{1}) {2}", x, y, d as i32);
			Rover {position: Point{x: x,y: y}, direction: d}
		}

		fn move(&mut self) -> () {
			match self.direction {
				N => {
					self.position.y += 1;
					println!("     Moving North...");
				},
				S => {
					self.position.y -= 1;
					println!("     Moving South...");
				},
				E => {
					self.position.x += 1;
					println!("     Moving East...");
				},
				W => {
					self.position.x -= 1;
					println!("     Moving West...");
				}
			}
			println!("     new robot position: ({0},{1}) {2}", self.position.x, self.position.y, self.direction as i32);
		}

		fn turn_left(&mut self) -> () {
			match self.direction {
				N => self.direction = W,
				S => self.direction = E,
				E => self.direction = N,
				W => self.direction = S
			}
		}

		fn turn_right(&mut self) -> () {
			match self.direction {
				N => self.direction = E,
				S => self.direction = W,
				E => self.direction = S,
				W => self.direction = N
			}
		}

		pub fn travel(&mut self, moves: &str) -> bool {
			let mut success = true;

			for c in moves.chars() {
				match c {
					'L' => self.turn_left(),
					'R' => self.turn_right(),
					'M' => self.move(),
					_ => {
						println!("Unknown command: {}", c);
						success = false;
					}
				}
			}

			success
		}

		pub fn print(&self) -> () {
			println!("     Robot position: ({0},{1}) {2}", self.position.x, self.position.y, self.direction as i32);
		}
	}
