use core::f32;
use std::cell::RefCell;
use std::rc::Rc;

pub trait Observer {
    fn update(&mut self, temperature: f32);
    fn print_status(&self);
    fn print_statistics(&mut self);
}

trait Subject {
    fn notify_observers(&self);
}

pub struct TemperatureStation {
    observers: Vec<(usize, Rc<RefCell<dyn Observer>>)>,
    temperature: f32,
    counter: usize,
}

impl TemperatureStation {
    pub fn new() -> Self {
        Self {
            observers: Vec::new(),
            temperature: 0.0,
            counter: 0,
        }
    }
    pub fn set_temperature(&mut self, temp: f32, observer_id: Option<usize>) {
        println!("INFO: Temp updated to {:.1}", temp);
        self.temperature = temp;
        self.notify_observers();
    }
    pub fn register_observer(&mut self, observer: Rc<RefCell<dyn Observer>>) {
        self.observers.push((self.counter, observer));
        self.counter += 1;
    }
    pub fn remove_observer(&mut self, observer_id: usize) {
        self.observers.retain(|(id, _)| *id != observer_id);
    }
}

impl Subject for TemperatureStation {
    fn notify_observers(&self) {
        for (_, obs) in &self.observers {
            let mut obs_mut = obs.borrow_mut();
            obs_mut.update(self.temperature);
            obs_mut.print_status();
            obs_mut.print_statistics();
        }
    }
}

pub struct Display {
    name: String,
    temperature: Vec<f32>,
    max: f32,
    min: f32,
}

impl Display {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            temperature: Vec::new(),
            max: 0.0f32,
            min: 0.0f32,
        }
    }
}

impl Observer for Display {
    fn update(&mut self, temperature: f32) {
        self.temperature.push(temperature);
    }
    fn print_status(&self) {
        println!(
            "INFO: The temperature at {} changed to {:.1}",
            self.name,
            self.temperature[self.temperature.len() - 1]
        );
    }
    fn print_statistics(&mut self) {
        self.max = self
            .temperature
            .iter()
            .copied()
            .reduce(f32::max)
            .expect("WOOHO");
        self.min = self
            .temperature
            .iter()
            .copied()
            .reduce(f32::min)
            .expect("WOOHO");

        let sum: f32 = self.temperature.iter().sum();
        let avg = sum / self.temperature.len() as f32;

        println!(
            "INFO: The temperature at {} : max -> {:.1}, min -> {:.1}, avg -> {:.1}",
            self.name, self.max, self.min, avg
        );
    }
}
