use std::cell::RefCell;
use std::rc::Rc;

trait Observer {
    fn update(&self, temperature: f32);
}

trait Subject {
    fn register_observer(&mut self, observer: Rc<dyn Observer>);
    fn remove_observer(&mut self, observer_id: usize);
    fn notify_observers(&self);
}

pub struct TemperatureStation {
    observers: Vec<(usize, Rc<dyn Observer>)>,
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
    pub fn set_temperature(&mut self, temp: f32) {
        println!("INFO: Temp updated to {:.1}", temp);
        self.temperature = temp;
        self.notify_observers();
    }
}

impl Subject for TemperatureStation {
    fn register_observer(&mut self, observer: Rc<dyn Observer>) {
        self.observers.push((self.counter, observer));
        self.counter += 1;
    }

    fn remove_observer(&mut self, observer_id: usize) {
        self.observers.retain(|(id, _)| *id != observer_id);
    }

    fn notify_observers(&self) {
        for (_, obs) in &self.observers {
            obs.update(self.temperature);
        }
    }
}
