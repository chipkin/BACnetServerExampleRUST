pub mod cas_bacnet_stack_example_constants;
pub mod database;
pub mod cas_bacnet_stack_adapters;

use cas_bacnet_stack_example_constants as bacnet_const;
use cas_bacnet_stack_adapters as adapter;
use crate::database::ExampleDatabaseObject;

use std::net::UdpSocket;

use std::collections::HashMap;

use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::sync::MutexGuard;

use std::time::SystemTime;
use std::time::Duration;

use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::SocketAddr;

use std::thread;

use std::os::raw::c_char;

use std::io;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;

// Constants
const APPLICATION_VERSION: &str = "0.0.1";  // See CHANGELOG.md for a full list of changes.
const MAX_RENDER_BUFFER_LENGTH: usize = 1497;

// Static Variables
static socket: Lazy<UdpSocket> = Lazy::new(|| {
    match UdpSocket::bind("192.168.68.109:47808") {
		Ok(udp_socket) => {
			println!("UDP Socket Setup Success");
			if let Err(err) = udp_socket.set_read_timeout(Some(Duration::from_millis(50))) {
				panic!("UDP Socket Read Timeout Setting Failed");
			}
			udp_socket
		},
		_ => {
			panic!("UDP Socket Setup Failed");
		}
	}
});

static db: Lazy<Mutex<HashMap<String, ExampleDatabaseObject>>> = Lazy::new(|| {
	let mut database = database::setup_database();
	Mutex::new(database)
});

// Main function
fn main() {
	// Print versioning
	println!("CAS BACnet Stack Version: {:?}.{:?}.{:?}.{:?}", 
        adapter::get_api_major_version().unwrap(), adapter::get_api_minor_version().unwrap(), adapter::get_api_patch_version().unwrap(), adapter::get_api_build_version().unwrap());
    println!("Application Version: {:?}", APPLICATION_VERSION);

	// Loading CAS BACnet Stack functions
	if let Err(err) = load_bacnet_functions() {
		panic!("Unable to load functions from DLL: {:?}", err);
	}

	// Print device instance
	let device_instance;
	if let Some(ExampleDatabaseObject::Device(device)) = db.lock().unwrap().get("device-389001") {
		println!("Device Instance: {:?}", device.instance);
		device_instance = device.instance;
	} else {
		println!("Unable to get device instance. Manually setting to 389000.");
		device_instance = 389000;
	}
    // SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

	// Add device and properties
	if let Ok(x) = adapter::add_device(device_instance) {
		if x {
			println!("Device added");
		} else {
			println!("ERROR: Device was unable to be added");
		}
	} else {
		println!("ERROR: Add Device failed");
	}
	
	if let Ok(x) = adapter::set_property_enabled(device_instance, bacnet_const::OBJECT_TYPE_DEVICE, device_instance, bacnet_const::PROPERTY_IDENTIFIER_DESCRIPTION, true) {
		if x {
			println!("Device description added");
		} else {
			println!("ERROR: Device description was unable to be added");
		}
	} else {
		println!("ERROR: Set Property Enabled failed");
	}

	// Set services enabled
	if let Ok(x) = adapter::set_service_enabled(device_instance, bacnet_const::SERVICE_I_AM.into(), true) {
		if x {
			println!("I Am service enabled");
		} else {
			println!("ERROR: I Am service was unable to be enabled");
		}
	} else {
		println!("ERROR: Enable service failed");
	}
	if let Ok(x) = adapter::set_service_enabled(device_instance, bacnet_const::SERVICE_READ_PROPERTY_MULTIPLE.into(), true) {
		if x {
			println!("Read Property Multiple service enabled");
		} else {
			println!("ERROR: Read Property Multiple service was unable to be enabled");
		}
	} else {
		println!("ERROR: Enable service failed");
	}

	// Add object
	// Object instance is hard-coded
	// ANALOG INPUT
	if let Ok(x) = adapter::add_object(device_instance, bacnet_const::OBJECT_TYPE_ANALOG_INPUT, 0) {
		if x {
			println!("Analog Input added");
		} else {
			println!("ERROR: Analog Input was unable to be added");
		}
	} else {
		println!("ERROR: Add Analog Input failed");
	}
	if let Ok(x) = adapter::set_property_by_object_type_enabled(device_instance, bacnet_const::OBJECT_TYPE_ANALOG_INPUT, bacnet_const::PROPERTY_IDENTIFIER_DESCRIPTION, true) {
		if x {
			println!("Analog Input description added");
		} else {
			println!("ERROR: Analog Input description was unable to be added");
		}
	} else {
		println!("ERROR: Set Property Enabled failed");
	}
	if let Ok(x) = adapter::set_property_by_object_type_enabled(device_instance, bacnet_const::OBJECT_TYPE_ANALOG_INPUT.into(), bacnet_const::PROPERTY_IDENTIFIER_RELIABILITY.into(), true) {
		if x {
			println!("Analog Input reliability added");
		} else {
			println!("ERROR: Analog Input reliability was unable to be added");
		}
	} else {
		println!("ERROR: Set Property Enabled failed");
	}

	// CHARACTERSTRING VALUE
	if let Ok(x) = adapter::add_object(device_instance, bacnet_const::OBJECT_TYPE_CHARACTERSTRING_VALUE, 40) {
		if x {
			println!("Characterstring Value added");
		} else {
			println!("ERROR: Characterstring Value was unable to be added");
		}
	} else {
		println!("ERROR: Add Characterstring Value failed");
	}

	// Main Loop
	println!("Entering main loop...");
	println!("FYI: To quit the application, enter Q");
	let mut update_once_a_second_timer = SystemTime::now();
	let stdin_channel = spawn_stdin_channel();
    loop {
		adapter::bacnet_loop().unwrap();
		database_loop(&mut update_once_a_second_timer);
		if let Ok(key) = stdin_channel.try_recv() {
            if check_end_loop(&key) {
				break;
			}
        }
		thread::sleep(Duration::from_millis(0));
    }
}

fn load_bacnet_functions() -> Result<bool, Box<dyn std::error::Error>> {
	adapter::register_callback_receive_message(callback_receive_message)?;
	adapter::register_callback_send_message(callback_send_message)?;
	adapter::register_callback_get_system_time(callback_get_system_time)?;
	adapter::register_callback_get_property_character_string(callback_get_character_string)?;
	adapter::register_callback_get_property_enumerated(callback_get_enumerated)?;
	adapter::register_callback_get_property_real(callback_get_real)?;
	adapter::register_callback_get_property_signed_integer(callback_get_signed_integer)?;
	Ok(true)
}

fn database_loop(update_once_a_second_timer: &mut SystemTime) {
	let mut database = db.lock().unwrap();
	if let Ok(duration) = update_once_a_second_timer.elapsed() {		
		if duration.as_secs() >= 5 {
			*update_once_a_second_timer = SystemTime::now();
			if let Some(ExampleDatabaseObject::AnalogInput(analog_input)) = database.get_mut("analog_input-0") {
				analog_input.present_value += 1.001;
				println!("Analog Input increased!");
			}
		}
	}
}

fn check_end_loop(key: &str) -> bool {
	if key == "q\r\n" || key == "Q\r\n" {
		true
	} else {
		println!("Invalid input, enter Q to quit.");
		false
	}
}

// Referenced: https://stackoverflow.com/questions/30012995/how-can-i-read-non-blocking-from-stdin
fn spawn_stdin_channel() -> Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        tx.send(buffer).unwrap();
    });
    rx
}

fn callback_receive_message(message: *mut u8, max_message_length: u16, received_connection_string: *mut u8, max_connection_string_length: u8, received_connection_string_length: *mut u8, network_type: *mut u8) -> u16 {
		
	// Check parameters
	if message.is_null() || max_message_length == 0 {
		println!("Invalid input buffer");
		return 0;
	}
	if received_connection_string.is_null() || max_connection_string_length == 0 {
		println!("Invalid connection string buffer");
		return 0;
	}
	if max_connection_string_length < 6 {
		println!("Not enough space for a UDP connection string");
		return 0;
	}

	let port: u16;

	// Attempt to read bytes
	let mut buf: [u8; MAX_RENDER_BUFFER_LENGTH] = [0; MAX_RENDER_BUFFER_LENGTH];
	let (bytes_read, src_addr) = if let Ok((bytes_read, src_addr)) = socket.recv_from(&mut buf) {
		(bytes_read, src_addr)
	} else {
		(0 as usize, SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080))
	};
	if bytes_read > 0 {
		port = src_addr.port();
		let ip_address = src_addr.ip().to_canonical();
		println!("FYI: Received message from [{0}:{1}], length [{2}]", ip_address, port, bytes_read);

		// Convert the IP Address to the connection string
		if !convert_ip_address_to_bytes(src_addr.ip(), received_connection_string, max_connection_string_length) {
			println!("Failed to convert the ip address into a connectionString");
			return 0;
		}
		unsafe {
			*received_connection_string.add(4) = (port / 256) as u8;
			*received_connection_string.add(5) = (port % 256) as u8;

			*received_connection_string_length = 6;
			*network_type = bacnet_const::NETWORK_TYPE_IP;
		}

		if usize::from(max_message_length) < bytes_read || usize::from(MAX_RENDER_BUFFER_LENGTH) < bytes_read {
			return 0;
		}

		let mut index = 0;
		unsafe {
			while index < bytes_read {
				*message.add(index) = buf[index];
				index += 1;
			}
		}
	}

	bytes_read.try_into().unwrap()
}

fn callback_send_message(message: *const u8, message_length: u16, connection_string: *const u8, connection_string_length: u8, network_type: u8, broadcast: bool) -> u16 {
	println!("callback_send_message");

	if message.is_null() || message_length == 0 {
		println!("Nothing to send");
		return 0;
	}
	if connection_string.is_null() || connection_string_length == 0 {
		println!("No connection string");
		return 0;
	}

	// Verify Network Type
	if network_type != bacnet_const::NETWORK_TYPE_IP {
		println!("Message for different network");
		return 0;
	}

	// Prepare the IP Address
	let ip_address: Ipv4Addr;
	unsafe {
		if broadcast {
			let database = db.lock().unwrap();
			if let Some(ExampleDatabaseObject::NetworkPort(network_port)) = database.get("network_port-56") {
				ip_address = Ipv4Addr::new(
					*connection_string.add(0) | !network_port.ip_subnet_mask[0], 
					*connection_string.add(1) | !network_port.ip_subnet_mask[1], 
					*connection_string.add(2) | !network_port.ip_subnet_mask[2], 
					*connection_string.add(3) | !network_port.ip_subnet_mask[3]
				);
			} else {
				ip_address = Ipv4Addr::new(*connection_string.add(0), *connection_string.add(1), *connection_string.add(2), *connection_string.add(3));
			}
		}
		else {
			ip_address = Ipv4Addr::new(*connection_string.add(0), *connection_string.add(1), *connection_string.add(2), *connection_string.add(3));
		}
	}
	

	// Get the port
	let mut port: u16 = 0;
	unsafe {
		port = port + *connection_string.add(4) as u16 * 256;
		port = port + *connection_string.add(5) as u16;
	}

	println!("FYI: Sending message to [{0}:{1}], length [{2}]", ip_address, port, message_length);

	// Send the message
	let mut buf = [0; MAX_RENDER_BUFFER_LENGTH];

	if usize::from(message_length) > MAX_RENDER_BUFFER_LENGTH {
		println!("Message too large for buffer");
		return 0;
	} else {
		let mut index: usize = 0;
		unsafe {
			while index < message_length.into() {
				buf[index] = *message.add(index.into());
				index += 1;
			}
		}
	}

	if let Ok(x) = socket.send_to(&buf[0..usize::from(message_length)], (ip_address, port)) {
		return message_length;
	} else {
		println!("Failed to send message");
		return 0;
	}
}

fn convert_ip_address_to_bytes(ip_address: IpAddr, received_connection_string: *mut u8, max_connection_string_length: u8) -> bool {
	if max_connection_string_length < 4 {
		return false;
	}
	let ip_address_v4 = ip_address.to_canonical();
	match ip_address_v4 {
        IpAddr::V4(address) => {
			let ip_address_octet = address.octets();
			unsafe {
				*received_connection_string.add(0) = ip_address_octet[0];
				*received_connection_string.add(1) = ip_address_octet[1];
				*received_connection_string.add(2) = ip_address_octet[2];
				*received_connection_string.add(3) = ip_address_octet[3];
			}
			return true;
		},
        _ => {
			println!("Invalid IP Address");
			return false;
		},
     }
}

fn callback_get_system_time() -> u64 {
	let database = db.lock().unwrap();
	if let Some(ExampleDatabaseObject::Device(device)) = database.get("device-389001") {
		return SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()/* - x.current_time_offset*/;
	}
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
}

fn get_object_name(device_instance: u32, object_type: u16, object_instance: u32, value: *mut c_char, value_element_count: *mut u32, max_element_count: u32, database: MutexGuard<'_, HashMap<String, ExampleDatabaseObject>>) -> bool {	
	if object_type == bacnet_const::OBJECT_TYPE_DEVICE {
		if let Some(ExampleDatabaseObject::Device(device)) = database.get(format!("device-{device_instance}").as_str()) {
			if device.object_name.len() < max_element_count.try_into().unwrap() {
				unsafe {
					let mut index = 0;
					for character in device.object_name.chars() {
						*value.add(index) = character as c_char;
						index += 1;
					}
					*value.add(index) = '\0' as c_char;
					*value_element_count = device.object_name.len() as u32;
				}
				return true;
			}
		}
	}
	else if object_type == bacnet_const::OBJECT_TYPE_ANALOG_INPUT {
		if let Some(ExampleDatabaseObject::AnalogInput(analog_input)) = database.get(format!("analog_input-{object_instance}").as_str()) {
			if analog_input.object_name.len() < max_element_count.try_into().unwrap() {
				unsafe {
					let mut index = 0;
					for character in analog_input.object_name.chars() {
						*value.add(index) = character as c_char;
						index += 1;
					}
					*value.add(index) = '\0' as c_char;
					*value_element_count = analog_input.object_name.len() as u32;
				}
				return true;
			}
		}
	}
	else if object_type == bacnet_const::OBJECT_TYPE_CHARACTERSTRING_VALUE {
		if let Some(ExampleDatabaseObject::CharacterStringValue(character_string_value)) = database.get(format!("character_string_value-{object_instance}").as_str()) {
			if character_string_value.object_name.len() < max_element_count.try_into().unwrap() {
				unsafe {
					let mut index = 0;
					for character in character_string_value.object_name.chars() {
						*value.add(index) = character as c_char;
						index += 1;
					}
					*value.add(index) = '\0' as c_char;
					*value_element_count = character_string_value.object_name.len() as u32;
				}
				return true;
			}
		}
	}
	false
}

fn callback_get_character_string(device_instance: u32, object_type: u16, object_instance: u32, property_identifier: u32, value: *mut c_char, value_element_count: *mut u32, max_element_count: u32, encoding_type: *mut u8, use_array_index: bool, property_array_index: u32) -> bool {
	let database = db.lock().unwrap();
	if property_identifier == bacnet_const::PROPERTY_IDENTIFIER_OBJECT_NAME {
		return get_object_name(device_instance, object_type, object_instance, value, value_element_count, max_element_count, database);
	}
	else if property_identifier == bacnet_const::PROPERTY_IDENTIFIER_DESCRIPTION {
		if object_type == bacnet_const::OBJECT_TYPE_DEVICE {
			if let Some(ExampleDatabaseObject::Device(device)) = database.get(format!("device-{device_instance}").as_str()) {
				if device.description.len() < max_element_count.try_into().unwrap() {
					unsafe {
						let mut index = 0;
						for character in device.description.chars() {
							*value.add(index) = character as c_char;
							index += 1;
						}
						*value.add(index) = '\0' as c_char;
						*value_element_count = device.description.len() as u32;
					}
					return true;
				}
			}
		}
		else if object_type == bacnet_const::OBJECT_TYPE_ANALOG_INPUT {
			if let Some(ExampleDatabaseObject::AnalogInput(analog_input)) = database.get(format!("analog_input-{object_instance}").as_str()) {
				if analog_input.description.len() < max_element_count.try_into().unwrap() {
					unsafe {
						let mut index = 0;
						for character in analog_input.description.chars() {
							*value.add(index) = character as c_char;
							index += 1;
						}
						*value.add(index) = '\0' as c_char;
						*value_element_count = analog_input.description.len() as u32;
					}
					return true;
				}
			}
		}
	}
	else if property_identifier == bacnet_const::PROPERTY_IDENTIFIER_PRESENT_VALUE {
		if object_type == bacnet_const::OBJECT_TYPE_CHARACTERSTRING_VALUE {
			if let Some(ExampleDatabaseObject::CharacterStringValue(character_string_value)) = database.get(format!("character_string_value-{object_instance}").as_str()) {
				if character_string_value.present_value.len() < max_element_count.try_into().unwrap() {
					unsafe {
						let mut index = 0;
						for character in character_string_value.present_value.chars() {
							*value.add(index) = character as c_char;
							index += 1;
						}
						*value.add(index) = '\0' as c_char;
						*value_element_count = character_string_value.present_value.len() as u32;
					}
					return true;
				}
			}
		}
	}
	else if object_type == bacnet_const::OBJECT_TYPE_DEVICE && property_identifier == bacnet_const::PROPERTY_IDENTIFIER_APPLICATION_SOFTWARE_VERSION {
		if APPLICATION_VERSION.len() < max_element_count.try_into().unwrap() {
			unsafe {
				let mut index = 0;
				for character in APPLICATION_VERSION.chars() {
					*value.add(index) = character as c_char;
					index += 1;
				}
				*value.add(index) = '\0' as c_char;
				*value_element_count = APPLICATION_VERSION.len() as u32;
			}
			return true;
		}
	}
	false
}

fn callback_get_signed_integer(device_instance: u32, object_type: u16, object_instance: u32, property_identifier: u32, value: *mut i32, use_array_index: bool, property_array_index: u32) -> bool {
	let database = db.lock().unwrap();
	if property_identifier == bacnet_const::PROPERTY_IDENTIFIER_UTC_OFFSET {
		if object_type == bacnet_const::OBJECT_TYPE_DEVICE {
			if let Some(ExampleDatabaseObject::Device(device)) = database.get(format!("device-{device_instance}").as_str()) {
				unsafe {
					*value = device.utc_offset; 
				}
				return true;
			}
			return false;
		}
	}
	false
}

fn callback_get_enumerated(device_instance: u32, object_type: u16, object_instance: u32, property_identifier: u32, value: *mut u32, use_array_index: bool, property_array_index: u32) -> bool {
	let database = db.lock().unwrap();
	if property_identifier == bacnet_const::PROPERTY_IDENTIFIER_RELIABILITY {
		if object_type == bacnet_const::OBJECT_TYPE_ANALOG_INPUT {
			if let Some(ExampleDatabaseObject::AnalogInput(analog_input)) = database.get(format!("analog_input-{object_instance}").as_str()) {
				unsafe {
					*value = analog_input.reliability; 
				}	
				return true;
			}
			return false;
		}
	}
	else if property_identifier == bacnet_const::PROPERTY_IDENTIFIER_SYSTEM_STATUS &&
		object_type == bacnet_const::OBJECT_TYPE_DEVICE {
		println!("Debug: Device:System Status");
		if let Some(ExampleDatabaseObject::Device(device)) = database.get(format!("device-{device_instance}").as_str()) {
			unsafe {
				*value = device.system_status; 
			}
			return true;
		}
		return false;
	}
	false
}

fn callback_get_real(device_instance: u32, object_type: u16, object_instance: u32, property_identifier: u32, value: *mut f32, use_array_index: bool, property_array_index: u32) -> bool {
	let database = db.lock().unwrap();
	if property_identifier == bacnet_const::PROPERTY_IDENTIFIER_PRESENT_VALUE {
		if object_type == bacnet_const::OBJECT_TYPE_ANALOG_INPUT {
			if let Some(ExampleDatabaseObject::AnalogInput(analog_input)) = database.get(format!("analog_input-{object_instance}").as_str()) {
				unsafe {
					*value = analog_input.present_value; 
				}	
				return true;
			}
			return false;
		}
	}
	else if property_identifier == bacnet_const::PROPERTY_IDENTIFIER_COV_INCURMENT && object_type == bacnet_const::OBJECT_TYPE_ANALOG_INPUT {
		if let Some(ExampleDatabaseObject::AnalogInput(analog_input)) = database.get(format!("analog_input-{object_instance}").as_str()) {
			unsafe {
				*value = analog_input.cov_incurment; 
			}	
			return true;
		}
		return false;
	}
	false
}