use std::fs::File;

pub struct TextFile {
    state: Option<Box<dyn State>>,
    content: String,
}

impl TextFile {
    pub fn new() -> TextFile {
        TextFile{
            state: Some(Box::new(CleanDetached {})),
            content: String::new(),
        }
    }

    pub fn set_content(&mut self, content: &str) {
        self.content = content.to_string();
    }
}

trait State {}

struct CleanDetached {}
struct DirtyDetached {}
struct CleanAttached {}
struct DirtyAttached {}

impl State for CleanDetached {}
impl State for DirtyDetached {}
impl State for CleanAttached {}
impl State for DirtyAttached {}

// operations:
// - edit
// - save to
// - save
//
// transitions:
// - CleanDetached
//   - [edit] -> DirtyDetached
//   - [save to] -> CleanAttached
//   - [save] -> CleanDetached
// - DirtyDetached
//   - [edit] -> DirtyDetached
//   - [save to] -> CleanAttached
//   - [save] -> ERROR
// - CleanAttached
//   - [edit] -> DirtyAttached
//   - [save to] -> CleanAttached
//   - [save] -> CleanAttached
// - DirtyAttached
//   - [edit] -> DirtyAttached
//   - [save to] -> CleanAttached
//   - [save] -> CleanAttached
