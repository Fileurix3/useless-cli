# useless-cli

Repository with useless CLI

## Install

**You should install `cargo` and `rustc`**

1. **Clone the Repository**

   ```bash
   git clone https://github.com/Fileurix3/useless-cli.git
   ```

2. **Navigate to the project folder**

   ```bash
   cd useless-cli
   ```

3. **Install CLI**

   ```bash
   sudo make install
   ```

## Uninstall

1. **Navigate to the project folder**

   ```bash
   cd useless-cli
   ```

2. **Remove CLI**

   ```bash
   sudo make uninstall
   ```

3. **Remove folder with this project**
   ```bash
   rm -rf useless-cli
   ```

## CLI Functions:

**CLI Name** `usls`

1. `--calc, -c`:

   Simple calculator that supports `-, +, /, *, ()` with only logical operators inside: `AND, OR, XOR, NOT`
   **Example cli:** `--calc "2 + (2 + 2) * 2`

2. `--xorshift, -x`:

   Generate pseudo random number use XORShift, you must specify a number between 0 and 255,
   **Example cli:** `--xorshift 10`
