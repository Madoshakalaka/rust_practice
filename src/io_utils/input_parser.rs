// use std::iter::Filter;
// use std::f64;
use std::io;
use std::io::Write;
use std::str::SplitWhitespace;


#[derive(Default)]
pub struct SpaceSepValGetter {
    input_buffer: String
}

impl SpaceSepValGetter {
    /// this fn gets space separated values from stdin
    pub(crate) fn get_ssv(&mut self, prompt: &str) -> SplitWhitespace {
        self.input_buffer.clear();
        print!("{}", prompt);
        io::stdout().flush().ok().expect("Could not flush stdout");

        io::stdin().read_line(&mut self.input_buffer).expect("Wrong!");
        let args = self.input_buffer.split_whitespace();
        args
    }
}
