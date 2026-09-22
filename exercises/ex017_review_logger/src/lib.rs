//! Lesson 04-5: 不要なtraitを見抜く
//!
//! 元の設計（このままでは実装しない）:
//!
//! ```text
//! trait Logger { fn log(&self, message: &str) -> String; }
//! struct ConsoleLogger;
//! impl Logger for ConsoleLogger {
//!     fn log(&self, message: &str) -> String { format!("[LOG] {message}") }
//! }
//! struct App<L: Logger> { logger: L }
//! impl<L: Logger> App<L> {
//!     fn new(logger: L) -> Self { App { logger } }
//!     fn run(&self) -> String { self.logger.log("starting up") }
//! }
//! ```
//!
//! `Logger` の実装は `ConsoleLogger` の1つしかなく、`App<L>` も
//! `ConsoleLogger` でしか使われていなかった。trait と generic パラメータを
//! 取り除き、直接メソッドを呼ぶ設計にする。

pub struct App;

impl App {
    pub fn new() -> Self {
        App
    }

    /// 元の Logger::log 相当のロジックを、直接メソッドとして持つ。
    pub fn run(&self) -> String {
        todo!("\"[LOG] starting up\" を返してください（元のConsoleLogger::logと同じ形式）")
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
