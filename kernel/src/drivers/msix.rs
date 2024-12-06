// Stolen from Redox OS...

#[repr(u8)]
pub enum TriggerMode {
	Edge = 0,
	Level = 1,
}

#[repr(u8)]
pub enum LevelTriggerMode {
	Deassert = 0,
	Assert = 1,
}

#[repr(u8)]
pub enum DeliveryMode {
	Fixed = 0b000,
	LowestPriority = 0b001,
	Smi = 0b010,
	// 0b011 is reserved
	Nmi = 0b100,
	Init = 0b101,
	// 0b110 is reserved
	ExtInit = 0b111,
}

// TODO: should the reserved field be preserved?
pub const fn message_address(
	destination_id: u8,
	redirect_hint: bool,
	dest_mode_logical: bool,
) -> u64 {
	0x0000_0000_FEE0_0000u64
		| ((destination_id as u64) << 12)
		| ((redirect_hint as u64) << 3)
		| ((dest_mode_logical as u64) << 2)
}
pub const fn message_data(
	trigger_mode: TriggerMode,
	level_trigger_mode: LevelTriggerMode,
	delivery_mode: DeliveryMode,
	vector: u8,
) -> u32 {
	((trigger_mode as u32) << 15)
		| ((level_trigger_mode as u32) << 14)
		| ((delivery_mode as u32) << 8)
		| vector as u32
}
pub const fn message_data_level_triggered(
	level_trigger_mode: LevelTriggerMode,
	delivery_mode: DeliveryMode,
	vector: u8,
) -> u32 {
	message_data(
		TriggerMode::Level,
		level_trigger_mode,
		delivery_mode,
		vector,
	)
}
pub const fn message_data_edge_triggered(delivery_mode: DeliveryMode, vector: u8) -> u32 {
	message_data(
		TriggerMode::Edge,
		LevelTriggerMode::Deassert,
		delivery_mode,
		vector,
	)
}
