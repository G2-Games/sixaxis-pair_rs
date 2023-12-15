# SixAxis Pair

This is a tool for managing the MAC address a PS3 SixAxis controller is paired to. 

Directly inspired by [sixaxispairer](https://github.com/user-none/sixaxispairer).

## Building/Running

### Linux:
```bash
git clone https://github.com/G2-Games/sixaxis-pair_rs.git
cd sixaxis-pair_rs

# You may need to install the HIDAPI library
# development files on some Linux distributions
sudo dnf install hidapi-devel  # Fedora
sudo apt install libhidapi-dev # Ubuntu

cargo run
```

## Usage:
To get the current controller's paired MAC simply run the tool with no arguments. Otherwise, you can run the tool followed by a MAC address to change the pairing.

Example: 

`./sixaxis-pair_rs [mac]`
