#[derive(Debug)]
pub enum TodoOperation {
    NEW,
    DEL,
    EDT,
}

#[derive(Clone, Debug)]
pub struct Todo {
    id: i32,
    body: String,
    checked: bool,
}

impl Todo {
    pub fn new(body: &str) -> Self {
        Self {
            id: 0,
            body: body.to_string(),
            checked: false,
        }
    }
}

#[derive(Debug)]
pub struct TodoHist {
    id: i32,
    before: Option<Todo>,
    after: Option<Todo>,
    operation: TodoOperation,
}

#[derive(Debug)]
pub struct TodoList {
    list: Vec<Todo>,
    undo_list: Vec<TodoHist>,
    redo_list: Vec<TodoHist>,
    counter: i32,
}

impl TodoList {
    fn search_elm(&mut self, id: i32) -> Option<(usize, &mut Todo)> {
        for i in 0..self.list.len() {
            if self.list[i].id == id {
                return Some((i, &mut self.list[i]));
            }
        }
        return None;
    }

    fn exec_todohist(&mut self, hist: &TodoHist) {
        todo!();
    }

    pub fn new() -> Self {
        Self {
            list: Vec::new(),
            undo_list: Vec::new(),
            redo_list: Vec::new(),
            counter: 0,
        }
    }

    pub fn add_todo(&mut self, todo: &mut Todo) {
        todo.id = self.counter;
        self.list.push(todo.clone());
        self.undo_list.push(TodoHist {
            id: todo.id,
            before: None,
            after: Some(todo.clone()),
            operation: TodoOperation::NEW,
        });
        self.redo_list.clear();
        self.counter += 1;
    }

    pub fn del_todo(&mut self, id: i32) {
        if let Some((i, t)) = self.search_elm(id) {
            let new_t = t.clone();
            self.undo_list.push(TodoHist {
                id: new_t.id,
                before: Some(new_t),
                after: None,
                operation: TodoOperation::NEW,
            });
            self.redo_list.clear();
            self.list.remove(i);
        }
    }

    pub fn toggle_todo(&mut self, id: i32) {
        if let Some((_, todo)) = self.search_elm(id) {
            let old_t = todo.clone();
            todo.checked = !todo.checked;
            let new_t = todo.clone();
            self.undo_list.push(TodoHist {
                id: old_t.id,
                before: Some(old_t),
                after: Some(new_t),
                operation: TodoOperation::EDT,
            });
        }
    }

    pub fn undo(&mut self) {
        todo!();
    }
    pub fn redo(&mut self) {
        todo!();
    }
}
