use std::sync::{ LazyLock, Once, Mutex };
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

// `INIT` can remain as-is because `Once::new()` is a const fn.
static INIT: Once = Once::new();

// Wrap the initialization in a closure via LazyLock.
static PROGRAM_ID: LazyLock<Mutex<Pubkey>> = LazyLock::new(|| {
    Mutex::new(
        Pubkey::from_str("didso1Dpqpm4CsiCjzP766BGY89CAdD6ZBL68cRhFPc").expect(
            "Invalid default program ID"
        )
    )
});

pub fn initialize_program_id(program_id: &str) {
    INIT.call_once(|| {
        if let Ok(pubkey) = Pubkey::from_str(program_id) {
            let mut writer = PROGRAM_ID.lock().unwrap();
            *writer = pubkey;
        }
    });
}

pub fn get_program_id() -> Pubkey {
    PROGRAM_ID.lock().unwrap().clone()
}

#[cfg(test)]
pub fn reset_program_id() {
    let mut writer = PROGRAM_ID.lock().unwrap();
    *writer = Pubkey::from_str("didso1Dpqpm4CsiCjzP766BGY89CAdD6ZBL68cRhFPc").expect(
        "Invalid default program ID"
    );
}
