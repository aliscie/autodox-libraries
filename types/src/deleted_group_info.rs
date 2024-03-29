use crate::{ChatId, TimestampMillis, UserId};
use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct DeletedGroupInfo {
    pub id: ChatId,
    pub timestamp: TimestampMillis,
    pub deleted_by: UserId,
    pub group_name: String,
    pub public: bool,
}
