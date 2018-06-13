#[macro_use]
extern crate chan;
extern crate chan_signal;
extern crate os_pipe;

use std::io::Write;
use std::process::Command;
use std::process::Stdio;
use std::collections::VecDeque;
use std::thread;
use std::env;

use chan_signal::Signal;
use os_pipe::{pipe, IntoStdio};

fn main() {
    if env::args().count() < 2 {
        eprintln!("At least one argument required (try ./mc-wrap java -jar server.jar)");
        std::process::exit(1);
    }

    let signal = chan_signal::notify(&[Signal::INT, Signal::TERM]);
    let (sdone, rdone) = chan::sync(0);
    let (stdout, mut stdin) = pipe().unwrap();

    thread::spawn(move || run(sdone, stdout));

    loop {
        chan_select! {
            signal.recv() -> signal => {
                println!("mc-wrap: Detected SIG{:?} - Shutting down Minecraft server...", signal.unwrap());
                stdin.write(b"stop\n").unwrap();
            },
            rdone.recv() => {
                println!("mc-wrap: Detected Minecraft server shutdown");
                break;
            }
        }
    }
}

fn run(_sdone: chan::Sender<()>, stdout: os_pipe::PipeReader) {
    let mut args: VecDeque<String> = env::args().collect();
    args.pop_front();

    Command::new(args.pop_front().unwrap())
        .args(args)
        .stdin(stdout.into_stdio())
        .stdout(Stdio::inherit())
        .spawn()
        .expect("Failed to spawn child")
        .wait()
        .expect("Failed to wait on child");
}
