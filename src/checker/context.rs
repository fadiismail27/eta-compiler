use std::collections::HashMap;

use crate::checker::check::SemanticType;

#[derive(Clone, Debug)]
enum LogEntry {
    Marker,
    Key(String),
}

/// Scoped symbol table using:
/// - one hash map from key -> stack of values
/// - one mutation log with scope markers
#[derive(Clone, Debug)]
pub struct ScopedSymbolTable<V> {
    table: HashMap<String, Vec<V>>,
    log: Vec<LogEntry>,
    scope_sizes: Vec<usize>,
}

impl<V> ScopedSymbolTable<V> {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
            log: Vec::new(),
            scope_sizes: Vec::new(),
        }
    }

    pub fn enter_scope(&mut self) {
        self.log.push(LogEntry::Marker);
        self.scope_sizes.push(0);
    }

    /// Panics if there is no active scope marker.
    pub fn exit_scope(&mut self) {
        assert!(
            self.scope_sizes.pop().is_some(),
            "exit_scope called with no active scope"
        );

        loop {
            match self.log.pop() {
                Some(LogEntry::Marker) => break,
                Some(LogEntry::Key(name)) => {
                    if let Some(stack) = self.table.get_mut(&name) {
                        stack.pop();
                        if stack.is_empty() {
                            self.table.remove(&name);
                        }
                    } else {
                        panic!("log/table invariant broken: key missing during exit_scope");
                    }
                }
                None => panic!("log/table invariant broken: missing scope marker"),
            }
        }
    }

    pub fn put(&mut self, name: &str, value: V) {
        self.table.entry(name.to_string()).or_default().push(value);
        self.log.push(LogEntry::Key(name.to_string()));
        if let Some(size) = self.scope_sizes.last_mut() {
            *size += 1;
        }
    }

    pub fn get(&self, name: &str) -> Option<&V> {
        self.table.get(name).and_then(|stack| stack.last())
    }

    pub fn contains(&self, name: &str) -> bool {
        self.table.contains_key(name)
    }

    pub fn depth(&self) -> usize {
        self.scope_sizes.len()
    }

    pub fn len_current_scope(&self) -> usize {
        self.scope_sizes.last().copied().unwrap_or(0)
    }
}

#[derive(Clone, Debug)]
pub struct Context {
    symbols: ScopedSymbolTable<SemanticType>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            symbols: ScopedSymbolTable::new(),
        }
    }

    pub fn enter_scope(&mut self) {
        self.symbols.enter_scope();
    }

    pub fn exit_scope(&mut self) {
        self.symbols.exit_scope();
    }

    pub fn search(&self, name: &String) -> Option<&SemanticType> {
        self.symbols.get(name)
    }

    pub fn push(&mut self, name: &String, typ: &SemanticType) -> bool {
        self.symbols.put(name, typ.clone());
        true
    }

    pub fn remove(&mut self, name: &String) -> bool {
        if let Some(stack) = self.symbols.table.get_mut(name) {
            stack.pop();
            if stack.is_empty() {
                self.symbols.table.remove(name);
            }
            true
        } else {
            false
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[path = "context_tests.rs"]
mod context_tests;

