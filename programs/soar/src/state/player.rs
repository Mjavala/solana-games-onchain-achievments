use super::*;
use crate::SoarError;

impl Player {
    pub const MAX_USERNAME_LEN: usize = 100;

    pub const SIZE: usize = 8 + // discriminator                
        32 + Self::MAX_USERNAME_LEN + 32;

    pub fn new(username: String, nft_meta: Pubkey, user: Pubkey) -> Self {
        Player {
            user,
            username,
            nft_meta,
        }
    }

    pub fn check_field_lengths(&self) -> Result<()> {
        if self.username.len() > Self::MAX_USERNAME_LEN {
            return Err(SoarError::InvalidFieldLength.into());
        }

        Ok(())
    }
}

impl PlayerEntryList {
    pub const SIZE_WITHOUT_VEC: usize = 8 + // discriminator
        32 + 32 + 8;
    /// We initially allocate space for 10 [ScoreEntry] objects and realloc if more is needed.
    pub const INITIAL_SCORES_LENGTH: usize = 10;
    /// How much more space for [ScoreEntry] objects we add during a resize.
    pub const REALLOC_WINDOW: usize = 10;

    /// Calculate the space required during initialization
    pub fn initial_size() -> usize {
        Self::SIZE_WITHOUT_VEC + 4 + (Self::INITIAL_SCORES_LENGTH * ScoreEntry::SIZE)
    }

    pub fn new(player_info: Pubkey, leaderboard: Pubkey) -> Self {
        PlayerEntryList {
            player_info,
            leaderboard,
            score_count: 0,
            scores: Vec::with_capacity(Self::INITIAL_SCORES_LENGTH),
        }
    }
}

impl Merged {
    pub fn size(others_count: usize) -> usize {
        4 + 32 + (others_count * (32 + 1))
    }
}

impl MergeInfo {
    pub fn new(key: Pubkey) -> Self {
        MergeInfo {
            key,
            approved: false,
        }
    }
}

impl ScoreEntry {
    pub const SIZE: usize = 8 + 8;

    pub fn new(score: u64, timestamp: i64) -> Self {
        ScoreEntry { score, timestamp }
    }
}
