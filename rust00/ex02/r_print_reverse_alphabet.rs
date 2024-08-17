pub fn r_print_reverse_alphabet()
{
    for i in (65..=90).rev() {
        print!("{} ", i as u8 as char);
    }
}