#![no_std]

extern crate alloc;

#[global_allocator]
static ALLOC: miden::BumpAlloc = miden::BumpAlloc::new();

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

mod bindings;

use bindings::exports::miden::game_account::game_account::Guest;
use miden::{component, Felt, StorageMap, StorageMapAccess, Word};

#[component]
struct GameAccount {
    #[storage(slot(0), description = "test map")]
    player_commitments: StorageMap,
}

bindings::export!(GameAccount with_types_in bindings);

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
