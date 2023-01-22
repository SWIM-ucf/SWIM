window.SIDEBAR_ITEMS = {"fn":[["append_binary","Append binary takes two numbers, shifts the first by a specified amount and then bitwise ors the two numbers together effectively appending the second onto the first."],["create_binary_vec","Creates a vector of u32 from the data found in the parser / assembler to put into memory."],["parser","Parser is the starting function of the parser / assembler process. It takes a string representation of a MIPS program and builds the binary of the instructions while cataloging any errors that are found."],["place_binary_in_middle_of_another","This function takes two numbers and inserts the binary of the second at a given index in the binary of the first. All binary values at and past the insertion index of the original string will be moved to the end of the resultant string. Since binary is sign extended on the left to 32 bits, insertion index must be the index from the end of the string."],["read_instructions","Takes the vector of instructions and assembles the binary for them."]]};