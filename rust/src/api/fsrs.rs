pub use fsrs::{FSRS, MemoryState, ItemState};

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}

// Define CardStatus enum
#[derive(Debug, Clone, Copy)]
pub enum CardStatus {
    Again,
    Hard,
    Good,
    Easy,
}

#[flutter_rust_bridge::frb(mirror(ItemState))]
pub struct _ItemState {
    pub memory: MemoryState,
    pub interval: f32,
}

#[flutter_rust_bridge::frb(mirror(MemoryState))]
pub struct _MemoryState {
    pub stability: f32,
    pub difficulty: f32,
}


#[flutter_rust_bridge::frb(sync)]
pub fn next_card(
    current_memory_state: Option<MemoryState>,
    days_elapsed: u32,
    status: CardStatus,
) -> ItemState {
    let desired_retention = 0.9;
    let fsrs = FSRS::new(Some(&[])).unwrap(); // Using default parameters

    let states = fsrs.next_states(current_memory_state, desired_retention, days_elapsed).unwrap();
    
    match status {
        CardStatus::Again => states.again,
        CardStatus::Hard => states.hard,
        CardStatus::Good => states.good,
        CardStatus::Easy => states.easy,
    }
}