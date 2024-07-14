#[macro_export]
macro_rules! debug {
    ($level:literal, $msg:literal, $($args:expr),*) => {
		unsafe {
			if $level <= DEBUG_LEVEL {
				if $level > 1 && COLOR_OUTPUT {
					print!("\x1b[90m");
					print!($msg, $($args),*);
					println!("\x1b[0m");
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
					print!("\x1b[90m");
					print!($msg);
					println!("\x1b[0m");
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
				print!("\x1b[31m");
				print!($msg, $($args),*);
				println!("\x1b[0m");
			} else {
				print!("ERROR! ");
				println!($msg, $($args),*);
			}
		}
    };
    ($msg:literal) => {
		unsafe {
			if COLOR_OUTPUT {
				print!("\x1b[31m");
				print!($msg);
				println!("\x1b[0m");
			} else {
				print!("ERROR! ");
				println!($msg);
			}
		}
    };
}
