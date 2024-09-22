---
# UDP Flooding Tool

## Overview

The UDP Flooding Tool is a network stress testing application designed to simulate high traffic conditions by sending a flood of UDP packets to a specified target IP address and port. This tool is useful for testing network performance, resilience, and stability under heavy load conditions.

## Features
![2CAdI2kQdz](https://github.com/user-attachments/assets/ea4034cc-02e2-4a0f-bc03-f012a3819091)
![image](https://github.com/user-attachments/assets/6cd5b181-a566-424c-8b46-1a0dab519e61)

- **Customizable Attack Parameters:**
  - **Target IP Address:** Specify the IP address of the target server.
  - **Packet Size:** Define the size of each UDP packet sent during the attack.
  - **UDP Port:** Choose from a list of common UDP ports or specify a custom port.
  - **Attack Duration:** Set the duration for the attack in seconds.
  - **Number of Threads:** Control the number of concurrent threads sending UDP packets.

- **Real-time Attack Monitoring:**
  - **Progress Bar:** Visual representation of the attack progress.
  - **Packet Frequency:** Display the frequency of packets being sent per second.
  - **Packets Sent:** Keep track of the total number of packets sent.

- **Alerts and Notifications:**
  - **Start Attack Alert:** Notification with attack parameters when starting the attack.
  - **End of Attack Alert:** Alert window that notifies you when the attack is complete.

## Installation

To build and run the UDP Flooding Tool, you'll need Rust installed on your system. If you haven't installed Rust yet, follow the instructions on the [official Rust website](https://www.rust-lang.org/).

1. **Clone the Repository:**
   ```sh
   git clone https://github.com/lfillaz/udp-flooding-tool.git
   cd udp-flooding-tool
   ```

2. **Build the Project:**
   ```sh
   cargo build
   ```

3. **Run the Application:**
   ```sh
   cargo run
   ```

## Usage

1. **Launch the Application:**
   - Run the application using `cargo run`. A window will appear with the tool's interface.

2. **Configure Attack Parameters:**
   - **Target IP:** Enter the IP address of the target server.
   - **Packet Size:** Use the slider to set the desired packet size (in bytes).
   - **UDP Port:** Select a common UDP port or enter a custom port.
   - **Attack Duration:** Set the duration for the attack.
   - **Number of Threads:** Adjust the number of threads for sending packets.

3. **Start the Attack:**
   - Click the "Start Attack" button to initiate the attack. An alert will display the attack parameters.
   - Monitor the attack progress through the progress bar and packet statistics.

4. **End of Attack:**
   - The tool will automatically notify you when the attack is complete.

## Contributing


Contributions are welcome! If you have suggestions or improvements, please open an issue or submit a pull request on [GitHub](https://github.com/lfillaz/udp-flooding-tool).


## Disclaimer

The UDP Flooding Tool is intended for educational and testing purposes only. Use this tool responsibly and ensure you have permission before testing any network or system.

---
