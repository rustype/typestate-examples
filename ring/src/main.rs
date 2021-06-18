fn main() {
    let a = RingA::<SendA>::new(0);
    let b = RingB::<RecvB>::new();
    let c = RingB::<RecvB>::new();
    let v = a.get_value();
    let a = a.send();
    let b = b.recv(v);
    let v = b.get_value();
    let b = b.send();
    b.end();
    let c = c.recv(v);
    let v = c.get_value();
    let c = c.send();
    let a = a.recv(v);
    c.end();
    a.end();
}

use ring_a::*;
use ring_b::*;

#[typestate::typestate]
mod ring_a {
    #[automaton]
    pub struct RingA;

    #[state]
    pub struct SendA(pub i32);

    pub trait SendA {
        fn new(value: i32) -> SendA;
        fn get_value(&self) -> i32;
        fn send(self) -> RecvA;
        fn end(self);
    }

    #[state]
    pub struct RecvA;

    pub trait RecvA {
        fn recv(self, value: i32) -> SendA;
    }
}

impl SendAState for RingA<SendA> {
    fn new(value: i32) -> Self {
        Self {
            state: SendA(value),
        }
    }

    fn get_value(&self) -> i32 {
        self.state.0
    }

    fn send(self) -> RingA<RecvA> {
        RingA::<RecvA> { state: RecvA }
    }

    fn end(self) {}
}

impl RecvAState for RingA<RecvA> {
    fn recv(self, value: i32) -> RingA<SendA> {
        RingA::<SendA> {
            state: SendA(value),
        }
    }


}

#[typestate::typestate]
mod ring_b {
    #[automaton]
    pub struct RingB;

    #[state]
    pub struct SendB(pub i32);

    pub trait SendB {
        fn get_value(&self) -> i32;
        fn send(self) -> RecvB;
    }

    #[state]
    pub struct RecvB;

    pub trait RecvB {
        fn new() -> RecvB;
        fn recv(self, value: i32) -> SendB;
        fn end(self);
    }
}

impl SendBState for RingB<SendB> {
    fn get_value(&self) -> i32 {
        self.state.0
    }

    fn send(self) -> RingB<RecvB> {
        RingB::<RecvB> { state: RecvB }
    }
}

impl RecvBState for RingB<RecvB> {
    fn new() -> Self {
        Self { state: RecvB }
    }
    fn recv(self, value: i32) -> RingB<SendB> {
        RingB::<SendB> {
            state: SendB(value),
        }
    }
    fn end(self) {}
}
