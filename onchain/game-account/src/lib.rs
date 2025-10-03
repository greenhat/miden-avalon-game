#![no_std]

extern crate alloc;

#[global_allocator]
static ALLOC: miden::BumpAlloc = miden::BumpAlloc::new();

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

use miden::{component, Felt, StorageMap, StorageMapAccess, Word};

use crate::bindings::exports::miden::game_account::game_account::Guest;

#[component]
struct GameAccount {
    #[storage(slot(0), description = "Player commitments by their IDs")]
    player_commitments: StorageMap,
}

impl Guest for GameAccount {
    fn register_player(id: Felt, commitment: Word) {
        let game_account = GameAccount::default();
        // TODO: it'll work after https://github.com/0xMiden/compiler/issues/673 is implemented
        if game_account.player_commitments.get(&id).is_none() {
            // TODO: it'll work after https://github.com/0xMiden/compiler/issues/672 is implemented
            game_account.player_commitments.set(id, commitment);
        }
    }
}
