import sys
import argparse

# ==========================================================
# CONFIGURATION
# These represent the internal state of the ScreenRegistry
# ==========================================================
HASH_TABLE_BASE = 0x21  # Replace with your actual base address
HASH_MASK = 0xa6ccde18                  # Replace with your registry->hash_mask

def resolve_screen_id(screen_hash, hash_mask, table_base):
    """
    Python implementation of FUN_7100350144
    """
    if screen_hash == 0 or hash_mask == 0:
        return 0xFFFFFFFF

    # Ghidra logic: uVar3 = screen_hash - (screen_hash / uVar1) * uVar1
    # This is essentially: index = screen_hash % hash_mask
    start_index = screen_hash % hash_mask
    current_index = start_index

    # In a real scenario, you'd need to read process memory here.
    # Since this is a script, we simulate the logic of the do-while loop.
    # We'll print what the script is "looking for" at the simulated offsets.
    
    print(f"[*] Starting search at index: {start_index}")

    while True:
        # Ghidra calculates the offset as: hash_table_base + (index << 3)
        # Each entry is 8 bytes: [4 bytes Hash][4 bytes ID]
        target_offset = table_base + (current_index * 8)
        
        # This is where the script would pull the value from the binary
        # print(f"    Checking Index {current_index} at offset {hex(target_offset)}...")

        # Simulation Warning: 
        # Since I don't have your live memory, this script assumes 
        # a standard linear probe. To actually find the ID, you would
        # need to dump the memory at HASH_TABLE_BASE.
        
        # Logical loop increment (Linear Probing)
        next_index = 0
        if (current_index + 1) < hash_mask:
            next_index = current_index + 1
        
        # If we loop back to start, we failed
        if next_index == start_index:
            break
            
        current_index = next_index

    return "ID_FROM_MEMORY_AT_OFFSET"

def main():
    parser = argparse.ArgumentParser(description="Resolve Tomodachi Life Screen Hash to ID")
    parser.add_argument("hash", help="The screen hash (e.g. 0xBCD650D3 or 12345)", type=str)
    args = parser.parse_args()

    # Handle hex or decimal input
    try:
        if args.hash.lower().startswith('0x'):
            screen_hash = int(args.hash, 16)
        else:
            screen_hash = int(args.hash)
    except ValueError:
        print(f"[-] Invalid hash input: {args.hash}")
        sys.exit(1)

    print(f"[*] Resolving Hash: {hex(screen_hash)}")
    print(f"[*] Hash Mask: {HASH_MASK}")
    print(f"[*] Table Base: {hex(HASH_TABLE_BASE)}")
    print("-" * 40)

    # Calculate initial probe
    idx = screen_hash % HASH_MASK
    offset = HASH_TABLE_BASE + (idx * 8)
    
    print(f"[+] Initial check should be at Index: {idx}")
    print(f"[+] Memory Address to check: {hex(offset)}")
    print(f"[+] The Screen ID will be the 4 bytes at: {hex(offset + 4)}")
    print("-" * 40)
    print("[!] To get the actual ID, read 4 bytes at (Table_Base + (Index * 8) + 4)")

if __name__ == "__main__":
    main()