use super::ScopedSymbolTable;

#[test]
fn basic_shadowing() {
    let mut st = ScopedSymbolTable::new();
    st.put("x", 1);
    assert_eq!(st.get("x"), Some(&1));

    st.enter_scope();
    st.put("x", 2);
    assert_eq!(st.get("x"), Some(&2));

    st.exit_scope();
    assert_eq!(st.get("x"), Some(&1));
}

#[test]
fn multiple_puts_same_scope() {
    let mut st = ScopedSymbolTable::new();
    st.put("x", 10);
    st.enter_scope();
    st.put("x", 1);
    st.put("x", 2);
    assert_eq!(st.get("x"), Some(&2));

    st.exit_scope();
    assert_eq!(st.get("x"), Some(&10));
}

#[test]
fn multiple_puts_same_scope_with_no_outer_binding() {
    let mut st = ScopedSymbolTable::new();
    st.enter_scope();
    st.put("x", 1);
    st.put("x", 2);
    assert_eq!(st.get("x"), Some(&2));

    st.exit_scope();
    assert_eq!(st.get("x"), None);
    assert!(!st.contains("x"));
}

#[test]
fn independent_keys() {
    let mut st = ScopedSymbolTable::new();
    st.put("x", 1);
    st.put("y", 9);

    st.enter_scope();
    st.put("x", 2);
    assert_eq!(st.get("x"), Some(&2));
    assert_eq!(st.get("y"), Some(&9));

    st.exit_scope();
    assert_eq!(st.get("x"), Some(&1));
    assert_eq!(st.get("y"), Some(&9));
}

#[test]
fn deep_nesting_sanity() {
    let mut st = ScopedSymbolTable::new();
    st.put("x", 0);

    st.enter_scope();
    st.put("x", 1);
    st.enter_scope();
    st.put("x", 2);
    st.enter_scope();
    st.put("x", 3);
    assert_eq!(st.get("x"), Some(&3));

    st.exit_scope();
    assert_eq!(st.get("x"), Some(&2));
    st.exit_scope();
    assert_eq!(st.get("x"), Some(&1));
    st.exit_scope();
    assert_eq!(st.get("x"), Some(&0));
}

#[test]
fn exit_scope_with_no_bindings_added() {
    let mut st = ScopedSymbolTable::<i32>::new();
    st.enter_scope();
    st.exit_scope();
    assert_eq!(st.depth(), 0);
}

#[test]
fn scope_debug_helpers() {
    let mut st = ScopedSymbolTable::new();
    assert_eq!(st.depth(), 0);
    assert_eq!(st.len_current_scope(), 0);

    st.enter_scope();
    assert_eq!(st.depth(), 1);
    assert_eq!(st.len_current_scope(), 0);

    st.put("a", 1);
    st.put("b", 2);
    assert_eq!(st.len_current_scope(), 2);

    st.enter_scope();
    assert_eq!(st.depth(), 2);
    assert_eq!(st.len_current_scope(), 0);

    st.put("a", 3);
    assert_eq!(st.len_current_scope(), 1);
}

#[test]
#[should_panic(expected = "exit_scope called with no active scope")]
fn exit_scope_without_marker_panics() {
    let mut st = ScopedSymbolTable::<i32>::new();
    st.exit_scope();
}
