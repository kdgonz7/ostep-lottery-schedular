// implements a lottery schedular

use rand::Rng;

#[derive(Debug)]
pub enum LotErr {
    NoProcesses,
}

struct Process {
    tickets: i32,
    name: String,
}

impl Process {
    fn new(tickets: i32, name: &str) -> Self {
        let new_name = String::from(name);
        Self { tickets, name: new_name }
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn tickets(&self) -> &i32 { &self.tickets }
}

struct Lottery {
    processes: Vec<Process>,
}

impl Lottery {
    fn new() -> Lottery {
        Lottery { processes: Vec::new() }
    }

    fn add_process(&mut self, process: Process) {
        self.processes.push(process);
    }

    fn get_total_tickets(&self) -> i32 {
        let mut total = 0;
        for process in &self.processes {
            total += process.tickets();
        }
        total
    }

    fn next_process(&mut self) -> Result<&Process, LotErr> {
        let mut rng = rand::thread_rng();
        if self.processes.is_empty() {
            return Err(LotErr::NoProcesses)
        }

        let winner = rng.gen_range(0..self.get_total_tickets());
        let mut counter = 0;

        let mut iter = self.processes.iter();

        for process in &mut iter {
            counter += process.tickets();

            if counter > winner {
                return Ok(process);
            }
        }
        
        Err(LotErr::NoProcesses)

    }

    fn ended(&self) -> bool {
        self.processes.is_empty()
    }
}

fn main() {
    let mut lot: Lottery = Lottery::new();

    let proc1 = Process::new(100, "A");
    let proc2 = Process::new(500, "B");
    let proc3 = Process::new(70, "C");

    lot.add_process(proc1);
    lot.add_process(proc2);
    lot.add_process(proc3);

    for _ in 0..15 {
        if lot.ended() {
            break;
        }

        let proc = lot.next_process();

        match proc {
            Ok(process) => {
                println!("{}\t{}", process.name(), process.tickets());
            }
            Err(err) => {
                println!("error: {:?}", err);
            }
        }
    }
}
