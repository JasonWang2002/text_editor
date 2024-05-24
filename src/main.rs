extern crate sdl2;

use std::time::Duration;
use std::collections::HashMap;
use std::default::Default;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;

use text_editor::editor;


fn main() {

	let char_map: CharMap = editor::CharMap( Default::default() );
	let mut my_file: File = editor::File( Default::default() );
	let my_string: String = String::from("Testing 123!");
	
	println!("{:?}", my_file.contents);
	println!("{}", my_file.buffersize);
	
	let mod_string: String = my_string.to_lowercase()
		.chars()
		.map( |c| char_map.convert_char(c) )
		.collect();


	println!("{}", mod_string);

	
	my_file.line += 1;
	my_file.column += 1;

	
	let my_display = editor::Display( file: my_file, Default::default() }; 	


	let mut index: u32 = 0;
	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();
	let window = video_subsystem.window("My text editor", 800, 600, )
		.position_centered()
		//.resizable()
		.build()
		.unwrap();

	let mut canvas = window.into_canvas().build().unwrap();


	canvas.set_draw_color(Color::RGB(0, 0, 0));
	canvas.clear();
	canvas.present();

	let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.clear();
		index += 1;
		for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => { break 'running },
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => { my_display.change_mode() },
            }
        }
		
		my_display.draw_screen();

        
		std::thread::sleep(Duration::new(0, 1_000_000_000u32 / my_display.fps));
    }


}

