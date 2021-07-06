use std::{
    sync::mpsc::{channel, Receiver, Sender},
    thread,
};

use ring_a::*;
use ring_b::*;

macro_rules! spawn_ring_b {
    ($ring:ident) => {
        thread::spawn(move || {
            let $ring = $ring.recv();
            println!("{}: {}", stringify!($ring), $ring.get_value());
            let $ring = $ring.send();
            $ring.end();
        })
    };
}

fn main() {
    let (a_sender, b_receiver) = channel::<i32>();
    let (b_sender, c_receiver) = channel::<i32>();
    let (c_sender, a_receiver) = channel::<i32>();

    let a = RingA::<SendA>::new(0, a_sender, a_receiver);
    let b = RingB::<RecvB>::new(b_sender, b_receiver);
    let c = RingB::<RecvB>::new(c_sender, c_receiver);

    vec![
        thread::spawn(move || {
            println!("a: {}", a.get_value());
            let a = a.send();
            let a = a.recv();
            a.end();
        }),
        thread::spawn(move || {
            let b = b.recv();
            println!("b: {}", b.get_value());
            let b = b.send();
            b.end();
        }),
        thread::spawn(move || {
            let c = c.recv();
            println!("c: {}", c.get_value());
            let c = c.send();
            c.end();
        }),
    ]
    .into_iter()
    .map(|handle| handle.join())
    .collect::<Result<_, _>>()
    .unwrap()
}



#[typestate::typestate]
mod ring_a {
    use std::sync::mpsc::{Receiver, Sender};

    #[automaton]
    pub struct RingA {
        pub(crate) send: Sender<i32>,
        pub(crate) receiver: Receiver<i32>,
    }

    #[state]
    pub struct SendA(pub i32);

    pub trait SendA {
        fn new(value: i32, send: Sender<i32>, receiver: Receiver<i32>) -> SendA;
        fn get_value(&self) -> i32;
        fn send(self) -> RecvA;
        fn end(self);
    }

    #[state]
    pub struct RecvA;

    pub trait RecvA {
        fn recv(self) -> SendA;
    }
}

impl SendAState for RingA<SendA> {
    fn new(value: i32, send: Sender<i32>, receiver: Receiver<i32>) -> Self {
        Self {
            send,
            receiver,
            state: SendA(value),
        }
    }

    fn get_value(&self) -> i32 {
        self.state.0
    }

    fn send(self) -> RingA<RecvA> {
        self.send.send(self.state.0).unwrap();
        RingA::<RecvA> {
            send: self.send,
            receiver: self.receiver,
            state: RecvA,
        }
    }

    fn end(self) {}
}

impl RecvAState for RingA<RecvA> {
    fn recv(self) -> RingA<SendA> {
        let value = self.receiver.recv().unwrap();
        RingA::<SendA> {
            send: self.send,
            receiver: self.receiver,
            state: SendA(value),
        }
    }
}

#[typestate::typestate]
mod ring_b {
    use std::sync::mpsc::{Receiver, Sender};
    #[automaton]
    pub struct RingB {
        pub(crate) send: Sender<i32>,
        pub(crate) receiver: Receiver<i32>,
    }

    #[state]
    pub struct SendB(pub i32);

    pub trait SendB {
        fn get_value(&self) -> i32;
        fn send(self) -> RecvB;
    }

    #[state]
    pub struct RecvB;

    pub trait RecvB {
        fn new(send: Sender<i32>, receiver: Receiver<i32>) -> RecvB;
        fn recv(self) -> SendB;
        fn end(self);
    }
}

impl SendBState for RingB<SendB> {
    fn get_value(&self) -> i32 {
        self.state.0
    }

    fn send(self) -> RingB<RecvB> {
        self.send.send(self.state.0).unwrap();
        RingB::<RecvB> {
            send: self.send,
            receiver: self.receiver,
            state: RecvB,
        }
    }
}

impl RecvBState for RingB<RecvB> {
    fn new(send: Sender<i32>, receiver: Receiver<i32>) -> Self {
        Self {
            send,
            receiver,
            state: RecvB,
        }
    }
    fn recv(self) -> RingB<SendB> {
        let value = self.receiver.recv().unwrap();
        RingB::<SendB> {
            send: self.send,
            receiver: self.receiver,
            state: SendB(value),
        }
    }
    fn end(self) {}
}
