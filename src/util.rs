// Remove all whitespace in mutable string reference
pub fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
}