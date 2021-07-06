mod auction;

use auction::Auction;
use auction_client_api::*;

fn main() {
    let auction = Auction::new("owner".into());
    let client = Client::<AuctionRunning>::start("client".into(), auction);
    match client.has_ended() {
        AuctionState::NoBids(client) => {
            match client.bid(10) {
                BidStatus::HasBidded(client) => todo!(),
                BidStatus::Withdraw(client) => {
                    client.withdraw();
                },
            }
        }
        AuctionState::End(client) => client.end(),
    }
}

#[typestate::typestate]
mod auction_client_api {
    use crate::auction::Auction;

    #[automaton]
    pub struct Client {
        pub(crate) name: String,
        pub(crate) auction: Auction,
    }

    #[state]
    pub struct AuctionRunning;
    pub trait AuctionRunning {
        fn start(name: String, auction: Auction) -> AuctionRunning;
        fn has_ended(self) -> AuctionState;
    }

    pub enum AuctionState {
        NoBids,
        #[metadata(label="bidding closed")]
        End,
    }

    #[state]
    pub struct NoBids;
    pub trait NoBids {
        fn bid(self, bid: u64) -> BidStatus;
    }

    pub enum BidStatus {
        #[metadata(label="bid > previous")]
        HasBidded,
        #[metadata(label="bid <= previous")]
        Withdraw,
    }

    #[state]
    pub struct HasBidded;
    pub trait HasBidded {
        fn check_bid(self) -> BidStatus;
        fn has_ended(self) -> AuctionEnded;
    }

    pub enum AuctionEnded {
        HasBidded,
        CheckWinner,
    }

    #[state]
    pub struct CheckWinner;
    pub trait CheckWinner {
        fn is_highest_bid(self) -> WinnerStatus;
    }

    pub enum WinnerStatus {
        Lost,
        Winner,
    }

    #[state]
    pub struct Withdraw;
    pub trait Withdraw {
        fn withdraw(self) -> AuctionRunning;
    }

    #[state]
    pub struct Lost;
    pub trait Lost {
        fn withdraw(self) -> End;
    }

    #[state]
    pub struct Winner;
    pub trait Winner {
        fn win(self) -> End;
    }

    #[state]
    pub struct End;
    pub trait End {
        fn end(self);
    }
}

impl AuctionRunningState for Client<AuctionRunning> {
    fn start(name: String, auction: Auction) -> Self {
        Self {
            name,
            auction,
            state: AuctionRunning,
        }
    }

    fn has_ended(self) -> AuctionState {
        if self.auction.has_ended() {
            AuctionState::End(Client::<End> {
                name: self.name,
                auction: self.auction,
                state: End,
            })
        } else {
            AuctionState::NoBids(Client::<NoBids> {
                name: self.name,
                auction: self.auction,
                state: NoBids,
            })
        }
    }
}

impl NoBidsState for Client<NoBids> {
    fn bid(mut self, bid: u64) -> BidStatus {
        match self.auction.bid(self.name.clone(), bid) {
            Some(()) => BidStatus::HasBidded(Client::<HasBidded> {
                name: self.name,
                auction: self.auction,
                state: HasBidded,
            }),
            None => BidStatus::Withdraw(Client::<Withdraw> {
                name: self.name,
                auction: self.auction,
                state: Withdraw,
            }),
        }
    }
}

impl HasBiddedState for Client<HasBidded> {
    fn check_bid(self) -> BidStatus {
        if self.auction.is_highest_bid(&self.name) {
            BidStatus::HasBidded(self)
        } else {
            BidStatus::Withdraw(Client::<Withdraw> {
                name: self.name,
                auction: self.auction,
                state: Withdraw,
            })
        }
    }

    fn has_ended(self) -> AuctionEnded {
        if self.auction.has_ended() {
            AuctionEnded::CheckWinner(Client::<CheckWinner> {
                name: self.name,
                auction: self.auction,
                state: CheckWinner,
            })
        } else {
            AuctionEnded::HasBidded(self)
        }
    }
}

impl CheckWinnerState for Client<CheckWinner> {
    fn is_highest_bid(self) -> WinnerStatus {
        if self.auction.is_highest_bid(&self.name) {
            WinnerStatus::Winner(Client::<Winner> {
                name: self.name,
                auction: self.auction,
                state: Winner,
            })
        } else {
            WinnerStatus::Lost(Client::<Lost> {
                name: self.name,
                auction: self.auction,
                state: Lost,
            })
        }
    }
}

impl WithdrawState for Client<Withdraw> {
    fn withdraw(self) -> Client<AuctionRunning> {
        Client::<AuctionRunning> {
            name: self.name,
            auction: self.auction,
            state: AuctionRunning,
        }
    }
}

impl LostState for Client<Lost> {
    fn withdraw(self) -> Client<End> {
        Client::<End> {
            name: self.name,
            auction: self.auction,
            state: End,
        }
    }
}

impl WinnerState for Client<Winner> {
    fn win(self) -> Client<End> {
        Client::<End> {
            name: self.name,
            auction: self.auction,
            state: End,
        }
    }
}

impl EndState for Client<End> {
    fn end(self) {}
}
