pub struct Logger<'a> {
    pub stdout: std::io::StdoutLock<'a>,
    pub stderr: std::io::StderrLock<'a>,
}

pub fn log(stream: &mut impl std::io::Write, message: String) {
    stream.write(message.as_bytes()).expect("Failed to write");
}