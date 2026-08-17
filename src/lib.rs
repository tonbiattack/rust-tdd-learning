//! TDDで小さな振る舞いを積み上げるRust教材の完成実装。

use std::collections::HashMap;
use std::io::Read;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;

/// Hello, Rust: 入力を所有する挨拶文へ変換する。
pub fn greeting(name: &str) -> String {
    match name.trim() {
        "" => "Hello, Rust!".to_owned(),
        name => format!("Hello, {name}!"),
    }
}

/// Integers: 小さな純粋関数からテストを始める。
pub fn add_two(value: i32) -> i32 {
    value + 2
}

/// Iteration: 境界値を含む反復。
pub fn repeat(value: &str, count: usize) -> String {
    value.repeat(count)
}
pub fn sum(values: &[i32]) -> i32 {
    values.iter().copied().sum()
}
pub fn average(values: &[i32]) -> Option<f64> {
    (!values.is_empty()).then(|| sum(values) as f64 / values.len() as f64)
}

/// Arrays and slices: 入力順を保ったフィルタ。
pub fn above(values: &[i32], threshold: i32) -> Vec<i32> {
    values
        .iter()
        .copied()
        .filter(|value| *value > threshold)
        .collect()
}

/// Maps: 単語を正規化して頻度を集計する。
pub fn word_counts(input: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for word in input.split_whitespace().map(|word| word.to_lowercase()) {
        *counts.entry(word).or_insert(0) += 1;
    }
    counts
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Score(u8);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScoreError {
    OutOfRange(u8),
}

impl Score {
    pub fn new(value: u8) -> Result<Self, ScoreError> {
        (value <= 100)
            .then_some(Self(value))
            .ok_or(ScoreError::OutOfRange(value))
    }
    pub fn value(&self) -> u8 {
        self.0
    }
    pub fn passed(&self) -> bool {
        self.0 >= 60
    }
}

/// Pointers and errors: 状態変更を専用型に閉じ込める。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WalletError {
    InsufficientFunds { requested: u32, available: u32 },
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Wallet {
    balance: u32,
}

impl Wallet {
    pub fn deposit(&mut self, amount: u32) {
        self.balance += amount;
    }
    pub fn withdraw(&mut self, amount: u32) -> Result<(), WalletError> {
        if amount > self.balance {
            return Err(WalletError::InsufficientFunds {
                requested: amount,
                available: self.balance,
            });
        }
        self.balance -= amount;
        Ok(())
    }
    pub fn balance(&self) -> u32 {
        self.balance
    }
}

pub trait Notifier {
    fn send(&mut self, message: &str);
}

pub fn publish_result<N: Notifier>(notifier: &mut N, score: &Score) {
    let result = if score.passed() {
        "passed"
    } else {
        "not passed"
    };
    notifier.send(&format!("score: {} ({result})", score.value()));
}

/// Concurrency: スレッド間の共有ではなく所有権を分割して集計する。
pub fn parallel_sum(values: &[i32]) -> i32 {
    let midpoint = values.len() / 2;
    thread::scope(|scope| {
        let left = scope.spawn(|| values[..midpoint].iter().sum::<i32>());
        let right = scope.spawn(|| values[midpoint..].iter().sum::<i32>());
        left.join().expect("left worker panicked") + right.join().expect("right worker panicked")
    })
}

/// Context相当: キャンセル可能な処理の最小契約。
pub fn count_until_cancel(cancelled: Arc<AtomicBool>, limit: usize) -> usize {
    let mut completed = 0;
    while completed < limit && !cancelled.load(Ordering::Acquire) {
        completed += 1;
    }
    completed
}

/// Select相当: 受け取った2種類のメッセージを共通のイベントへ変換する。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    Data(String),
    Closed,
}

pub fn event_from_message(message: Option<String>) -> Event {
    message.map_or(Event::Closed, Event::Data)
}

/// Reflection相当: enumで許可された表示形式だけを扱う。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Plain,
    Uppercase,
}

pub fn format_name(name: &str, format: Format) -> String {
    match format {
        Format::Plain => name.to_owned(),
        Format::Uppercase => name.to_uppercase(),
    }
}

/// 数学: 時刻を円形時計のSVGへ変換する純粋関数。
pub fn render_clock_svg(hour: u32, minute: u32) -> String {
    let hour = hour % 12;
    let hour_angle = (hour as f64 + minute as f64 / 60.0) * 30.0;
    let minute_angle = minute as f64 * 6.0;
    format!(
        "<svg data-hour-angle=\"{hour_angle:.1}\" data-minute-angle=\"{minute_angle:.1}\"></svg>"
    )
}

/// Roman numerals: 変換結果の不変条件をテストしやすい小さな実装。
pub fn roman(value: u16) -> Option<String> {
    if !(1..=3999).contains(&value) {
        return None;
    }
    let table = [
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];
    let mut remaining = value;
    let mut output = String::new();
    for (number, symbol) in table {
        while remaining >= number {
            output.push_str(symbol);
            remaining -= number;
        }
    }
    Some(output)
}

pub trait Clock {
    fn now_minutes(&self) -> u32;
}
pub fn greeting_for_clock<C: Clock>(clock: &C) -> &'static str {
    match clock.now_minutes() / 60 % 24 {
        5..=11 => "Good morning",
        12..=17 => "Good afternoon",
        _ => "Good evening",
    }
}

/// I/O: 実ファイルではなくRead境界を注入してパースする。
pub fn read_numbers<R: Read>(mut reader: R) -> Result<Vec<i32>, String> {
    let mut input = String::new();
    reader
        .read_to_string(&mut input)
        .map_err(|error| error.to_string())?;
    input
        .split_whitespace()
        .map(|token| {
            token
                .parse::<i32>()
                .map_err(|error| format!("invalid integer {token}: {error}"))
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    pub name: String,
    pub score: i32,
}

pub fn sorted_records(mut records: Vec<Record>) -> Vec<Record> {
    records.sort_by(|left, right| {
        right
            .score
            .cmp(&left.score)
            .then_with(|| left.name.cmp(&right.name))
    });
    records
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Add(String),
    List,
}

pub fn parse_command(args: &[&str]) -> Result<Command, &'static str> {
    match args {
        ["add", title] if !title.trim().is_empty() => Ok(Command::Add((*title).to_owned())),
        ["list"] => Ok(Command::List),
        _ => Err("usage: add <title> | list"),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Todo {
    pub title: String,
    pub completed: bool,
}

#[derive(Debug, Default)]
pub struct TodoList {
    items: Vec<Todo>,
}

impl TodoList {
    pub fn add(&mut self, title: impl Into<String>) -> Result<(), &'static str> {
        let title = title.into();
        if title.trim().is_empty() {
            return Err("title must not be empty");
        }
        self.items.push(Todo {
            title,
            completed: false,
        });
        Ok(())
    }
    pub fn complete(&mut self, index: usize) -> Result<(), &'static str> {
        let item = self.items.get_mut(index).ok_or("todo not found")?;
        item.completed = true;
        Ok(())
    }
    pub fn pending_titles(&self) -> Vec<&str> {
        self.items
            .iter()
            .filter(|item| !item.completed)
            .map(|item| item.title.as_str())
            .collect()
    }
    pub fn len(&self) -> usize {
        self.items.len()
    }
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_uses_name_and_default() {
        assert_eq!(greeting("Ada"), "Hello, Ada!");
        assert_eq!(greeting("  "), "Hello, Rust!");
    }
    #[test]
    fn add_two_covers_negative_and_zero() {
        assert_eq!(add_two(-2), 0);
        assert_eq!(add_two(0), 2);
    }
    #[test]
    fn repeat_and_average_handle_boundaries() {
        assert_eq!(repeat("ha", 3), "hahaha");
        assert_eq!(repeat("ha", 0), "");
        assert_eq!(average(&[]), None);
    }
    #[test]
    fn collections_keep_behavior_explicit() {
        assert_eq!(sum(&[1, 2, 3]), 6);
        assert_eq!(above(&[1, 5, 8], 5), vec![8]);
        let counts = word_counts("Rust rust test");
        assert_eq!(counts["rust"], 2);
    }
    #[test]
    fn score_enforces_range_and_pass_mark() {
        assert!(Score::new(100).unwrap().passed());
        assert!(!Score::new(59).unwrap().passed());
        assert_eq!(Score::new(101), Err(ScoreError::OutOfRange(101)));
    }
    #[test]
    fn wallet_preserves_state_after_failed_withdrawal() {
        let mut wallet = Wallet::default();
        wallet.deposit(10);
        assert_eq!(
            wallet.withdraw(11),
            Err(WalletError::InsufficientFunds {
                requested: 11,
                available: 10
            })
        );
        assert_eq!(wallet.balance(), 10);
    }
    struct Spy {
        messages: Vec<String>,
    }
    impl Notifier for Spy {
        fn send(&mut self, message: &str) {
            self.messages.push(message.to_owned());
        }
    }
    #[test]
    fn publishing_uses_injected_notifier() {
        let mut spy = Spy { messages: vec![] };
        publish_result(&mut spy, &Score::new(80).unwrap());
        assert_eq!(spy.messages, vec!["score: 80 (passed)"]);
    }
    #[test]
    fn concurrency_and_cancellation_are_observable() {
        assert_eq!(parallel_sum(&[1, 2, 3, 4]), 10);
        let flag = Arc::new(AtomicBool::new(true));
        assert_eq!(count_until_cancel(flag, 10), 0);
    }
    #[test]
    fn enum_boundary_is_exhaustive() {
        assert_eq!(event_from_message(None), Event::Closed);
        assert_eq!(format_name("Ada", Format::Uppercase), "ADA");
    }
    #[test]
    fn roman_and_clock_have_invariants() {
        assert_eq!(roman(4), Some("IV".to_owned()));
        assert_eq!(roman(4000), None);
        assert!(render_clock_svg(3, 30).contains("data-hour-angle=\"105.0\""));
    }
    struct FakeClock {
        minutes: u32,
    }
    impl Clock for FakeClock {
        fn now_minutes(&self) -> u32 {
            self.minutes
        }
    }
    #[test]
    fn time_is_injected() {
        assert_eq!(
            greeting_for_clock(&FakeClock { minutes: 8 * 60 }),
            "Good morning"
        );
        assert_eq!(
            greeting_for_clock(&FakeClock { minutes: 20 * 60 }),
            "Good evening"
        );
    }
    #[test]
    fn io_sorting_and_cli_are_testable_without_environment() {
        assert_eq!(read_numbers("3 1 2".as_bytes()).unwrap(), vec![3, 1, 2]);
        assert!(read_numbers("x".as_bytes()).is_err());
        let records = sorted_records(vec![
            Record {
                name: "B".into(),
                score: 2,
            },
            Record {
                name: "A".into(),
                score: 3,
            },
        ]);
        assert_eq!(records[0].name, "A");
        assert_eq!(parse_command(&["list"]), Ok(Command::List));
    }
    #[test]
    fn todo_list_rejects_empty_and_tracks_completion() {
        let mut todos = TodoList::default();
        assert_eq!(todos.add(""), Err("title must not be empty"));
        todos.add("write tests").unwrap();
        todos.add("refactor").unwrap();
        todos.complete(0).unwrap();
        assert_eq!(todos.pending_titles(), vec!["refactor"]);
        assert_eq!(todos.complete(99), Err("todo not found"));
        assert_eq!(todos.len(), 2);
    }
}
