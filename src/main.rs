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

    /// ## Next Process
    /// 
    /// returns a random process based on their amount of tickets.
    /// Lottery scheduling works by using randomness to find a process, while
    /// the randomness is influenced by their total number of tickets.
    /// 
    /// Higher tickets = higher chances of winning the lottery.
    /// 
    /// ### Pros
    ///     - Easy to implement
    ///     - Lottery scheduling is deterministic
    ///     - Lottery scheduling is fair
    /// 
    /// ### Cons
    ///     - Can slow up if there's a lot of processes
    ///     - Easy to manipulate and have more processes running faster
    fn next_process(&mut self) -> Result<&Process, LotErr> {
        let mut rng = rand::thread_rng();

        if self.processes.is_empty() {
            return Err(LotErr::NoProcesses)
        }

        // generating a random range from 0-total tickets
        // this will be used to determine which process will win
        let winner = rng.gen_range(0..self.get_total_tickets());

        // add the tickets to this number to determine the process that
        // will win
        let mut cumulative_tickets = 0;

        // iterate over the processes
        let mut iter = self.processes.iter();

        // for each process
        for process in &mut iter {
            // we add their tickets
            cumulative_tickets += process.tickets();

            // if we reach the winner, we return that process
            if cumulative_tickets > winner {
                return Ok(process);
            }
        }
        
        /* edge case */
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

    /* a majority of these choices will be process B. Process A will run 2nd most, and C will only run once/twice. */
    /* reference: https://pages.cs.wisc.edu/~remzi/OSTEP/cpu-sched-lottery.pdf */
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
