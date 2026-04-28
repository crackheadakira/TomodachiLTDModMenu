import mmh3

def process_screens(input_file, output_file):
    try:
        with open(input_file, 'r', encoding='utf-8') as infile, \
             open(output_file, 'w', encoding='utf-8') as outfile:
            
            for line in infile:
                line = line.strip()
                if not line or "ScreenName:" not in line:
                    continue
                
                # Extract the ScreenName part from the line
                # Example line: VTable: 71031c6c18 | ScreenName: ScreenEditorCommonBg
                screen_name = line.split("ScreenName:")[1].strip()
                
                # Strip the prefix "Screen" if it exists
                if screen_name.startswith("Screen"):
                    stripped_name = screen_name[6:] # Removes the first 6 characters ("Screen")
                else:
                    stripped_name = screen_name
                
                # Calculate the Murmur3 hash (UTF-8, seed=0, unsigned)
                hash_val = mmh3.hash(stripped_name.encode('utf-8'), seed=0, signed=False)
                hash_hex = f"{hash_val:08x}" # Format as 8-character lowercase hex
                
                # Write the output to the file
                out_line = f"{hash_hex} | {stripped_name}\n"
                outfile.write(out_line)
                
                # Optional: print to console so you can see it working
                print(f"Processed: {screen_name} -> {stripped_name} -> {hash_hex}")
                
        print(f"\n--- Success: Hashes written to {output_file} ---")

    except FileNotFoundError:
        print(f"Error: Could not find the file '{input_file}'. Make sure it is in the same directory as this script.")

if __name__ == "__main__":
    input_filename = "out_2.txt"
    output_filename = "screen_hashes_output.txt"
    
    process_screens(input_filename, output_filename)