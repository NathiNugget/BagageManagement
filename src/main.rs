use core::time;
use std::fmt::format;
use rand::Rng;
use std::error::Error;
use std::fs::{File, FileType};
use std::ops::Index;
use std::sync::{Arc, Mutex};
use std::{result, string, thread, vec};
use std::thread::{sleep, JoinHandle};
use std::io::Write;


#[derive(Debug, Clone)]
struct Lufthavn {
    skranke: Vec<Skranke>,
    flights: Vec<Fly>,
    rejsende: Arc<Mutex<Vec<Rejsende>>>,
    terminal : Arc<Mutex<Vec<Terminal>>>,
    log : Arc<Mutex<Vec<String>>>,
    file: Arc<Mutex<File>>

}

impl Lufthavn {
    fn new() -> Self {
        Lufthavn {
            skranke: Vec::new(),
            flights: Vec::new(),
            rejsende: Arc::new(Mutex::new(Vec::new())),
            terminal: Arc::new(Mutex::new(Vec::new())),
            log : Arc::new(Mutex::new(Vec::new())),
            file: Arc::new(Mutex::new(File::create("flylog.txt").unwrap())),
        }
    }

    fn log(&mut self, msg: String) {
        match self.log.lock() {
            Ok(mut log_vectoren) => {
                log_vectoren.push(msg.clone());
            }
            Err(e) => {
                println!("fail ved lock af log vec")
            }
        }
        match self.file.lock() {
            Ok(mut file) => {
                if let Err(e) = writeln!(file, "{}", msg) {
                    println!("fail ved skriving til fil... {}", e)
                }
            }

            Err(e) => {
                println!("fejl under locking af fil")

            }
        }
        
        println!("logged besked {}", msg);
    }


    fn book(&mut self, rejsende: Rejsende, dest : String) {
        let need_name = rejsende.navn.clone(); // skulle bare se om det her virkede 

        match self.rejsende.lock() {
            Ok(mut rejsende_vec) => {
                rejsende_vec.push(rejsende);
            }
            Err(e)=> {
                println!("rejsende_liste tråden var forgifted !");
                return;
            }
            
        }
        
        match self.log.lock() {
            Ok(mut log_vector) => {
                log_vector.push(format!("booked person {}, til desitination {}", need_name, dest));
            }

            Err(e) => {
                println!("log tråden var forgifted")
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Fly {
    id : u16,
    rejsende : Vec<Rejsende>,
    baggage : Vec<Kuffert>,
}

impl Fly {
    // mangler metoder
    fn new(id : u16) -> Self {
        Fly{id, rejsende: Vec::new(), baggage: Vec::new()}
    
    }

    fn load_passenger(&mut self, rejsende: Rejsende) {
        let rejsende_id = rejsende.id;
        self.rejsende.push(rejsende);
        println!("Rejsende id: {} har nu bordet flyet med id: {}",rejsende_id, self.id)
    }
    
    fn load_baggage(&mut self, baggage: Kuffert) {
        self.baggage.push(baggage);
        println!("baggage er loaded på flyet! Baggage {:?}", self.baggage.last())
    }

    fn depart(&self) {
        println!("Flyet med id: {}, antal Passagerer : {} og antal Baggage {} ", self.id, self.rejsende.len(), self.baggage.len() );
        sleep(time::Duration::from_millis(500));
        println!("så er vi landet")
    }

    fn unload_passengers(&mut self) -> Vec<Rejsende> {
        let passenger_after_landing = std::mem::take(&mut self.rejsende);
        println!("passagerer bliver nu unloaded");

        return  passenger_after_landing;
    }

    fn unload_baggage(&mut self) -> Vec<Kuffert> {
        let baggage_after_landing = std::mem::take(&mut self.baggage);
        println!("baggage er nu unloaded af flyet. Antal Baggage {}", baggage_after_landing.len());

        return baggage_after_landing;
    }
}

#[derive(Debug, Clone)]
struct Skranke {
    id : u16,
    is_busy : bool,
}

impl Skranke {
    
    fn new(id: u16) -> Self {
        Skranke {id, is_busy: false} //
    }
    
    fn load_on_plane(&mut self, baggage: Kuffert, dest : String, fly: &mut Fly) -> () {
        if self.is_busy {
            println!("den her skranke har dsv travlt: {}", self.id);
            return;
        }

        self.is_busy = true;
        let ejer_id_til_baggage =  baggage.ejer_id;
        fly.load_baggage(baggage);
        println!("Baggage med dest: {}, Ejerens id: {}, fly id: {}", dest, ejer_id_til_baggage, fly.id );


        let mut rng = rand::thread_rng();
        let random_delay_for_duration = rng.gen_range(400.. 1600); // simulere virkeligheden i en lufthavn skranke hvor man aflevere baggage noget i den still
        sleep(time::Duration::from_millis(random_delay_for_duration));
        println!("Vente tiden {}, i milisek btw", random_delay_for_duration);
        self.is_busy = false;

    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
struct Terminal {
    id : u16,
    rejsende : Vec<Rejsende>,
    baggage : Vec<Kuffert>
}


impl Terminal {
    fn new(id: u16) -> Self {
        Terminal{id, rejsende: Vec::new(), baggage:Vec::new()}
    }

    fn pickup_baggage(&mut self, rejsende : &Rejsende, baggage: Kuffert) // tænker at vi skal bruge en mutable reference for at få fat i listen med rejsende og ændre den .
    {
        if let Some(i) = self.baggage.iter().position(|x| x.ejer_id == rejsende.id) {
            self.baggage.remove(i);
            println!("baggage taget fra rejsende med id: {}! forsæt god ferie!", rejsende.id)
        }
    }
} 

#[derive(Debug, Clone)]
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
    let lufthavn_1 = Arc::new(Mutex::new(Lufthavn::new()));

    let mut rejsende1 = Rejsende::new(1, String::from("Chris"), None);
    let mut rejsende2 = Rejsende::new(2, String::from("Nat"), None);
    let mut rejsende3 = Rejsende::new(3, String::from("Jimmy"), None);
    let mut rejsende4 = Rejsende::new(4, String::from("Victoria"), None);

    let kuffert1 = Kuffert::new(rejsende1.id, String::from("Sort Kuffert"));
    let kuffert2 = Kuffert::new(rejsende2.id , String::from("Hvid Kuffert med apple klistermærke"));
    let kuffert3 = Kuffert::new(rejsende3.id, String::from("Brun Lille Kuffert"));
    let kuffert4 = Kuffert::new(rejsende4.id , String::from("Rød Stor Kuffert"));

    rejsende1.kuffert = Some(kuffert1.clone());
    rejsende2.kuffert = Some(kuffert2.clone());
    rejsende3.kuffert = Some(kuffert3.clone());
    rejsende4.kuffert = Some(kuffert4.clone());

    // boooking 
    let mut lufthavn = lufthavn_1.lock().unwrap();
    lufthavn.book(rejsende1.clone(), String::from("Germany"));
    lufthavn.book(rejsende2.clone(), String::from("Germany"));
    lufthavn.book(rejsende3.clone(), String::from("Germany"));
    lufthavn.book(rejsende4.clone(), String::from("Germany"));
    lufthavn.flights.push(Fly::new(1));
    drop(lufthavn); // freee em up

    let mut lufthavn = lufthavn_1.lock().unwrap();
    let mut booked_rejsende = lufthavn.rejsende.lock().unwrap();
    let rejsende_to_boarding: Vec<Rejsende> = booked_rejsende.iter().cloned().collect();
    drop(booked_rejsende);
    if let Some(fly) = lufthavn.flights.get_mut(0) {
        for rejsende in rejsende_to_boarding {
            fly.load_passenger(rejsende);
        }
    }
    drop(lufthavn); // freeee em
    
    let lufthavn_clon = Arc::clone(&lufthavn_1); // ref til tråedene
    let mut handles = Vec::new();
    let mut baggage_to_processs = vec![(kuffert1, String::from("Germany")),
    (kuffert2, String::from("Germany")),
    (kuffert3, String::from("Germany")),
    (kuffert4, String::from("Germany"))];

    for i in 0..2 {
        let lufthavn_clone = Arc::clone(&lufthavn_clon);
        let assigned_baggage: Vec<(Kuffert, String)> = if i == 1 {
            vec![baggage_to_processs.remove(0), baggage_to_processs.remove(0)]
        } else {
            vec![baggage_to_processs.remove(0), baggage_to_processs.remove(0)]
        };
        let handle = thread::spawn(move || {

            let mut skranke = Skranke::new(i);
            let mut lufthavn = lufthavn_clone.lock().unwrap();
            
            if let Some(fly) = lufthavn.flights.get_mut(0) {
                for (baggage, dest ) in assigned_baggage {
                    skranke.load_on_plane(baggage, dest, fly);
                }
            }

        });

        handles.push(handle);
        let mut lufthavn = lufthavn_1.lock().unwrap();

        lufthavn.skranke.push(Skranke::new(i));
        
        
    }

    for handle in handles {
            handle.join().unwrap()
    }

    let mut lufthavn = lufthavn_1.lock().unwrap();
    if let Some(fly) = lufthavn.flights.get_mut(0) {
        fly.depart();
    }
    // landet igen


    //terminal håndtering, få rejsende af flyet og baggage.

    println!("{}", lufthavn.skranke.len());


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