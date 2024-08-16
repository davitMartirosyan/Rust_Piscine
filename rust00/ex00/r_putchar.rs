use std::io::Write;

pub fn r_putchar(c: char)
{
	let mut stdout = std::io::stdout();
	stdout.write_all(&[c as u8]).expect("Faild to write char");
	stdout.flush().expect("Failed to flush");
}

