#[typestate::typestate]
pub mod card_api {
    #[automaton]
    pub struct Card {
        pub valid_pin: [u8; 4],
        pub attempts_left: u8,
    }

    #[state]
    pub struct Start;

    pub trait Start {
        fn new() -> Start;
        fn perform_authentication(self, pin: [u8; 4]) -> AuthResult;
        fn disconnect(self);
    }

    #[state]
    pub struct Authenticated;

    pub trait Authenticated {
        fn browse(&self);
        fn disconnect(self);
    }

    #[state]
    pub struct Error;

    pub trait Error {
        fn retry(self) -> Start;
        fn disconnect(self);
    }

    #[state]
    pub struct Blocked;
    pub trait Blocked {
        fn disconnect(self);
    }

    pub enum AuthResult {
        Authenticated,
        Blocked,
        Error,
    }
}

use card_api::*;

impl StartState for Card<Start> {
    fn new() -> Self {
        Self {
            valid_pin: [0, 0, 0, 0],
            attempts_left: 3,
            state: Start,
        }
    }

    fn perform_authentication(self, pin: [u8; 4]) -> AuthResult {
        if self.attempts_left > 0 {
            if self.valid_pin == pin {
                AuthResult::Authenticated(Card::<Authenticated> {
                    attempts_left: self.attempts_left,
                    valid_pin: self.valid_pin,
                    state: Authenticated,
                })
            } else {
                AuthResult::Error(Card::<Error> {
                    attempts_left: self.attempts_left - 1,
                    valid_pin: self.valid_pin,
                    state: Error,
                })
            }
        } else {
            AuthResult::Blocked(Card::<Blocked> {
                attempts_left: self.attempts_left,
                valid_pin: self.valid_pin,
                state: Blocked,
            })
        }
    }

    fn disconnect(self) {}
}

impl AuthenticatedState for Card<Authenticated> {
    fn browse(&self) {}
    fn disconnect(self) {}
}

impl ErrorState for Card<Error> {
    fn retry(self) -> Card<Start> {
        Card::<Start> {
            attempts_left: self.attempts_left,
            valid_pin: self.valid_pin,
            state: Start,
        }
    }
    fn disconnect(self) {}
}

impl BlockedState for Card<Blocked> {
    fn disconnect(self) {}
}
