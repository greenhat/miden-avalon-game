#![no_std]

extern crate alloc;

#[global_allocator]
static ALLOC: miden::BumpAlloc = miden::BumpAlloc::new();

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

use alloc::{collections::btree_map::BTreeMap, vec::Vec};
use miden::{component, Felt, StorageMap, StorageMapAccess, Value, ValueAccess, Word};

use crate::bindings::exports::miden::game_account::game_account::Guest;

type PlayerCommitment = Word;

enum GameState {
    None,
    TeamProposal,
    MissionExecution,
}

enum TeamProposalVote {
    Approve,
    Reject,
}

enum PlayerRole {
    Knight,
    Traitor,
}

#[component]
struct GameAccount {
    #[storage(slot(0), description = "game state", type = "word")]
    game_state: Value,

    #[storage(slot(1), description = "Player commitments")]
    player_commits: StorageMap,

    #[storage(
        slot(2),
        description = "Team proposal with player NFT by their commitments"
    )]
    team_proposal: StorageMap,

    #[storage(slot(3), description = "Player votes by their commitments")]
    team_proposal_votes: StorageMap,
}

impl Guest for GameAccount {
    /// Register a player in the game
    fn register_player(commit: PlayerCommitment) {
        let game_account = GameAccount::default();
        // TODO: it'll work after https://github.com/0xMiden/compiler/issues/673 is implemented
        if game_account.player_commits.get(&commit).is_none() {
            // only unregistered players can register
            // TODO: it'll work after https://github.com/0xMiden/compiler/issues/672 is implemented
            game_account.player_commits.set(commit, Word::ZERO);
        }
    }

    /// Propose a team to be voted
    fn propose_team(players: Vec<PlayerCommitment>) {
        let game_account = GameAccount::default();
        let game_state: GameState = game_account.game_state.read();
        let game_state = GameState::None else {
            panic!("Invalid game state. Expected None");
        };
        let registerer_players = players
            .into_iter()
            .map(|p| game_account.player_commits.get(&p).is_some())
            .collect();
        let roles = assign_roles(registerer_players);
        for (player, role) in roles {
            game_account.team_proposal.set(player, role);
        }

        todo!("Creates NFTs with a role as a private notes for all the registered players");

        // Creates a team proposal to be voted by the players
        // The players are expected to approve or reject the proposal via a network note
        game_account.game_state.write(GameState::TeamProposal);
    }

    /// Player vote on the team proposal
    fn vote_team_proposal(commit: PlayerCommitment, vote: Felt) {
        let game_account = GameAccount::default();
        let game_state: GameState = game_account.game_state.read();
        let game_state = GameState::TeamProposal else {
            panic!("Invalid game state. Expected TeamProposal");
        };
        let vote: TeamProposalVote = vote.into();
        if let Some(nft) = game_account.team_proposal.get(commit)() {
            // TODO: check the NFT that allows to vote
            game_account.team_proposal_votes.set(commit, vote);
        }
        // TODO: Check if all voted and start the mission
    }
}

fn assign_roles(players: Vec<PlayerCommitment>) -> BTreeMap<PlayerCommitment, PlayerRole> {
    let roles = BTreeMap::new();
    for player_commit in players {
        let role = todo!("get a role from the oracle via FPI");
        roles.insert(player_commit, role);
    }
    roles
}
