//! TDDで小さな振る舞いを積み上げるRust教材の完成実装。

pub fn greeting(name: &str) -> String {
    match name.trim() {
        "" => "Hello, Rust!".to_owned(),
        name => format!("Hello, {name}!"),
    }
}

pub fn repeat(value: &str, count: usize) -> String {
    value.repeat(count)
}
pub fn sum(values: &[i32]) -> i32 {
    values.iter().copied().sum()
}
pub fn average(values: &[i32]) -> Option<f64> {
    (!values.is_empty()).then(|| sum(values) as f64 / values.len() as f64)
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
    fn repeat_handles_zero_and_multiple_values() {
        assert_eq!(repeat("ha", 3), "hahaha");
        assert_eq!(repeat("ha", 0), "");
    }

    #[test]
    fn average_returns_none_for_empty_input() {
        assert_eq!(sum(&[1, 2, 3]), 6);
        assert_eq!(average(&[1, 2, 4]), Some(7.0 / 3.0));
        assert_eq!(average(&[]), None);
    }

    #[test]
    fn score_enforces_range_and_pass_mark() {
        assert!(Score::new(100).unwrap().passed());
        assert!(!Score::new(59).unwrap().passed());
        assert_eq!(Score::new(101), Err(ScoreError::OutOfRange(101)));
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
