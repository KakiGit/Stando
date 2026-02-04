#[cfg(test)]
use std::sync::Mutex;

#[cfg(test)]
pub static ENV_LOCK: Mutex<()> = Mutex::new(());
