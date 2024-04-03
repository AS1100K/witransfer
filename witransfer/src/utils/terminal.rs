//! Dynamic User Selectable List using `BTreeMap`
pub use crossterm::style::Color;
use crossterm::{cursor, queue, style, ExecutableCommand, QueueableCommand};
use std::collections::BTreeMap;
use std::io::{stdout, Stdout, Write};

pub struct Terminal<K>
where
    K: Ord,
{
    stdout: Stdout,
    /// > NOTE: Any modification to `BTreeMap` will not update the terminal directly.
    pub data: BTreeMap<K, String>,
    background_color: Option<Color>,
    prev_lines: u32,
}

impl<K> Terminal<K>
where
    K: Ord,
{
    /// Returns an instance of Terminal
    ///
    /// Here `background_color` is the background color for the terminal text.
    ///
    /// # Examples
    /// Terminal with no background color
    /// ```rust
    /// use witransfer::terminal::Terminal;
    ///
    /// let terminal: Terminal<String> = Terminal::new(None);
    /// ```
    ///
    /// Terminal with background color `Cyan`
    /// ```rust
    /// use witransfer::terminal::{Terminal, Color};
    ///
    /// let terminal: Terminal<String> = Terminal::new(Some(Color::Cyan));
    /// ```
    pub fn new(background_color: Option<Color>) -> Terminal<K> {
        Terminal {
            stdout: stdout(),
            data: BTreeMap::new(),
            background_color,
            prev_lines: 0,
        }
    }

    /// Create a terminal instance with a prompt question.
    ///
    /// # Example
    ///
    /// ```rust
    /// use witransfer::terminal::Terminal;
    /// // --snip--
    /// # let mut terminal: Terminal<String> = Terminal::new(None);
    /// terminal.with_prompt("Please select the device:".to_string()).unwrap();
    /// ```
    pub fn with_prompt(&mut self, prompt: String) -> Result<(), &'static str> {
        if self.stdout.write_all((prompt + "\n").as_bytes()).is_ok() {
            // self.prev_lines += 1;
            Ok(())
        } else {
            Err("Unable to write line in the terminal.")
        }
    }

    /// Inserts a new value to `BTreeMap` if it doesn't exist.
    ///
    /// Here, `identifier` is the `key` of `BTreeMap`
    /// and `content` is the `value` of `BTreeMap`.
    ///
    /// Returns `Result` enum.
    ///
    /// # Example
    ///
    /// ```rust
    /// use witransfer::terminal::Terminal;
    ///
    /// let mut terminal: Terminal<usize> = Terminal::new(None);
    /// terminal.insert(1, "i am here.".to_string()).unwrap();
    /// assert_eq!(terminal.get(&1).unwrap(), &"i am here.");
    /// ```
    pub fn insert(&mut self, identifier: K, content: String) -> Result<(), &'static str> {
        if self.data.contains_key(&identifier) {
            Err("Key Already exists.")
        } else {
            // let mut content_two = &content;
            self.stdout.write_all((content.to_owned() + "\n").as_bytes()).expect("TODO: panic message");
            self.data.insert(identifier, content);
            Ok(())
        }
    }

    /// Modify existing `identifier`'s content
    /// or insert that data if it doesn't exist.
    ///
    /// # Examples
    /// ```rust
    /// use witransfer::terminal::Terminal;
    ///
    /// let mut terminal: Terminal<usize> = Terminal::new(None);
    /// terminal.insert(1, "Hello World!".to_string()).unwrap();
    /// assert_eq!(terminal.get(&1).unwrap(), &"Hello World!".to_string());
    ///
    /// terminal.modify(1, "Hello, I was there.".to_string()).unwrap();
    /// assert_eq!(terminal.get(&1).unwrap(), &"Hello, I was there.".to_string());
    /// ```
    pub fn modify(&mut self, identifier: K, content: String) -> Result<(), &'static str> {
        self.data.insert(identifier, content);
        Ok(())
    }

    /// Removes the `identifier` from `BTreeMap.
    ///
    /// # Examples
    ///
    /// The following example will panic because there is no key `Hello, I was there.` on the map.
    /// ```rust, should_panic
    /// use witransfer::terminal::Terminal;
    ///
    /// let mut terminal: Terminal<&str> = Terminal::new(None);
    /// terminal.remove("Hello, I was there.").unwrap();
    /// ```
    pub fn remove(&mut self, identifier: K) -> Result<String, &'static str> {
        match self.data.remove(&identifier) {
            Some(content) => Ok(content),
            None => Err("Identifier doesn't exist in the map."),
        }
    }

    /// Returns the key-value pair corresponding to the supplied key.
    ///
    /// The supplied key may be any borrowed form of the map’s key type, but the ordering on the
    /// borrowed form must match the ordering on the key type.
    ///
    /// # Examples
    /// ```rust
    /// use witransfer::terminal::Terminal;
    /// let mut terminal: Terminal<usize> = Terminal::new(None);
    /// terminal.insert(1, "Two".to_string()).unwrap();
    ///
    /// assert_eq!(terminal.get(&1).unwrap(), &"Two".to_string());
    /// ```
    pub fn get(&mut self, identifier: &K) -> Result<&String, &'static str> {
        match self.data.get(identifier) {
            Some(content) => Ok(content),
            None => Err("Unable to get the value of designated identifier"),
        }
    }
}
