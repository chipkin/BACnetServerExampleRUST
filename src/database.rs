use std::collections::HashMap;

const PRIORITY_ARRAY_LENGTH: usize = 16;

pub enum ExampleDatabaseObject {
	AnalogInput(ExampleDatabaseAnalogInput),
	AnalogOutput(ExampleDatabaseAnalogOutput),
	AnalogValue(ExampleDatabaseAnalogValue),
	BinaryInput(ExampleDatabaseBinaryInput),
	Device(ExampleDatabaseDevice),
	MultiStateInput(ExampleDatabaseMultiStateInput),
	BitstringValue(ExampleDatabaseBitstringValue),
	CharacterStringValue(ExampleDatabaseCharacterStringValue),
	IntegerValue(ExampleDatabaseIntegerValue),
	LargeAnalogValue(ExampleDatabaseLargeAnalogValue),
	OctetStringValue(ExampleDatabaseOctetStringValue),
	PositiveIntegerValue(ExampleDatabasePositiveIntegerValue),
	NetworkPort(ExampleDatabaseNetworkPort),
	DateTimeValue(ExampleDatabaseDateTimeValue)
}

pub struct ExampleDatabaseAnalogInput {
	pub object_name: String,
	pub instance: u32,
	pub present_value: f32,

	pub cov_incurment: f32,
	pub reliability: u32,
	pub description: String,

	pub proprietary_year: u8,
	pub proprietary_month: u8,
	pub proprietary_day: u8,
	pub proprietary_weekday: u8,
	pub proprietary_hour: u8,
	pub proprietary_minute: u8,
	pub proprietary_second: u8,
	pub proprietary_hundredth_seconds: u8,

	pub proprietary_real: f32,

	pub proprietary_array_of_real: Vec<f32>
}

pub struct ExampleDatabaseAnalogOutput {
	pub object_name: String,
	pub instance: u32,
	pub priority_array_nulls: [bool; PRIORITY_ARRAY_LENGTH],
	pub priority_array_values: [bool; PRIORITY_ARRAY_LENGTH]	
}

pub struct ExampleDatabaseAnalogValue {
	pub object_name: String,
	pub instance: u32,
	pub present_value: f32,
	pub max_pres_value: f32,
	pub min_pres_value: f32
}

pub struct ExampleDatabaseBinaryInput {
	pub object_name: String,
	pub instance: u32,
	pub present_value: bool,
	pub description: String
}

pub struct ExampleDatabaseDevice {
	pub object_name: String,
	pub instance: u32,
	pub utc_offset: i32,
	pub current_time_offset: i32,
	pub description: String,
	pub system_status: u32
}

pub struct ExampleDatabaseMultiStateInput {
	pub object_name: String,
	pub instance: u32,
	pub present_value: u32,
	pub state_text: Vec<String>
}

pub struct ExampleDatabaseBitstringValue {
	pub object_name: String,
	pub instance: u32,
	pub present_value: Vec<bool>,
	pub bit_text: Vec<String>
}

pub struct ExampleDatabaseCharacterStringValue {
	pub object_name: String,
	pub instance: u32,
	pub present_value: String		
}

pub struct ExampleDatabaseIntegerValue {
	pub object_name: String,
	pub instance: u32,
	pub present_value: i32
}

pub struct ExampleDatabaseLargeAnalogValue {
	pub object_name: String,
	pub instance: u32,
	pub present_value: f64
}

pub struct ExampleDatabaseOctetStringValue {
	pub object_name: String,
	pub instance: u32,
	pub present_value: Vec<u8>
}

pub struct ExampleDatabasePositiveIntegerValue {
	pub object_name: String,
	pub instance: u32,
	pub present_value: u32
}

pub struct ExampleDatabaseNetworkPort {
	pub object_name: String,
	pub instance: u32,
	
	pub bacnet_ip_udp_port: u16,
	pub ip_address: [u8; 4],
	pub ip_address_length: u8,
	pub ip_default_gateway: [u8; 4],
	pub ip_default_gateway_length: u8,
	pub ip_subnet_mask: [u8; 4],
	pub ip_subnet_mask_length: u8,
	pub ip_dns_servers: Vec<u8>,
	pub ip_dns_server_length: u8,

	pub broadcast_ip_address: [u8; 4],

	pub changes_pending: bool,
	pub fd_bbmd_address_host_type: u8,
	pub fd_bbmd_address_host_ip: [u8; 4],
	pub fd_bbmd_address_port: u16,
	pub fd_subscription_lifetime: u16
}

pub struct ExampleDatabaseDateTimeValue {
	pub object_name: String,
	pub instance: u32,
	pub present_value_year: u8,
	pub present_value_month: u8,
	pub present_value_day: u8,
	pub present_value_weekday: u8,
	pub present_value_hour: u8,
	pub present_value_minute: u8,
	pub present_value_second: u8,
	pub present_value_hundredth_seconds: u8
}

pub fn setup_database() -> HashMap::<String, ExampleDatabaseObject> {
	let mut database = HashMap::<String, ExampleDatabaseObject>::new();
    database.insert("device-389001".to_string(), ExampleDatabaseObject::Device(setup_device()));
	database.insert("analog_input-0".to_string(), ExampleDatabaseObject::AnalogInput(setup_analog_input()));
	database.insert("analog_output-1".to_string(), ExampleDatabaseObject::AnalogOutput(setup_analog_output()));
	database.insert("analog_value-2".to_string(), ExampleDatabaseObject::AnalogValue(setup_analog_value()));
	database.insert("binary_input-3".to_string(), ExampleDatabaseObject::BinaryInput(setup_binary_input()));
	database.insert("multistate_input-13".to_string(), ExampleDatabaseObject::MultiStateInput(setup_multistate_input()));
	database.insert("bitstring_value-39".to_string(), ExampleDatabaseObject::BitstringValue(setup_bitstring_value()));
	database.insert("character_string_value-40".to_string(), ExampleDatabaseObject::CharacterStringValue(setup_character_string_value()));
	database.insert("integer_value-45".to_string(), ExampleDatabaseObject::IntegerValue(setup_integer_value()));
	database.insert("large_analog_value-46".to_string(), ExampleDatabaseObject::LargeAnalogValue(setup_large_analog_value()));
	database.insert("octet_string_value-47".to_string(), ExampleDatabaseObject::OctetStringValue(setup_octet_string_value()));
	database.insert("positive_integer_value-48".to_string(), ExampleDatabaseObject::PositiveIntegerValue(setup_positive_integer_value()));
	database.insert("network_port-56".to_string(), ExampleDatabaseObject::NetworkPort(setup_network_port()));
	database.insert("date_time_value-44".to_string(), ExampleDatabaseObject::DateTimeValue(setup_date_time_value()));
    database
}

fn setup_device() -> ExampleDatabaseDevice {
	let mut device = ExampleDatabaseDevice {	
		object_name: "Red King Device".to_string(),
		instance: 389001,
		utc_offset: 0,
		current_time_offset: 0,
		description: "CAS BACnet Rust Server Device".to_string(),
		system_status: 0
	};
	device
}

fn setup_analog_input() -> ExampleDatabaseAnalogInput {
	let mut analog_input = ExampleDatabaseAnalogInput {
		object_name: "Dungeness AnalogInput".to_string(),
		instance: 0,
		present_value: 1.001,
		cov_incurment: 2.0,
		reliability: 0,
		description: "Incurments once every 5 seconds".to_string(),
		proprietary_year: 122,
		proprietary_month: 3,
		proprietary_day: 20,
		proprietary_weekday: 7,
		proprietary_hour: 12,
		proprietary_minute: 34,
		proprietary_second: 23,
		proprietary_hundredth_seconds: 45,
		proprietary_real: 1.23,
		proprietary_array_of_real: vec![1.00; 5]
	};
	analog_input
}

fn setup_analog_output() -> ExampleDatabaseAnalogOutput {
	let mut analog_output = ExampleDatabaseAnalogOutput {
		object_name: "Snow AnalogOutput".to_string(),
		instance: 1,
		priority_array_nulls: [true; PRIORITY_ARRAY_LENGTH],
		priority_array_values: [false; PRIORITY_ARRAY_LENGTH]
	};
	analog_output
}

fn setup_analog_value() -> ExampleDatabaseAnalogValue {
	let mut analog_value = ExampleDatabaseAnalogValue {
		object_name: "Flower AnalogValue".to_string(),
		instance: 2,
		present_value: 5.43,
		max_pres_value: 1000.0,
		min_pres_value: -1000.0
	};
	analog_value
}

fn setup_binary_input() -> ExampleDatabaseBinaryInput {
	let mut binary_input = ExampleDatabaseBinaryInput {
		object_name: "Chesapeake Blue BinaryInput".to_string(),
		instance: 3,
		present_value: true,
		description: "I am an optional property!".to_string()
	};
	binary_input
}

fn setup_multistate_input() -> ExampleDatabaseMultiStateInput {
	let mut multistate_input = ExampleDatabaseMultiStateInput {
		object_name: "Pea MultiStateInput".to_string(),
		instance: 13,
		present_value: 1,
		state_text: vec!["one".to_string(), "two".to_string(), "three".to_string()]
	};
	multistate_input
}

fn setup_bitstring_value() -> ExampleDatabaseBitstringValue {
	let mut bitstring_value = ExampleDatabaseBitstringValue {	
		object_name: "Yeti BitstringValue".to_string(),
		instance: 39,
		present_value: vec![true, false, false, false],
		bit_text: vec!["A".to_string(), "B".to_string(), "C".to_string(), "D".to_string()]
	};
	bitstring_value
}

fn setup_character_string_value() -> ExampleDatabaseCharacterStringValue {
	let mut character_string_value = ExampleDatabaseCharacterStringValue {
		object_name: "Coconut CharacterStringValue".to_string(),
		instance: 40,
		present_value: "Hello World!".to_string()
	};
	character_string_value
}

fn setup_integer_value() -> ExampleDatabaseIntegerValue {
	let mut integer_value = ExampleDatabaseIntegerValue {
		object_name: "Spider IntegerValue".to_string(),
		instance: 45,
		present_value: 42
	};
	integer_value
}

fn setup_large_analog_value() -> ExampleDatabaseLargeAnalogValue {
	let mut large_analog_value = ExampleDatabaseLargeAnalogValue {
		object_name: "Tanner LargeAnalogValue".to_string(),
		instance: 46,
		present_value: 123456789.85
	};
	large_analog_value
}

fn setup_octet_string_value() -> ExampleDatabaseOctetStringValue {
	let mut octet_string_value = ExampleDatabaseOctetStringValue {
		object_name: "Brown Box OctetStringValue".to_string(),
		instance: 47,
		present_value: vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07]
	};
	octet_string_value
}

fn setup_positive_integer_value() -> ExampleDatabasePositiveIntegerValue {
	let mut positive_integer_value = ExampleDatabasePositiveIntegerValue {
		object_name: "Strawberry PositiveIntegerValue".to_string(),
		instance: 48,
		present_value: 12345
	};
	positive_integer_value
}

fn setup_network_port() -> ExampleDatabaseNetworkPort {
	let mut network_port = ExampleDatabaseNetworkPort {
		object_name: "Mitten NetworkPort".to_string(),
		instance: 56,
		bacnet_ip_udp_port: 47808,
		ip_address: [198, 168, 68, 105],
		ip_address_length: 6,
		ip_default_gateway: [198, 168, 68, 126],
		ip_default_gateway_length: 4,
		ip_subnet_mask: [255, 255, 255, 0],
		ip_subnet_mask_length: 4,
		ip_dns_servers: Vec::new(),
		ip_dns_server_length: 4,
		broadcast_ip_address: [198, 168, 68, 105],
		changes_pending: false,
		fd_bbmd_address_host_type: 1,
		fd_bbmd_address_host_ip: [198, 168, 68, 105],
		fd_bbmd_address_port: 47809,
		fd_subscription_lifetime: 3600
	};
	network_port
}

fn setup_date_time_value() -> ExampleDatabaseDateTimeValue {
	let mut date_time_value = ExampleDatabaseDateTimeValue {
		object_name: "Atlantic Rock DateTimeValue".to_string(),
		instance: 44,
		present_value_year: 122,
		present_value_month: 1,
		present_value_day: 28,
		present_value_weekday: 5,
		present_value_hour: 16,
		present_value_minute: 54,
		present_value_second: 47,
		present_value_hundredth_seconds: 55
	};
	date_time_value
}

/*
List of object names (species of Crabs):
"Dungeness", "Snow", "Flower", "Chesapeake Blue", "Red King", "Pea", "Yeti", "Coconut", 
"Japanese Spider", "Tanner", "Brown Box", "Strawberry", "Chinese Mitten", "Atlantic Rock"
*/