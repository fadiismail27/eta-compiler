use std::fmt;

// ── Temp ──────────────────────────────────────────────────
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Temp {
    name: String, // private
}

// Create new Temp for any type that can be turned into a String (e.g., String or &str)
impl Temp {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name :name.into() }
    }

    pub fn as_str(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for Temp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }    
} 

// ── Label ──────────────────────────────────────────────────
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Label {
    name:String, // private
}

impl Label {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name:name.into()}
    }

    pub fn as_str(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }    
} 

