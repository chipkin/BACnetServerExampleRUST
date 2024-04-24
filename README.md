# BACnetServerExampleRUST

This is a CAS BACnet Stack Server example written in Rust using the CAS BACnet Stack.

## Usage

Pre-configured with the following example BACnet device and objects:

- **Device**: 389001 (Red King Device)
  - analog_input: 0  (Dungeness AnalogInput)
  - characterstring_value: 40  (Coconut CharacterStringValue)

Enter "q" or "Q" in the application to quit out of the application.

## Build

1. Install Rust and Cargo through the official Rust website: [Rust Getting Started](https://www.rust-lang.org/learn/get-started). Follow the instructions under “Rustup: the Rust installer and version management tool” and download the executable.
2. Follow the instructions for the executable on installation. Upon completion, you may use `cargo --version` to check if Rust and Cargo was installed properly. You will need at least Cargo Version 1.75.0 for this application.
3. Place `CASBACnetStack_x64_Debug.dll` into the bin folder. If there is no bin folder, create one.
4. Replace the IP Address of the static socket variable in `main.rs` to your IP Address.
5. Run `cargo run` in the Command Line to start the application. Make sure you are in the correct directory!

## Example Output

```txt
CAS BACnet Stack Version: 4.1.19.2330
Application Version: "0.0.1"
Device Instance: 389001
Device added
Device description added
I Am service enabled
Read Property Multiple service enabled
Analog Input added
Analog Input reliability added
Characterstring Value added
Entering main loop...
FYI: To quit the application, enter Q
UDP Socket Setup Success
::CASBACnetStack::BACnetDataLinkSC::Loop() in file: C:\dev\gitlab-runner\builds\b1afdc2b\2\chipkin\cas-bacnet-stack\source\BACnetDataLinkSC.cpp(250) - Error: UUID has not been set.  A UUID must be set for the BACnetSC device to start
```
