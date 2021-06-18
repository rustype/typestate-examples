type state = {
    auctionEnded : int;
    address : int;
    bid : int;
    highestBidder : int;
    highestBid : int
}

type store = (int, int) map

type return = operation list * store

let owner : int = 0

let init (state, store : state * store) : return =
  let no_op : operation list = [] in
  if state.address <> 0 then
    (failwith ("Not the owner of the contract.") : operation list * store) // Annotation
  else if state.auctionEnded = 0 then
    (failwith("The auction is already running") : return)
  else (([] : operation list), store)

let finish (state, store : state * store) : return =
  let no_op : operation list = [] in
  let cond : bool = (state.address = 0 && state.auctionEnded = 0) in
  if cond then
    //let state.auctionEnded = 1 in
    (([] : operation list), store)
  else
    (failwith ("Cannot terminate auction") : return)

let bid (state, store : state * store) : return =
  let no_op : operation list = [] in
 // let b: int = 1 in
  let b: int option = Map.find_opt state.address store in
  let hbid: int = state.highestBid in
  if state.address = 0 then
    (failwith ("Owner cannot bid") : return)
  else if state.address = 1 then
    (failwith ("Auction is over") : return)
  else if state.address = state.highestBidder then
    (failwith ("Client is already the highest bidder") : return)
  else if b > 0 then
    (failwith ("Already bid, need to withdraw first") : return)
  else if state.bid <= 0 then
    (failwith ("Value of bid needs to be greater than 0") : return)
  else
    let s: store = Map.update state.address (Some state.bid) store in
    (([] : operation list), s)

let withdraw (state, store : state * store) : return =
  let b: int option = Map.find_opt state.address store in
  if state.address = 0 then
    (failwith ("Owner cannot withdraw") : return)
  else if state.address = state.highestBidder then
    (failwith ("Client is the highest bidder, cannot withdraw") : return)
  else if b > 0 then
    (failwith ("Client has no pending returns") : return)
  else
    let s: store = Map.update state.address (Some 0) store in
    (([] : operation list), s)


let win (state, store : state * store) : return =
  let b: int option = Map.find_opt state.address store in
  if state.address = 0 then
    (failwith ("Owner cannot win") : return)
  else if state.auctionEnded = 0 then
    (failwith ("Auction is still running") : return)
  else if state.address <> state.highestBidder then
    (failwith ("Isnt the winner of the auction") : return)
  else if b = 0 then
    (failwith ("Prize was already collected") : return)
  else
    let s: store = Map.update state.address (Some 0) store in
    (([] : operation list), s)
