#![no_std]

extern crate alloc;

#[global_allocator]
static ALLOC: miden::BumpAlloc = miden::BumpAlloc::new();

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

use crate::bindings::exports::miden::player_account::player_account::Guest;
use miden::{component, Felt, Value};

#[component]
struct PlayerAccount {
    #[storage(
        slot(0),
        description = "owner's public key",
        type = "auth::rpo_falcon512::pub_key"
    )]
    owner_public_key: Value,

    #[storage(
        slot(1),
        description = "player secret",
        type = "auth::rpo_falcon512::pub_key"
    )]
    player_secret: Value,
}

impl Guest for PlayerAccount {
    fn add(a: Felt, b: Felt) -> Felt {
        a + b
    }
}
