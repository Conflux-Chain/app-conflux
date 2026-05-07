pub const ADDRRESS_BYTES_LEN: usize = 20;

pub const HASH_BYTES_LEN: usize = 32;

/**
 * Maximum transaction length (bytes).
 */
pub const MAX_TRANSACTION_LEN: usize = 765;

/**
 * Exponent used to convert Drip to CFX unit (N CFX = N * 10^18 Drip).
 */
pub const EXPONENT_SMALLEST_UNIT: usize = 18;

/**
 * Application flags.
 */
pub const APP_FLAG_BLIND_SIGNING_ENABLED: u8 = 0x01;

pub const APP_FLAG_DETAILED_DISPLAY_ENABLED: u8 = 0x02;

/**
 * One CFX that can be paid for storage.
 */
pub const STORAGE_OF_ONE_CFX: u64 = 1024;
