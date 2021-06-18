use std::collections::HashMap;

const LIMIT: u32 = 10;

pub struct Auction {
    owner: String,
    bids: HashMap<String, u64>,
    // since we dont have a timer
    // we use a number of possible bids
    remaining_bids: u32,
    highest_bid: u64,
}

impl Auction {
    pub fn new(owner: String) -> Self {
        Self {
            owner,
            bids: HashMap::new(),
            remaining_bids: LIMIT,
            highest_bid: 0,
        }
    }

    pub fn bid(&mut self, client: String, bid: u64) -> Option<()> {
        if self.owner == client {
            return None;
        }
        if !self.has_ended() {
            self.remaining_bids -= 1;
            self.bids.insert(client, bid);
            if self.highest_bid < bid {
                self.highest_bid = bid;
            }
            Some(())
        } else {
            None
        }
    }

    pub fn is_highest_bid(&self, client: &String) -> bool {
        match self.bids.get(client).map(|bid| self.highest_bid == *bid) {
            Some(is_highest) => is_highest,
            None => false,
        }
    }

    pub fn get_bid(&self, client: String) -> Option<&u64> {
        self.bids.get(&client)
    }

    pub fn has_ended(&self) -> bool {
        self.remaining_bids > 0
    }
}
