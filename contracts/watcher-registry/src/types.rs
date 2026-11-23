use soroban_sdk::{contracterror, contracttype, Address};

// ── Errors ────────────────────────────────────────────────────────────────────

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum ContractError {
    AlreadyInitialized = 1,
    Unauthorized = 2,
    NotInitialized = 3,
    /// Returned when trying to remove the last admin, which would lock the contract.
    LastAdmin = 4,
    /// Returned when the specified watcher is not currently registered.
    WatcherNotFound = 5,
}

// ── Storage keys ─────────────────────────────────────────────────────────────

/// Storage key variants used to address instance entries.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    /// Stores the `Vec<Address>` of current admins.
    Admins,
    /// Stores the `Vec<Address>` of authorized watcher nodes.
    Watchers,
    /// Stores the running count of authorized watchers.
    WatcherCount,
    /// Stores the `WatcherInfo` for a specific watcher address.
    WatcherInfo(Address),
}

// ── Data structures ──────────────────────────────────────────────────────────

/// Information associated with a registered watcher.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WatcherInfo {
    /// The watcher's address.
    pub address: Address,
    /// Ledger sequence at which the watcher's registration expires.
    pub expiry: u32,
}