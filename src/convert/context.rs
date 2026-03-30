use crate::protos;

impl From<&protos::EpochSchedule> for solana_epoch_schedule::EpochSchedule {
    fn from(value: &protos::EpochSchedule) -> Self {
        solana_epoch_schedule::EpochSchedule {
            slots_per_epoch: value.slots_per_epoch,
            leader_schedule_slot_offset: value.leader_schedule_slot_offset,
            warmup: value.warmup,
            first_normal_epoch: value.first_normal_epoch,
            first_normal_slot: value.first_normal_slot,
        }
    }
}
