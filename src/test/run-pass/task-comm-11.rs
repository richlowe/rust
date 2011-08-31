use std;
import std::comm;
import std::task;

fn start(c: comm::chan<comm::chan<int>>) {
    let p: comm::port<int> = comm::port();
    comm::send(c, comm::chan(p));
}

fn main() {
    let p = comm::port();
    let child = task::spawn(bind start(comm::chan(p)));
    let c = comm::recv(p);
}
