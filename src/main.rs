mod calc;
use calc::*;

mod todo;
use todo::*;

fn main() {
    calc_demo();
    todo_demo();
}

fn calc_demo() {
    let mut calc = Calculator::new(10.0f32, 2.5f32);
    calc.apply::<AddCommand>();
    println!("Add: {}", calc.res);

    calc.apply::<MinCommand>();
    println!("Min: {}", calc.res);
}

fn todo_demo() {
    let mut todo = TodoList::new();
    let first = todo.add_todo(&mut Todo::new("Buy Milk"));
    let second = todo.add_todo(&mut Todo::new("Rice my linux desktop"));

    println!("Added todo : {:#?}\n", todo);

    todo.toggle_todo(first);
    println!("Toggle todo : {:#?}\n", todo);

    todo.del_todo(second);
    println!("Delete todo : {:#?}\n", todo);

    todo.undo();
    println!("Undo todo : {:#?}\n", todo);

    todo.redo();
    println!("Redo todo : {:#?}\n", todo);
}
