use std::error::Error;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::{JoinHandle};

struct CheckinCounter {
    id: u8,
    baggage_log: Arc<Mutex<Vec<String>>>,
}

impl CheckinCounter {
    fn new(num_of_counters :Arc<Mutex<u8>>, baggage_log: Arc<Mutex<Vec<String>>>) -> Self {
        let mut id = *num_of_counters.lock().unwrap();
        id += 1;
        CheckinCounter{id, baggage_log}
    }

    fn process_baggage(&mut self, baggage : String) -> Result<JoinHandle<()>, Box<dyn Error>> {
        let mut baggage_log = self.baggage_log.clone();
        let handle = thread::spawn( move|| {

            let mut local_lock = baggage_log.lock().unwrap();


            local_lock.push(baggage.clone());

            println!("Baggage checked in: {}", baggage);
            println!("{}", local_lock.join("\n"));
        });
        Ok(handle)


    }
}


fn main() -> Result<(), Box<dyn Error>> {
    let number_of_counters = Arc::new(Mutex::new(0u8));
    let baggage_log = Arc::new(Mutex::new(Vec::<String>::with_capacity(10)));
    baggage_log.lock().unwrap().push(String::from("Sko, jakke, pæn"));
    baggage_log.lock().unwrap().push(String::from("Bent, swagster"));
    baggage_log.lock().unwrap().push(String::from("Svend, Polo"));
    let mut counter1:CheckinCounter = CheckinCounter::new(number_of_counters.clone(), baggage_log.clone());
    let mut counter2:CheckinCounter = CheckinCounter::new(number_of_counters.clone(), baggage_log.clone());
    counter1.process_baggage(String::from("test1"))?.join();
    counter2.process_baggage(String::from("test2"))?.join();

    Ok(())





}


