use libproc::libproc::proc_pid;
use std::process::Command;
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::{thread, time};

const LISTPID_INTERVAL: time::Duration = time::Duration::from_millis(1000);
const KILL_INTERVAL: time::Duration = time::Duration::from_millis(500);

// pub struct Proc {
//     pid: u32,
// }

pub type Proc = u32;
pub fn get_procs() -> Vec<Proc> {
    if let Ok(pids) = proc_pid::listpids(proc_pid::ProcType::ProcAllPIDS) {
        println!("Found {} processes using listpids()", pids.len());
        return pids;
    } else {
        return Vec::new();
    }
}

pub struct ProcManager {
    pub procs: Vec<Proc>,
    proc_rx: Receiver<Vec<Proc>>,
    kill_tx: Sender<u32>,
}

impl ProcManager {
    pub fn new(rx: Receiver<Vec<u32>>, tx: Sender<u32>) -> ProcManager {
        ProcManager {
            procs: get_procs(),
            proc_rx: rx,
            kill_tx: tx,
        }
    }

    pub fn poll_pids(&mut self) {
        if let Ok(procs) = self.proc_rx.try_recv() {
            println!("GOT NEW PIDS");
            self.procs = procs;
        } else {
            // something
        }
    }

    pub fn kill_pid(&mut self, pid: u32) {
        if self.kill_tx.send(pid).is_ok() {
            println!("SENT KILL {}", pid)
        } else {
            // error
        }
    }
}

pub fn get_proc_manager() -> ProcManager {
    let (tx1, rx1) = mpsc::channel();

    // pid monitoring thread
    thread::spawn(move || loop {
        let procs = get_procs();
        println!("GETTING PIDS");
        if tx1.send(procs).is_ok() {
            thread::sleep(LISTPID_INTERVAL);
        } else {
            println!("EXITING PID THREAD");
            break;
        }
    });

    // proc killing thread
    let (tx2, rx2) = mpsc::channel::<u32>();
    thread::spawn(move || loop {
        match rx2.try_recv() {
            Ok(pid) => {
                if let Ok(child) = Command::new("echo").arg(pid.to_string()).spawn() {
                    println!("Child's ID is {}", child.id());
                } else {
                    println!("command didn't start");
                }
            }
            Err(TryRecvError::Disconnected) => {
                println!("Terminating.");
                break;
            }
            Err(TryRecvError::Empty) => {}
        }

        thread::sleep(KILL_INTERVAL);
    });

    ProcManager::new(rx1, tx2)
}

/*
use std::process::Command;

Command::new("ls")
        .arg("-l")
        .arg("-a")
        .spawn()
        .expect("ls command failed to start");
*/
