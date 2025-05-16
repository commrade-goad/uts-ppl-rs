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
    todo.add_todo(&mut Todo::new("Buy Milk"));
    println!("{:?}", todo);
}
