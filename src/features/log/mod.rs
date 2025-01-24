use chrono::Local;
use log::{Level, LevelFilter, Metadata, Record};
use std::fs::OpenOptions;
use std::io::Write;

struct SimpleLogger;

static LOGGER: SimpleLogger = SimpleLogger;

pub fn init() {
    let res = log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Debug));
    if let Err(e) = res {
        panic!("初始化日志失败！===> {}", e);
    }
}
impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let out = format!(
                "[{} {} {}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.target(),
                record.args()
            );
            println!("{}", out);
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open("output.log")
                .expect("不能打开output.log文件");
            writeln!(file, "{}", out).expect("写入文件失败！");
        }
    }

    fn flush(&self) {}
}