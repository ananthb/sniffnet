use crate::enums::sound::Sound;
use serde::{Deserialize, Serialize};

/// Used to contain the notifications configuration set by the user
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Debug, Copy)]
pub struct Notifications {
    pub volume: u8,
    pub packets_notification: ThresholdNotification,
    pub bytes_notification: ThresholdNotification,
    pub favorite_notification: FavoriteNotification,
}

impl Default for Notifications {
    fn default() -> Self {
        Notifications {
            volume: 60,
            packets_notification: ThresholdNotification {
                threshold: None,
                sound: Sound::Gulp,
                previous_threshold: 500,
            },
            bytes_notification: ThresholdNotification {
                threshold: None,
                sound: Sound::Pop,
                previous_threshold: 5_000_000,
            },
            favorite_notification: FavoriteNotification {
                notify_on_favorite: false,
                sound: Sound::Swhoosh,
            },
        }
    }
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Debug, Copy)]
pub struct ThresholdNotification {
    /// Threshold of received + sent bytes; if exceeded a notification is emitted
    pub threshold: Option<u32>,
    /// The sound to emit
    pub sound: Sound,
    /// The last used Some value for the threshold field
    pub previous_threshold: u32,
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Debug, Copy)]
pub struct FavoriteNotification {
    /// Flag to determine if this notification is enabled
    pub notify_on_favorite: bool,
    /// The sound to emit
    pub sound: Sound,
}
