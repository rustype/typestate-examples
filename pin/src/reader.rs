use rand::random;

#[typestate::typestate]
pub mod reader_api {

    use crate::card::card_api;

    #[automaton]
    pub struct Reader;

    #[state]
    pub struct Start;

    pub trait Start {
        fn start() -> Start;
        /// Check if a card is inserted in the reader.
        ///
        /// As this a "physical" API, we emulate it by generating a random number and
        /// checking for a given probability.
        fn check_for_card(self) -> CheckCardResult;
    }

    #[state]
    pub struct CardPresent {
        pub card: card_api::Card<card_api::Start>,
    }

    pub trait CardPresent {
        fn authenticate(self, pin: [u8; 4]) -> AuthResult;
    }

    #[state]
    pub struct Authenticated {
        pub card: card_api::Card<card_api::Authenticated>,
    }

    pub trait Authenticated {
        fn browse(&self);
        fn end(self);
    }

    #[state]
    pub struct Error {
        pub message: String,
    }

    pub trait Error {
        fn end(self);
    }

    pub enum CheckCardResult {
        CardPresent,
        Error,
    }

    pub enum AuthResult {
        Authenticated,
        Error,
    }
}

use reader_api::*;

use crate::card::card_api;

impl StartState for Reader<Start> {
    fn start() -> Self {
        Self { state: Start }
    }

    fn check_for_card(self) -> CheckCardResult {
        // 0.5 chance
        use crate::card::card_api::StartState;
        if random::<bool>() {
            CheckCardResult::CardPresent(Reader::<CardPresent> {
                state: CardPresent {
                    card: card_api::Card::<card_api::Start>::new(),
                },
            })
        } else {
            CheckCardResult::Error(Reader::<Error> {
                state: Error {
                    message: "card has not been inserted".into(),
                },
            })
        }
    }
}

impl CardPresentState for Reader<CardPresent> {
    fn authenticate(self, pin: [u8; 4]) -> AuthResult {
        // to avoid name clashing we import inside the function
        use crate::card::card_api::StartState;
        let card = self.state.card;
        match card.perform_authentication(pin) {
            card_api::AuthResult::Authenticated(authenticated) => {
                AuthResult::Authenticated(Reader::<Authenticated> {
                    state: Authenticated {
                        card: authenticated,
                    },
                })
            }
            card_api::AuthResult::Blocked(blocked) => AuthResult::Error(Reader::<Error> {
                state: Error {
                    message: "card is blocked".into(),
                },
            }),
            card_api::AuthResult::Error(error) => AuthResult::Error(Reader::<Error> {
                state: Error {
                    message: "wrong pin".into(),
                },
            }),
        }
    }
}

impl AuthenticatedState for Reader<Authenticated> {
    fn browse(&self) {}
    fn end(self) {}
}

impl ErrorState for Reader<Error> {
    fn end(self) {}
}
