pub mod editor {	
	
	use std::collections::HashMap;
	
	pub struct CharMap {
		cmap: HashMap<char, char>,
	}
	
	impl Default for CharMap {
		pub fn default() -> CharMap {
			let char_map: HashMap<char, char> = HashMap::from([
				('a', '@'),
				('b', '8'),
				('c', '('),
				('d', '\u{1d15E}'),
				('e', '3'),
				('f', '\u{20A3}'),
				('g', '9'),
				('h', '\u{210F}'),
				('i', '|'),
				('j', ';'),
				('k', '\u{03BA}'),
				('l', '1'),
				('m', '\u{2AD9}'),
				('n', '\u{2229}'),
				('o', '0'),
				('p', '\u{2690}'),
				('q', '\u{1D16F}'),
				('r', '\u{0393}'),
				('s', '\u{222B}'),
				('t', '+'),
				('u', '\u{03BC}'),
				('v', '\u{22CE}'),
				('w', '\u{A7FF}'),
				('x', '\u{2297}'),
				('y', '\u{03B3}'),
				('z', '\u{01B5}'),
				('0', '\u{03B8}'),
				('1', 'I'),
				('2', '\u{01A7}'),
				('3', '\u{2208}'),
				('4', '\u{0AEB}'),
				('5', '\u{108AC}'),
				('6', '\u{194C}'),
				('7', '\u{0A6D}'),
				('8', '\u{1098}'),
				('9', '\u{0AED}'),
			]);

			Self { cmap: char_map }	
		}
	}


	impl CharMap {
		pub fn convert_char(&self, c: char) -> char {
			match self.cmap.get(&c) {
				None => c,
				Some(val) => *val,
			}
		}


		pub fn edit_map(&mut self, key: char, val: char) -> () {
			let currval = self.cmap.entry(key).or_insert(' ');
			*currval = val;
		}

	}


	pub struct EditMode {}
	pub struct CommandMode {}

	enum Modes {
		EditMode,
		CommandMode,
	}


	pub struct File {
		contents: Vec<String>,
		buffersize: i8,
		line: i32,
		column: i32,
	}

	impl Default for File {
		pub fn default() -> File {
			File{
				contents: vec![String::from("hello")], 
				buffersize: 40,
				line: 0,
				column: 0,
			}
		}
	}



	pub struct Display {
		file: File,
		mode: Modes,
		fps: u32,
		linewidth: i32,
		charwidth: i32,
		blinkrate: u32,
	}

	impl Default for Display {
		pub fn default() -> Display {
			Display{	
				file: File{ contents: Vec::new(), buffersize: 0, line: 0, column: 0 },
				mode: Modes::CommandMode,
				linewidth: 20,
				charwidth: 15,
				fps: 60,
				blinkrate: 1,
			}
		}
	}

	impl Display {

		pub fn draw_screen(&mut self) -> () {
			canvas.set_draw_color(Color::RGB(255, 255, 255));
			
			// blinking cursor
			if index % my_display.fps < my_display.fps/(2*my_display.blinkrate) {
				canvas.draw_line( 
					Point::new( self.file.column*self.charwidth, self.file.line*self.linewidth ),
					Point::new( self.file.column*self.charwidth, (self.file.line+1)*self.linewidth ) 
				).unwrap();
			
			// deal with magic values here
			canvas.draw_line( Point::new( 0, 37*self.linewidth), Point::new(800, 37*self.linewidth) );

			canvas.set_draw_color(Color::RGB(0, 0, 0));
        	canvas.present();
		}


		pub fn change_mode(&mut self) -> () {
			match self.mode {
				Modes::CommandMode => {self.mode = Modes::EditMode},
				Modes::EditMode => {self.mode = Modes::CommandMode},
			};
		}

	}

}
