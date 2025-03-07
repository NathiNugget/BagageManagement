use std::error::Error;
use std::fs::{File, FileType};
use std::ops::Index;
use std::sync::{Arc, Mutex};
use std::{result, thread};
use std::thread::{JoinHandle};

struct Lufthavn {
    skranke: Vec<Skranke>,
    flights: Vec<Fly>,
    rejsende: Arc<Mutex<Vec<Rejsende>>>,
    terminal : Arc<Mutex<Vec<Terminal>>>,
    log : Arc<Mutex<Vec<String>>>,
    file: Arc<Mutex<File>>

    // mangler book impl
}

struct Fly {
    id : u16,
    passagere : Vec<Rejsende>,
    baggage : Vec<Kuffert>,
}

impl Fly {
    // mangler metoder
}

struct Skranke {
    id : u16,
    is_busy : bool,
}

impl Skranke {
    // mangler LoadOnPlane(Kuffert, dest)
}

#[derive(Debug)]
struct Rejsende {
    id : u16,
    navn: String,
    kuffert: Option<Kuffert>,
}

impl Rejsende {
    fn new(id: u16, navn: String, kuffert: Option<Kuffert>) -> Self {
        Rejsende {id, navn, kuffert}
    }
}

struct Terminal {
    id : u16,
    rejsende : Vec<Rejsende>,
    baggage : Vec<Kuffert>
}

impl Terminal {
    fn pickup_baggage(&mut self, rejsende : &Rejsende, baggage: Kuffert) // tænker at vi skal bruge en mutable reference for at få fat i listen med rejsende og ændre den .
    {
        if let Some(i) = self.baggage.iter().position(|x| x.ejer_id == rejsende.id) {
            self.baggage.remove(i);
            println!("baggage taget fra rejsende med id: {}", rejsende.id)
        }
    }
}

#[derive(Debug)]
struct Kuffert {
    ejer_id : u16,
    beskrivelse : String,

}

impl Kuffert {
    fn new(ejer_id: u16, beskrivelse: String) -> Self {
        Kuffert{ejer_id, beskrivelse}
    }
}


fn main() -> Result<(), Box<dyn Error>> {

    let mut rejsende1 = Rejsende::new(1, String::from("Chris"), None);
    let mut rejsende2 = Rejsende::new(2, String::from("Nat"), None);
    let mut rejsende3 = Rejsende::new(3, String::from("Jimmy"), None);
    let mut rejsende4 = Rejsende::new(4, String::from("Victoria"), None);

    let kuffert1 = Kuffert::new(rejsende1.id, String::from("Sort Kuffert"));
    let kuffert2 = Kuffert::new(rejsende2.id , String::from("Hvid Kuffert med apple klistermærke"));
    let kuffert3 = Kuffert::new(rejsende3.id, String::from("Brun Lille Kuffert"));
    let kuffert4 = Kuffert::new(rejsende4.id , String::from("Rød Stor Kuffert"));

    rejsende1.kuffert = Some(kuffert1);
    rejsende2.kuffert = Some(kuffert2);
    rejsende3.kuffert = Some(kuffert3);
    rejsende4.kuffert = Some(kuffert4);

    println!("hele rejsnde passeger : {:?}", rejsende1); // testede lige for at se om det virkede, implementede derfor debug

    
    Ok(())
}



// struct CheckinCounter {
//     id: u8,
//     baggage_log: Arc<Mutex<Vec<String>>>,
// }

// impl CheckinCounter {
//     fn new(num_of_counters :Arc<Mutex<u8>>, baggage_log: Arc<Mutex<Vec<String>>>) -> Self {
//         let mut id = *num_of_counters.lock().unwrap();
//         id += 1;
//         CheckinCounter{id, baggage_log}
//     }

//     fn process_baggage(&mut self, baggage : String) -> Result<JoinHandle<()>, Box<dyn Error>> {
//         let mut baggage_log = self.baggage_log.clone();
//         let handle = thread::spawn( move|| {

//             let mut local_lock = baggage_log.lock().unwrap();


//             local_lock.push(baggage.clone());

//             println!("Baggage checked in: {}", baggage);
//             println!("{}", local_lock.join("\n"));
//         });
//         Ok(handle)


//     }
// }


// fn main() -> Result<(), Box<dyn Error>> {

//     let number_of_counters = Arc::new(Mutex::new(0u8));
//         let baggage_log = Arc::new(Mutex::new(Vec::<String>::with_capacity(10)));
//         baggage_log.lock().unwrap().push(String::from("Sko, jakke, pæn"));
//         baggage_log.lock().unwrap().push(String::from("Bent, swagster"));
//         baggage_log.lock().unwrap().push(String::from("Svend, Polo"));
//         let mut counter1:CheckinCounter = CheckinCounter::new(number_of_counters.clone(), baggage_log.clone());
//         let mut counter2:CheckinCounter = CheckinCounter::new(number_of_counters.clone(), baggage_log.clone());
//         counter1.process_baggage(String::from("test1"))?.join();
//         counter2.process_baggage(String::from("test2"))?.join();

//         Ok(())
// }