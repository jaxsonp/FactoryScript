#[macro_export]
macro_rules! debug {
    ($level:literal, $msg:literal, $($args:expr),*) => {
		unsafe {
			if $level <= DEBUG_LEVEL {
				if $level > 1 && COLOR_OUTPUT {
					use inline_colorization::{color_bright_black, color_reset};
					print!("{color_bright_black}");
					print!($msg, $($args),*);
					println!("{color_reset}");
				} else {
					println!($msg, $($args),*);
				}
			}
		}
    };
    ($level:literal, $msg:literal) => {
		unsafe {
			if $level <= DEBUG_LEVEL {
				if $level > 1 && COLOR_OUTPUT {
					use inline_colorization::{color_bright_black, color_reset};
					print!("{color_bright_black}");
					print!($msg);
					println!("{color_reset}");
				} else {
					println!($msg);
				}
			}
		}
    };
}

#[macro_export]
macro_rules! print_err {
    ($msg:literal, $($args:expr),*) => {
		unsafe {
			if COLOR_OUTPUT {
				use inline_colorization::{color_red, color_reset};
				print!($msg, $($args),*);
				println!("{color_reset}");
			} else {
				print!("ERROR: ");
				println!($msg, $($args),*);
			}
		}
    };
    ($msg:literal) => {
		unsafe {
			if COLOR_OUTPUT {
				use inline_colorization::{color_red, color_reset};
				print!("{color_red}ERROR: ");
				print!($msg);
				println!("{color_reset}");
			} else {
				print!("ERROR: ");
				println!($msg);
			}
		}
    };
}
