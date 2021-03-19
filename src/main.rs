mod app;
mod config;
mod imap;
mod input;
mod mbox;
mod msg;
mod output;
mod smtp;
mod table;
mod flag {
    pub(crate) mod cli;
}

use crate::app::App;

fn main() {
    if let Err(ref errs) = App::new().run() {
        let mut errs = errs.iter();
        match errs.next() {
            Some(err) => {
                eprintln!("{}", err);
                errs.for_each(|err| eprintln!(" ↳ {}", err));
            }
            None => (),
        }
    }
}
