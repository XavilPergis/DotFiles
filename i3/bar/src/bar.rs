use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::io::prelude::*;
use std::process::{ Command, Stdio, Child };
use std::sync::mpsc::*;
use std::thread;

use format::*;
use pair::ChannelPair;

pub struct Lemonbar<'a> {
    process: Child,
    chan: ChannelPair<Segment>,
    segments: HashMap<&'a str, SyncSender<()>>,
    segment_order: Vec<&'a str>
}

pub enum BarPosition {
    Top,
    Bottom
}

impl<'a> Lemonbar<'a> {
    #[inline]
    pub fn new(bar_pos: BarPosition) -> Lemonbar<'a> {
        // What a hot mess...

        let pos_arg = match bar_pos {
            BarPosition::Top => "",
            BarPosition::Bottom => "-b"
        };

        // Spawn our lemonbar process.
        // TODO: Configurable arguments.
        // TODO: Possibly pipe stdout through our program?
        let process = match Command::new("lemonbar")
                                    .args(&[pos_arg, "-p", "-u", "7", "-f", "RobotoMono Nerd Font:style=Regular:size=10"])
                                    .stdin(Stdio::piped())
                                    .stdout(Stdio::inherit())
                                    .spawn() {
            Err(why) => panic!("Failed to run lemonbar. {}", why.description()),
            Ok(process) => process
        };

        Lemonbar {
            segments: HashMap::new(),
            process: process,
            chan: ChannelPair::new(0),
            segment_order: Vec::new()
        }
    }

    pub fn set_order(&mut self, order: Vec<&'a str>) {
        self.segment_order = order;
    }

    // TODO: Possibly add some setup that only runs once?
    pub fn add_segment<F>(&mut self, name: &'a str, update_fn: Box<F>) where F: Fn(SyncSender<Segment>) + Send + 'static {
        // Copy our sender to move into the thread
        let local_tx = self.chan.tx.clone();
        let local_name = name.to_string();
        let (lock_tx, lock_rx) = sync_channel::<()>(0);

        // Insert our lock into a map so we can tell the thread when to execute
        self.segments.insert(name, lock_tx);

        // Just keep on calling the update function...
        // This would completely break if we didn't use a `SyncSender`
        thread::spawn(move || {
            io::stderr().write(format!("Starting Thread: {}\n", local_name).as_bytes()).unwrap();
            loop {
                // Block until the bar wants its data
                lock_rx.recv().expect(format!("Thread {} failed", local_name).as_str());
                update_fn(local_tx.clone());
            }
        });
    }

    pub fn get_bar_string(&mut self) -> String {
        let mut string_builder = String::new();

        let ref mut seg_order = self.segment_order;

        // Iterate our segment names *in order* and unblock each thread in serial
        for segment_name in seg_order.iter() {
            let tx = self.segments.get(segment_name).unwrap();

            // Unblock the thread and wait for data
            tx.send(()).expect(format!("Failed to unlock thread {}", segment_name).as_str());
            let seg_string = self.chan.rx.recv().unwrap().process_segment();

            string_builder += seg_string.as_str();
        }

        // Return the Finished string plus a color to reset the background to
        string_builder + "%{B#073642}\n"
    }

    #[inline]
    pub fn write(&mut self, data: String) -> io::Result<usize> {
        self.process.stdin.as_mut().unwrap().write(data.as_bytes())
    }
}
