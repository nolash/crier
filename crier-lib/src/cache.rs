use std::io::Write;


pub trait Cache {
    fn open(&mut self, id: String) -> &mut dyn Write;
    fn close(&mut self, id: String) -> usize;
}
