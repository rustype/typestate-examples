use reader::reader_api::{
    Authenticated, AuthenticatedState, CardPresent, CardPresentState, Reader, Start, StartState,
};

mod card;
mod reader;

fn main() {
    let reader = Reader::<Start>::start();
    match reader.check_for_card() {
        reader::reader_api::CheckCardResult::CardPresent(reader) => request_pin(reader),
        reader::reader_api::CheckCardResult::Error(err) => eprintln!("{}", err.state.message),
    }
}

fn request_pin(reader: Reader<CardPresent>) {
    println!("please insert the card pin:");

    let stdin = std::io::stdin();
    let mut pin_buffer = String::with_capacity(1024);
    stdin.read_line(&mut pin_buffer).expect("failed to read from stdin");

    let pin_entries = pin_buffer
        .split_ascii_whitespace()
        .map(|n| n.parse::<u8>())
        .collect::<Result<Vec<_>, _>>();

    match pin_entries {
        Ok(pin_entries) => {
            if pin_entries.len() == 4 && pin_entries.iter().all(|n| *n < 10) {
                let pin: [u8; 4] = [
                    pin_entries[0],
                    pin_entries[1],
                    pin_entries[2],
                    pin_entries[3],
                ];
                match reader.authenticate(pin) {
                    reader::reader_api::AuthResult::Authenticated(reader) => {
                        post_auth(reader);
                    }
                    reader::reader_api::AuthResult::Error(err) => eprintln!("{}", err.state.message),
                }
            } else {
                eprintln!("invalid pin");
            }
        }
        Err(err) => eprintln!("{}", err),
    }
}

fn post_auth(reader: Reader<Authenticated>) {
    println!("authenticated");
    reader.end();
}
