def generate_rust_substitution_table(file_path):
    with open(file_path, 'r') as f:
        # Read the file and convert each line to an integer
        numbers = [int(line.strip()) for line in f.readlines()]

    if len(numbers) != 256:
        raise ValueError("The file must contain exactly 256 values.")

    # Generate the Rust code
    rust_code = "const SUBSTITUTION_TABLE: [u8; 256] = [\n"

    # Add numbers to the Rust array, formatting it for readability
    for i, num in enumerate(numbers):
        rust_code += f"    {num:#04x}, "  # Format as hexadecimal, e.g., 0x01
        if (i + 1) % 8 == 0:  # Line break after every 8 entries
            rust_code += "\n"

    rust_code += "];\n"

    return rust_code

if __name__ == "__main__":
    file_path = 'random.txt'  # Path to the random.txt file
    rust_code = generate_rust_substitution_table(file_path)

    print(rust_code)

    print("Rust code generated and saved to substitution_table.rs")
