use std::collections::HashMap;

pub struct Auction {
    owner: String,
    bids: HashMap<String, u64>,
    highest_bid: u64,
    ended: bool,
}

impl Auction {
    pub fn new(owner: String) -> Self {
        Self {
            owner,
            bids: HashMap::new(),
            highest_bid: 0,
            ended: false,
        }
    }

    pub fn bid(&mut self, client: String, bid: u64) -> Option<()> {
        if self.owner == client {
            return None;
        }
        if !self.has_ended() {
            self.ended = rand::random();
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
        self.ended
    }
}
