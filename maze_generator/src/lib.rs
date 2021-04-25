use std::vec;
use rand::seq::SliceRandom;

#[derive(PartialEq, Clone)]
pub enum MazePixel {
	Wall,
	Space,
	Target,
	Player,
}

#[derive(PartialEq, Clone)]
pub enum Direction {
	Up,
	Down,
	Left,
	Right,
}

#[derive(Clone)]
pub struct Maze {
	size: (u32, u32),
	pixels: Vec<Vec<MazePixel>>,
	pub player: (u32, u32),
	pub target: (u32, u32),
}

#[derive(Clone)]
pub struct Charset {
	pub wall: String,
	pub air: String,
	pub target: String,
	pub player: String,
}

impl Maze {
	pub fn new(_size: u32) -> Self {
		let size = (_size, _size);
		
		let mut maze = Self {
			size: (size.0 * 2 + 1, size.1 * 2 + 1),
			pixels: vec![],
			player: (1, 1),
			target: (0, 0),
		};

		for i in 0..maze.size.1 {
			maze.pixels.push(vec![MazePixel::Wall; maze.size.0 as usize]);

			for j in 0..maze.size.0 {
				if i % 2 == 1 && j % 2 == 1 {
					maze.pixels[i as usize][j as usize] = MazePixel::Space;
				}
			}
		}

		maze.pixels[1][1] = MazePixel::Player;

		let mut stack = Vec::<(u32, u32)>::new();
		let mut carved = vec![vec![false; size.0 as usize]; size.1 as usize];
		let mut top = (0, (0, 0));

		let mut current = (0, 0);
		let mut init = true;
		let mut rng = rand::thread_rng();

		while stack.len() > 0 || init {
			init = false;

			let mut is_carved = true;
			let mut options = Vec::<((u32, u32), (u32, u32))>::new();

			if
				current.0 > 0 &&
				current.0 - 1 < size.0 &&
				!carved[current.1 as usize][(current.0 - 1) as usize] 
			{
				options.push((
					(current.0 - 1, current.1),
					((current.0 * 2), (current.1 * 2) + 1)
				));
			}
			if
				current.1 > 0 &&
				current.1 - 1 < size.1 &&
				!carved[(current.1 - 1) as usize][current.0 as usize]
			{
				options.push((
					(current.0, current.1 - 1),
					((current.0 * 2) + 1, (current.1 * 2))
				));
			}
			if
				current.0 + 1 < size.0 &&
				!carved[current.1 as usize][(current.0 + 1) as usize]
			{
				options.push((
					(current.0 + 1, current.1),
					((current.0 * 2) + 2, (current.1 * 2) + 1)
				));
			}
			if
				current.1 + 1 < size.1 &&
				!carved[(current.1 + 1) as usize][current.0 as usize]
			{
				options.push((
					(current.0, current.1 + 1),
					((current.0 * 2) + 1, (current.1 * 2) + 2)
				));
			}
			
			if options.len() == 0 {
				current = stack.pop().unwrap();
				is_carved = false;
			}

			if is_carved {
				let opt = options.choose(&mut rng).unwrap();
				current = opt.0;
				maze.pixels[(opt.1).1 as usize][(opt.1).0 as usize] = MazePixel::Space;

				stack.push(current);
				carved[current.1 as usize][current.0 as usize] = true;
			}

			if stack.len() > top.0 {
				top = (stack.len(), current);
			}
		}

		maze.target = (((top.1).0 * 2) + 1, ((top.1).1 * 2) + 1);
		maze.pixels[maze.target.1 as usize][maze.target.0 as usize] = MazePixel::Target;

		maze
	}

	pub fn player_move(&mut self, direction: Direction) -> bool {
		let will_overflow = match direction {
			Direction::Up => self.player.1 <= 1,
			Direction::Left => self.player.0 <= 1,
			_ => false,
		};

		if will_overflow {
			return false;
		}

		let new_pos = match direction {
			Direction::Up => (self.player.0, self.player.1 - 1),
			Direction::Down => (self.player.0, self.player.1 + 1),
			Direction::Left => (self.player.0 - 1, self.player.1),
			Direction::Right => (self.player.0 + 1, self.player.1),
		};

		if
			new_pos.0 >= self.size.0 ||
			new_pos.1 >= self.size.1
		{
			return false;
		}

		let target_cell = &self.pixels[new_pos.0 as usize][new_pos.1 as usize];
		if target_cell == &MazePixel::Wall {
			return false;
		}

		self.pixels[self.player.0 as usize][self.player.1 as usize] = MazePixel::Space;
		self.pixels[new_pos.0 as usize][new_pos.1 as usize] = MazePixel::Player;
		self.player = new_pos;
		
		true
	}

	pub fn as_string(self, charset: Charset) -> String {
		let wall_char = charset.wall.as_str();
		let space_char = charset.air.as_str();
		let target_char = charset.target.as_str();
		let player_char = charset.player.as_str();

		let mut out = String::from("");
		
		let apixels: Vec<Vec<MazePixel>> = self.as_vec();

		for row in apixels {
			for pixel in row {
				out.push_str(match pixel {
					MazePixel::Wall => wall_char,
					MazePixel::Space => space_char,
					MazePixel::Target => target_char,
					MazePixel::Player => player_char,
				});
			}

			out.push('\n');
		}

		out
	}

	pub fn as_vec(self) -> Vec<Vec<MazePixel>> {
		let mut apixels: Vec<Vec<MazePixel>> = vec![vec![]; self.size.1 as usize];
		for col in self.pixels {
			let mut row: usize = 0;
			for pixel in col {
				apixels.get_mut(row).unwrap().push(pixel);

				row += 1;
			}
		}

		apixels
	}
}
