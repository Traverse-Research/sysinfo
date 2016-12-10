// 
// Sysinfo
// 
// Copyright (c) 2015 Guillaume Gomez
//

use std::fmt::{self, Formatter, Debug};
use libc::{c_int};
use sys::ffi;

#[derive(Clone)]
pub struct Process {
    /// name of the program
    pub name: String,
    /// command line, split into arguments
    pub cmd: Vec<String>,
    /// path to the executable
    pub exe: String,
    /// pid of the process
    pub pid: i64,
    /// pid of the parent process
    pub parent: Option<i64>,
    /// environment of the process
    pub environ: Vec<String>,
    /// current working directory
    pub cwd: String,
    /// path of the root directory
    pub root: String,
    /// memory usage (in kB)
    pub memory: u64,
    utime: u64,
    stime: u64,
    old_utime: u64,
    old_stime: u64,
    /// time of process launch (in seconds)
    pub start_time: u64,
    updated: bool,
    /// total cpu usage
    pub cpu_usage: f32,
    /// user id of the process owner
    pub uid: i64,
    /// group id of the process owner
    pub gid: i64,
}

impl Process {
    pub fn new(pid: i64, parent: Option<i64>, start_time: u64) -> Process {
        Process {
            name: String::new(),
            pid: pid,
            parent: parent,
            cmd: Vec::new(),
            environ: Vec::new(),
            exe: String::new(),
            cwd: String::new(),
            root: String::new(),
            memory: 0,
            cpu_usage: 0.,
            utime: 0,
            stime: 0,
            old_utime: 0,
            old_stime: 0,
            updated: true,
            start_time: start_time,
            uid: 0,
            gid: 0,
        }
    }

    pub fn kill(&self, signal: ::Signal) -> bool {
        unsafe { ffi::kill(self.pid as c_int, signal as c_int) == 0 }
    }
}

#[allow(unused_must_use)]
impl Debug for Process {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "pid: {}", self.pid);
        writeln!(f, "parent: {:?}", self.parent);
        writeln!(f, "name: {}", self.name);
        writeln!(f, "environment:");
        for var in self.environ.iter() {
        if var.len() > 0 {
                writeln!(f, "\t{}", var);
            }
        }
        writeln!(f, "command:");
        for arg in &self.cmd {
            writeln!(f, "\t{}", arg);
        }
        writeln!(f, "executable path: {}", self.exe);
        writeln!(f, "current working directory: {}", self.cwd);
        writeln!(f, "owner/group: {}:{}", self.uid, self.gid);
        writeln!(f, "memory usage: {} kB", self.memory);
        writeln!(f, "cpu usage: {}%", self.cpu_usage);
        write!(f, "root path: {}", self.root)
    }
}

pub fn compute_cpu_usage(p: &mut Process, time: u64, task_time: u64) {
    let system_time_delta = task_time - p.old_utime;
    let time_delta = time - p.old_stime;
    p.old_utime = task_time;
    p.old_stime = time;
    p.cpu_usage = if time_delta == 0 {
        0f32
    } else {
        (system_time_delta as f64 * 100f64 / time_delta as f64) as f32
    };
    if p.cpu_usage > 100f32 {
        p.cpu_usage = 100f32;
    }
}

// COMMON PART
//
// Need to be moved into a "common" file to avoid duplication.

pub fn set_time(p: &mut Process, utime: u64, stime: u64) {
    p.old_utime = p.utime;
    p.old_stime = p.stime;
    p.utime = utime;
    p.stime = stime;
    p.updated = true;
}

pub fn has_been_updated(p: &Process) -> bool {
    p.updated
}
