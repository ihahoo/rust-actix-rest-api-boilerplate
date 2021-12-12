use slog::Logger;
use slog::Drain;
use std::fs;

pub fn get_logger() -> Logger {
    fs::create_dir_all("data/logs").unwrap();
    let log_path = "data/logs/app.log";
    let file = fs::OpenOptions::new()
      .create(true)
      .write(true)
      .append(true)
      .open(log_path)
      .unwrap();

    let decorator = slog_term::TermDecorator::new().build();
    let drain1 = slog_term::FullFormat::new(decorator).build().fuse();

    let drain2 = slog_json::Json::new(file)
        .set_pretty(false)
        .set_newlines(true)
        .build()
        .fuse();

    let drain = slog_async::Async::new(slog::Duplicate::new(drain1, drain2).fuse()).build().fuse();
  
    let logger = slog::Logger::root(
        drain,
        o!(
            "msg" => slog::PushFnValue(move |record, ser| {
                ser.emit(record.msg())
            }),
            "time" => slog::PushFnValue(move |_ : &slog::Record, ser| {
                ser.emit(chrono::Utc::now().to_rfc3339())
            }),
            "level" => slog::FnValue(move |record| {
                record.level().as_str()
            }),
            "file" => slog::FnValue(move |record| {
                format!("{}:{}", record.file(), record.line())
            }),
        ),
    );

    logger
}