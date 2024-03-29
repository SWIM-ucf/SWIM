//! Helpful common functions used for the visual datapath.

use gloo::utils::{document, window};
use gloo_console::log;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlCollection, HtmlElement, HtmlObjectElement, MouseEvent};
use yew::UseReducerHandle;

use crate::{
    agent::datapath_reducer::DatapathReducer,
    emulation_core::{architectures::AvailableDatapaths, line_info::LineInformation},
};

use super::consts::*;

/// Returns an [`HtmlObjectElement`] corresponding to the `<object>` element in HTML.
pub fn get_datapath_root() -> HtmlObjectElement {
    document()
        .get_element_by_id(DATAPATH_ID)
        .unwrap()
        .dyn_into::<HtmlObjectElement>()
        .unwrap()
}

/// Returns an [`HtmlElement`] corresponding to the `<div id="popup">` element in HTML.
pub fn get_popup_element() -> HtmlElement {
    document()
        .get_element_by_id("popup")
        .unwrap()
        .unchecked_into::<HtmlElement>()
}

/// Returns an [`HtmlCollection`] containing all the `<g>` elements within the SVG diagram.
pub fn get_g_elements() -> HtmlCollection {
    get_datapath_root()
        .content_document()
        .unwrap()
        .first_element_child()
        .unwrap()
        .query_selector("g")
        .unwrap()
        .unwrap()
        .children()
}

/// Returns the size of the browser window in pixels.
pub fn get_window_size() -> (i32, i32) {
    (
        window().inner_width().unwrap().as_f64().unwrap() as i32,
        window().inner_height().unwrap().as_f64().unwrap() as i32,
    )
}

/// Returns the relative coordinates of the `<object>` element to the page.
pub fn get_datapath_position() -> (i32, i32) {
    let datapath_root = get_datapath_root();

    (datapath_root.offset_left(), datapath_root.offset_top())
}

/// Given a [`MouseEvent`] inside the datapath `<object>` element, returns
/// the coordinates of the mouse on the full screen.
///
/// This coordinate is highly dependent on the implementation and structure
/// of the page. This should be considered more like a macro than a re-usable
/// function.
pub fn get_datapath_iframe_mouse_position(event: MouseEvent) -> (i32, i32) {
    let datapath_position = get_datapath_position();

    let datapath_wrapper = gloo::utils::document()
        .get_element_by_id("datapath-scrollbox")
        .unwrap()
        .unchecked_into::<HtmlElement>();
    let scroll_position = (
        datapath_wrapper.scroll_left(),
        datapath_wrapper.scroll_top(),
    );

    (
        event.client_x() + datapath_position.0 - scroll_position.0,
        event.client_y() + datapath_position.1 - scroll_position.1,
    )
}

/// Given the mouse location, the size of the popup, and the window size,
/// return the coordinates of the top left corner of where the popup should go.
pub fn calculate_popup_position(
    mouse_position: (i32, i32),
    popup_size: (i32, i32),
    window_size: (i32, i32),
) -> (i32, i32) {
    // The horizontal and vertical distance that the popup should be from the mouse.
    const MOUSE_GAP: i32 = 20;

    // As a start, try to put the popup to the lower-right of the mouse.
    let mut position = (mouse_position.0 + MOUSE_GAP, mouse_position.1 + MOUSE_GAP);

    // If the popup gets cut off at the bottom, go to the upper-right of the mouse instead.
    if position.1 + popup_size.1 > window_size.1 {
        position = (position.0, mouse_position.1 - popup_size.1 - MOUSE_GAP);
    }

    // If the popup gets cut off at the right, force the x-position against the side of the screen.
    if position.0 + popup_size.0 > window_size.0 {
        position = (window_size.0 - popup_size.0, position.1);
    }

    position
}

/// Perform some function over an [`HtmlCollection`], assuming each element
/// inside of it is valid.
pub fn do_over_html_collection<F>(html_collection: &HtmlCollection, mut function: F)
where
    F: FnMut(&Element),
{
    for i in 0..html_collection.length() {
        let element = html_collection.item(i).unwrap();

        function(&element);
    }
}

/// Perform some function over an [`HtmlCollection`], but without unwrapping
/// each element.
///
/// This allows the programmer to check first if an `unwrap()` was successful,
/// for example.
pub fn do_over_html_collection_safe<F>(html_collection: &HtmlCollection, mut function: F)
where
    F: FnMut(&Option<Element>),
{
    for i in 0..html_collection.length() {
        let element = html_collection.item(i);

        function(&element);
    }
}

/// Set the data contained in the popup.
///
/// Parameters:
/// - `datapath`: A reference to the datapath that information will be pulled from.
/// - `variable`: The "variable" attribute of the line in the diagram that will have information.
pub fn populate_popup_information(
    datapath_state: &UseReducerHandle<DatapathReducer>,
    variable: &str,
) {
    let popup = get_popup_element();

    let title = popup.query_selector(".title").unwrap().unwrap();
    let description = popup.query_selector(".description").unwrap().unwrap();
    let bits = popup.query_selector(".data .code").unwrap().unwrap();
    let meaning = popup.query_selector(".meaning").unwrap().unwrap();

    let information = visual_line_to_data(variable, datapath_state);

    title.set_text_content(Some(&information.title));
    description.set_text_content(Some(&information.description));
    bits.set_text_content(Some(&u64_to_bits(information.value, information.bits)));
    meaning.set_text_content(Some(&u64::to_string(&information.value)));
}

/// Convert an integer value to a string, limited to `bits` number of bits.
///
/// If `bits` is less than 64, the lower `bits` number of bits in `value` will be used.
pub fn u64_to_bits(mut value: u64, bits: u64) -> String {
    let mut output = String::new();

    for _ in 0..bits {
        let bit = (value % 2) as u32;
        output.push(char::from_digit(bit, 10).unwrap_or_default());
        value /= 2;
    }

    output = output.chars().rev().collect::<String>();

    output
}

pub fn visual_line_to_data(
    variable: &str,
    datapath_state: &UseReducerHandle<DatapathReducer>,
) -> LineInformation {
    log!("Calling here");
    match datapath_state.current_architecture {
        AvailableDatapaths::MIPS => {
            match variable {
                "alu_input2" => LineInformation {
                    title: String::from("ALU Input 2"),
                    description: String::from("The second input to the ALU. This is determined by the ALUSrc control signal to select between register data, a sign-extended and left-shifted immediate value, or a zero-extended immediate value."),
                    value: datapath_state.mips.state.alu_input2,
                    bits: 64,
                },
                "alu_result" => LineInformation {
                    title: String::from("ALU Result"),
                    description: String::from("The result of the calculation performed by the ALU. This is used either as an address to access memory or as a value that is saved into a register."),
                    value: datapath_state.mips.state.alu_result,
                    bits: 64,
                },
                "data_result" => LineInformation {
                    title: String::from("Writeback Data"),
                    description: String::from("After finishing processing the instruction, this will either be the ALU result, data from memory, or PC + 4, based on the MemToReg control signal. This data is saved into registers."),
                    value: datapath_state.mips.state.data_result,
                    bits: 64,
                },
                "fpu_alu_result" => LineInformation {
                    title: String::from("Floating-Point ALU Result"),
                    description: String::from("The result of the calculation performed by the floating-point ALU. This is used as an option to be written to a floating-point register, based on the DataWrite and FpuMemToReg control signals."),
                    value: datapath_state.mips.coprocessor_state.alu_result,
                    bits: 64,
                },
                "fpu_branch_decision" => LineInformation {
                    title: String::from("FPU Branch Decision"),
                    description: String::from("Based on the true/false branch flag, determines whether to branch. (The FpuBranch control signal must also be set.)"),
                    value: datapath_state.mips.coprocessor_state.condition_code_mux as u64,
                    bits: 1,
                },
                "fpu_branch_flag" => LineInformation {
                    title: String::from("Instruction [16] (True/False Branch Flag)"),
                    description: String::from("The true/false branch flag of branching datapath_state.mips.coprocessor instructions. This flag specifies whether a floating-point branch instruction is BC1T or BC1F."),
                    value: datapath_state.mips.coprocessor_state.branch_flag as u64,
                    bits: 1,
                },
                "fpu_comparator_result" => LineInformation {
                    title: String::from("Floating-Point Comparator Result"),
                    description: String::from("The result of the comparison of two floating-point values. This is routed to the \"Condition Code\" (cc) register, and will be written there if the CcWrite control signal is set."),
                    value: datapath_state.mips.coprocessor_state.comparator_result,
                    bits: 64,
                },
                "fpu_condition_code" => LineInformation {
                    title: String::from("Condition Code Value"),
                    description: String::from("Data retrieved from the \"Condition Code\" (cc) register. This specifies whether a previous conditional instruction was true or false."),
                    value: datapath_state.mips.coprocessor_state.condition_code_bit as u64,
                    bits: 1,
                },
                "fpu_condition_code_inverted" => LineInformation {
                    title: String::from("Condition Code Value (Inverted)"),
                    description: String::from("Inverted form of the condition code register value."),
                    value: datapath_state.mips.coprocessor_state.condition_code_bit_inverted as u64,
                    bits: 1,
                },
                "fpu_data" => LineInformation {
                    title: String::from("Floating-Point Data Register Value"),
                    description: String::from("Data retrieved from the \"Data\" register. This register acts as a means to communicate data between the main processor and floating-point datapath_state.mips.coprocessor in MTC1 and MFC1 instructions."),
                    value: datapath_state.mips.coprocessor_state.fmt as u64,
                    bits: 64,
                },
                "fpu_data_writeback" => LineInformation {
                    title: String::from("Floating-Point Data Writeback"),
                    description: String::from("The value from the floating-point unit's \"Data\" register. Depending on the FpuRegWidth control signal, this will be 64-bit data or sign-extended 32-bit data."),
                    value: datapath_state.mips.coprocessor_state.data_writeback,
                    bits: 64,
                },
                "fpu_destination" => LineInformation {
                    title: String::from("Floating-Point Write Register"),
                    description: String::from("The register that will be written to, assuming FpuRegWrite is set. Depending on the FpuRegDst control signal, this will consist of the fs, ft, or fd register."),
                    value: datapath_state.mips.coprocessor_state.destination as u64,
                    bits: 5,
                },
                "fpu_fd" => LineInformation {
                    title: String::from("Instruction [10-6] (fd)"),
                    description: String::from("The fd field. Depending on the FpuRegDst control signal, this will be the register written to in a floating-point operation. This register is used as the destination for most floating-point arithmetic instructions."),
                    value: datapath_state.mips.coprocessor_state.fd as u64,
                    bits: 5,
                },
                "fpu_fmt" => LineInformation {
                    title: String::from("Instruction [25-21] (fmt)"),
                    description: String::from("The fmt field. This is used to distinguish between single-precision and double-precision floating-point instructions."),
                    value: datapath_state.mips.coprocessor_state.fmt as u64,
                    bits: 5,
                },
                "fpu_fp_register_data_from_main_processor" => LineInformation {
                    title: String::from("Writeback Data (To Floating-Point Coprocessor)"),
                    description: String::from("This data is written to a floating-point register, given FpuMemToReg is set. This line allows data to load from memory to a floating-point register, specifically in the case of the LWC1 instruction."),
                    value: datapath_state.mips.coprocessor_state.fp_register_data_from_main_processor,
                    bits: 64,
                },
                "fpu_fp_register_to_memory" => LineInformation {
                    title: String::from("Memory Write Data (from FPU)"),
                    description: String::from("If the MemWriteSrc control signal is set, this data will be written to memory. This is used for the SWC1 instruction."),
                    value: datapath_state.mips.coprocessor_state.fp_register_to_memory,
                    bits: 64,
                },
                "fpu_fs" => LineInformation {
                    title: String::from("Instruction [15-11] (fs)"),
                    description: String::from("The fs field. Contains the first register to be read for a floating-point instruction."),
                    value: datapath_state.mips.coprocessor_state.fs as u64,
                    bits: 5,
                },
                "fpu_ft" => LineInformation {
                    title: String::from("Instruction [20-16] (ft)"),
                    description: String::from("The ft field. Contains the second register to be read for a floating-point instruction."),
                    value: datapath_state.mips.coprocessor_state.ft as u64,
                    bits: 5,
                },
                "fpu_new_data" => LineInformation {
                    title: String::from("New Floating-Point Data Register Value"),
                    description: String::from("Data sent to the \"Data\" register. Depending on the DataSrc control signal, this will either be data from the main processor or the floating-point datapath_state.mips.coprocessor. This register acts as a means to communicate data between the main processor and floating-point datapath_state.mips.coprocessor in MTC1 and MFC1 instructions."),
                    value: datapath_state.mips.coprocessor_state.fmt as u64,
                    bits: 64,
                },
                "fpu_read_data_1" => LineInformation {
                    title: String::from("FPU Read Data 1"),
                    description: String::from("Data retrieved from the register specified by the fs instruction field. This is used as the first inputs to the floating-point ALU and comparator. This can additionally be written to the \"Data\" register, based on the DataSrc and DataWrite control signals."),
                    value: datapath_state.mips.coprocessor_state.read_data_1,
                    bits: 64,
                },
                "fpu_read_data_2" => LineInformation {
                    title: String::from("FPU Read Data 2"),
                    description: String::from("Data retrieved from the register specified by the ft instruction field. This is used as the second inputs to the floating-point ALU and comparator. This can additionally be used as data to be written to memory, based on the MemWriteSrc control signal."),
                    value: datapath_state.mips.coprocessor_state.read_data_2,
                    bits: 64,
                },
                "fpu_register_write_data" => LineInformation {
                    title: String::from("FPU Register Write Data"),
                    description: String::from("Data that will be written to a floating-point register, given that FpuRegWrite is set."),
                    value: datapath_state.mips.coprocessor_state.register_write_data,
                    bits: 64,
                },
                "fpu_register_write_mux_to_mux" => LineInformation {
                    title: String::from("FPU Register Write Data (When FpuMemToReg is Unset)"),
                    description: String::from("Based on the DataWrite control signal, this will either be the result of the floating-point ALU or the contents of the \"Data\" register. (The \"Data\" register is used for transferring data between the processor and floating-point datapath_state.mips.coprocessor.)"),
                    value: datapath_state.mips.coprocessor_state.register_write_mux_to_mux,
                    bits: 64,
                },
                "fpu_sign_extend_data" => LineInformation {
                    title: String::from("Floating-Point Data Register Value (Sign-Extended)"),
                    description: String::from("In the case where FpuRegWidth indicates a 32-bit width, this is the bottom 32 bits of the value from the \"Data\" register, then sign-extended to 64 bits."),
                    value: datapath_state.mips.coprocessor_state.sign_extend_data,
                    bits: 64,
                },
                "funct" => LineInformation {
                    title: String::from("Instruction [5-0] (funct)"),
                    description: String::from("The funct field. Contains the type of operation to execute for R-type instructions."),
                    value: datapath_state.mips.state.funct as u64,
                    bits: 6,
                },
                "imm" => LineInformation {
                    title: String::from("Instruction [15-0] (immediate)"),
                    description: String::from("The immediate field. Contains the 16-bit constant value used for I-type instructions."),
                    value: datapath_state.mips.state.imm as u64,
                    bits: 16,
                },
                "instruction" => LineInformation {
                    title: String::from("Instruction"),
                    description: String::from("The currently-loaded instruction. This is broken down into different fields, where each field serves a different purpose in identifying what the instruction does."),
                    value: datapath_state.mips.state.instruction as u64,
                    bits: 32,
                },
                "jump_address" => LineInformation {
                    title: String::from("Jump Address"),
                    description: String::from("The calculated next program counter for jump instructions. For I type instructions, this is the immediate value added to the data in rs1. For U type instructions, this is the immediate value shifted left by 2."),
                    value: datapath_state.mips.state.jump_address,
                    bits: 64,
                },
                "lower_26" => LineInformation {
                    title: String::from("Instruction [25-0]"),
                    description: String::from("The lower 26 bits of instruction. This is used as part of the new PC value for J-type instructions."),
                    value: datapath_state.mips.state.lower_26 as u64,
                    bits: 26,
                },
                "lower_26_shifted_left_by_2" => LineInformation {
                    title: String::from("Instruction [25-0] << 2"),
                    description: String::from("The lower 26 bits of instruction, shifted left by 2. This is used as part of the new PC value for J-type instructions."),
                    value: datapath_state.mips.state.lower_26_shifted_left_by_2 as u64,
                    bits: 28,
                },
                "mem_mux1_to_mem_mux2" => LineInformation {
                    title: String::from("Relative PC Address"),
                    description: String::from("Based on the control signals for branching and jumping, this address may be the next PC value. This is used for general non-branching instructions or branch-type instructions."),
                    value: datapath_state.mips.state.mem_mux1_to_mem_mux2,
                    bits: 64,
                },
                "memory_data" => LineInformation {
                    title: String::from("Memory Data"),
                    description: String::from("The data retrieved from memory, given that the MemRead control signal is set. This may be 32 bits or 64 bits, depending on the RegWidth control signal."),
                    value: datapath_state.mips.state.memory_data,
                    bits: 64,
                },
                "new_pc" => LineInformation {
                    title: String::from("New Program Counter"),
                    description: String::from("The address of the next instruction to execute. In other words, the next value of the program counter (PC) register."),
                    value: datapath_state.mips.state.new_pc,
                    bits: 64,
                },
                "pc" => LineInformation {
                    title: String::from("Program Counter"),
                    description: String::from("The address of the currently-executing instruction."),
                    value: datapath_state.mips.registers.pc,
                    bits: 64,
                },
                "pc_plus_4" => LineInformation {
                    title: String::from("PC + 4"),
                    description: String::from("The address of the currently-executing instruction, plus 4. By default, this will become the next value of the PC register. However, a different address may be used in the case of a branch or jump instruction."),
                    value: datapath_state.mips.state.pc_plus_4,
                    bits: 64,
                },
                "pc_plus_4_upper" => LineInformation {
                    title: String::from("PC + 4 [63-28]"),
                    description: String::from("The upper 36 bits of PC + 4. This is to be concatenated with the lower 26 bits of the instruction to calculate a jump address."),
                    value: datapath_state.mips.state.pc_plus_4 & 0xffff_ffff_f000_0000 >> 28,
                    bits: 36,
                },
                "ra_id" => LineInformation {
                    title: String::from("Return Address Register Index"),
                    description: String::from("The value 31. This represents the thirty-second register, the return address register ($ra)."),
                    value: 31,
                    bits: 5,
                },
                "rd" => LineInformation {
                    title: String::from("Instruction [15-11] (rd)"),
                    description: String::from("The rd field. Depending on the RegDst control signal, this will be the register written to for an instruction. This register is used as the destination for most R-type instructions."),
                    value: datapath_state.mips.state.rd as u64,
                    bits: 5,
                },
                "read_data_1" => LineInformation {
                    title: String::from("Read Data 1"),
                    description: String::from("Data retrieved from the register specified by the rs instruction field. Based on the instruction, this may be used as the first input to the ALU, or the next value of the PC register."),
                    value: datapath_state.mips.state.read_data_1,
                    bits: 64,
                },
                "read_data_2" => LineInformation {
                    title: String::from("Read Data 2"),
                    description: String::from("Data retrieved from the register specified by the rt instruction field. Based on the instruction, this may be used as the second input to the ALU, data written to memory, or data transferred to the floating-point coprocessor."),
                    value: datapath_state.mips.state.read_data_2,
                    bits: 64,
                },
                "register_write_data" => LineInformation {
                    title: String::from("Register Write Data"),
                    description: String::from("Data that will be written to a general-purpose register, given that RegWrite is set."),
                    value: datapath_state.mips.state.register_write_data,
                    bits: 64,
                },
                "relative_pc_branch" => LineInformation {
                    title: String::from("Relative PC Branch Address"),
                    description: String::from("The relative address used in branch instructions. This is the sum of PC + 4 and the sign-extended immediate value, shifted left by 2."),
                    value: datapath_state.mips.state.relative_pc_branch,
                    bits: 64,
                },
                "rs" => LineInformation {
                    title: String::from("Instruction [25-21] (rs)"),
                    description: String::from("The rs field. Contains the first register to be read for an instruction."),
                    value: datapath_state.mips.state.rs as u64,
                    bits: 5,
                },
                "rt" => LineInformation {
                    title: String::from("Instruction [20-16] (rt)"),
                    description: String::from("The rt field. Contains the second register to be read for an instruction."),
                    value: datapath_state.mips.state.rt as u64,
                    bits: 5,
                },
                "shamt" => LineInformation {
                    title: String::from("Instruction [10-6] (shamt)"),
                    description: String::from("The shamt (\"shift amount\") field. Specifies the number of bits to shift for those instructions that perform bit-shifting."),
                    value: datapath_state.mips.state.shamt as u64,
                    bits: 5,
                },
                "sign_extend" => LineInformation {
                    title: String::from("Sign-Extended Immediate"),
                    description: String::from("The immediate field, sign-extended to a 64-bit value."),
                    value: datapath_state.mips.state.sign_extend,
                    bits: 64,
                },
                "sign_extend_shift_left_by_2" => LineInformation {
                    title: String::from("Sign-Extended Immediate << 2"),
                    description: String::from("The immediate field, sign-extended to a 64-bit value, then shifted left by 2."),
                    value: datapath_state.mips.state.sign_extend_shift_left_by_2,
                    bits: 64,
                },
                "write_data" => LineInformation {
                    title: String::from("Memory Write Data"),
                    description: String::from("Given that the MemWrite control signal is set, this data will be written to memory."),
                    value: datapath_state.mips.state.write_data,
                    bits: 64,
                },
                "write_register" => LineInformation {
                    title: String::from("Write Register"),
                    description: String::from("The register that will be written to, assuming RegWrite is set. Depending on the RegDst control signal, this will consist of the rs, rt, or rd register, or 31 (indicating the $ra register)."),
                    value: datapath_state.mips.state.write_register_destination as u64,
                    bits: 5,
                },
                "zero_extended_immediate" => LineInformation {
                    title: String::from("Zero-Extended Immediate"),
                    description: String::from("The immediate field, zero-extended to a 64-bit value."),
                    value: datapath_state.mips.state.imm as u64,
                    bits: 64,
                },
                _ => LineInformation {
                    title: String::from("[Title]"),
                    description: String::from("[Description]"),
                    value: 0,
                    bits: 0,
                },
            }
        },
        AvailableDatapaths::RISCV => {
            match variable {
                "alu_input1" => LineInformation {
                    title: String::from("ALU Input 1"),
                    description: String::from("The first input to the ALU. This is determined by the OP1Select control signal to select between the current program counter, a sign-extended and left-shifted immediate value, or the data from rs1."),
                    value: datapath_state.riscv.state.alu_input1,
                    bits: 64,
                },
                "alu_input2" => LineInformation {
                    title: String::from("ALU Input 2"),
                    description: String::from("The second input to the ALU. This is determined by the ALUSrc control signal to select between register data and a sign-extended and left-shifted immediate value."),
                    value: datapath_state.riscv.state.alu_input2,
                    bits: 64,
                },
                "alu_result" => LineInformation {
                    title: String::from("ALU Result"),
                    description: String::from("The result of the calculation performed by the ALU. This is used either as an address to access memory or as a value that is saved into a register."),
                    value: datapath_state.riscv.state.alu_result,
                    bits: 64,
                },
                "data_result" => LineInformation {
                    title: String::from("Writeback Data"),
                    description: String::from("After finishing processing the instruction, this will either be the ALU result, data from memory, or PC + 4, based on the MemToReg control signal. This data is saved into registers."),
                    value: datapath_state.riscv.state.data_result,
                    bits: 64,
                },
                "funct3" => LineInformation {
                    title: String::from("Instruction [14-12] (funct)"),
                    description: String::from("The funct3 field. Controls the type of operation to execute for all RISC-V instructions except for U and J type instructions."),
                    value: datapath_state.riscv.state.funct3 as u64,
                    bits: 3,
                },
                "funct7" => LineInformation {
                    title: String::from("Instruction [31-25] (funct7)"),
                    description: String::from("The funct7 field. Controls the type of operation to execute for R type instructions."),
                    value: datapath_state.riscv.state.funct7 as u64,
                    bits: 7,
                },
                "imm" => LineInformation {
                    title: String::from("Immediate"),
                    description: String::from("The immediate field. This field is a different size depending on the instruction type, but it can be at most 20 bits."),
                    value: datapath_state.riscv.state.imm as u64,
                    bits: 20,
                },
                "imm_input" => LineInformation {
                    title: String::from("Instruction [31-12] (immediate)"),
                    description: String::from("The part of the instruction used to extract the imm field from an instruction. In RISC-V, depending on the instruction type, the imm field can be split up within an instruction (S, B, and J type) or even out of order (B and J type)."),
                    value: 0u64, // FIXME: Use the real line
                    bits: 20,
                },
                "instruction" => LineInformation {
                    title: String::from("Instruction"),
                    description: String::from("The currently-loaded instruction. This is broken down into different fields, where each field serves a different purpose in identifying what the instruction does."),
                    value: datapath_state.riscv.state.instruction as u64,
                    bits: 32,
                },
                "i_type_address" => LineInformation {
                    title: String::from("I type jump address"),
                    description: String::from("The jump address used for I type instructions. Calculated from the data in rs1 and imm."),
                    value: 0u64, // FIXME: Use the real line
                    bits: 16,
                },
                "jump_address" => LineInformation {
                    title: String::from("Jump Address"),
                    description: String::from("The calculated next program counter for jump instructions. For I type instructions, this is the immediate value added to the data in rs1. For U type instructions, this is the immediate value shifted left by 2."),
                    value: datapath_state.riscv.state.jump_address,
                    bits: 64,
                },
                "lower_26" => LineInformation {
                    title: String::from("Instruction [25-0]"),
                    description: String::from("The lower 26 bits of instruction. This is used as part of the new PC value for J-type instructions."),
                    value: datapath_state.riscv.state.lower_26 as u64,
                    bits: 26,
                },
                "lower_26_shifted_left_by_2" => LineInformation {
                    title: String::from("Instruction [25-0] << 2"),
                    description: String::from("The lower 26 bits of instruction, shifted left by 2. This is used as part of the new PC value for J-type instructions."),
                    value: datapath_state.riscv.state.lower_26_shifted_left_by_2 as u64,
                    bits: 28,
                },
                "mem_mux1_to_mem_mux2" => LineInformation {
                    title: String::from("Relative PC Address"),
                    description: String::from("Based on the control signals for branching and jumping, this address may be the next PC value. This is used for general non-branching instructions or branch-type instructions."),
                    value: datapath_state.riscv.state.mem_mux1_to_mem_mux2,
                    bits: 64,
                },
                "memory_data" => LineInformation {
                    title: String::from("Memory Data"),
                    description: String::from("The data retrieved from memory, given that the MemRead control signal is set. This may be 32 bits or 64 bits, depending on the RegWidth control signal."),
                    value: datapath_state.riscv.state.memory_data,
                    bits: 64,
                },
                "new_pc" => LineInformation {
                    title: String::from("New Program Counter"),
                    description: String::from("The address of the next instruction to execute. In other words, the next value of the program counter (PC) register."),
                    value: datapath_state.riscv.state.new_pc,
                    bits: 64,
                },
                "pc" => LineInformation {
                    title: String::from("Program Counter"),
                    description: String::from("The address of the currently-executing instruction."),
                    value: datapath_state.riscv.registers.pc,
                    bits: 64,
                },
                "pc_plus_4" => LineInformation {
                    title: String::from("PC + 4"),
                    description: String::from("The address of the currently-executing instruction, plus 4. By default, this will become the next value of the PC register. However, a different address may be used in the case of a branch or jump instruction."),
                    value: datapath_state.riscv.state.pc_plus_4,
                    bits: 64,
                },
                "pc_plus_4_upper" => LineInformation {
                    title: String::from("PC + 4 [63-28]"),
                    description: String::from("The upper 36 bits of PC + 4. This is to be concatenated with the lower 26 bits of the instruction to calculate a jump address."),
                    value: datapath_state.riscv.state.pc_plus_4 & 0xffff_ffff_f000_0000 >> 28,
                    bits: 36,
                },
                "ra_id" => LineInformation {
                    title: String::from("Return Address Register Index"),
                    description: String::from("The value 31. This represents the thirty-second register, the return address register ($ra)."),
                    value: 31,
                    bits: 5,
                },
                "rd" => LineInformation {
                    title: String::from("Instruction [11-7] (rd)"),
                    description: String::from("The rd field. This will be the register written to for an instruction."),
                    value: datapath_state.riscv.state.rd as u64,
                    bits: 5,
                },
                "read_data_1" => LineInformation {
                    title: String::from("Read Data 1"),
                    description: String::from("Data retrieved from the register specified by the rs1 instruction field. Based on the instruction, this may be used as the first input to the ALU, or in the calculation of the next value of the PC register."),
                    value: datapath_state.riscv.state.read_data_1,
                    bits: 64,
                },
                "read_data_2" => LineInformation {
                    title: String::from("Read Data 2"),
                    description: String::from("Data retrieved from the register specified by the rs2 instruction field. Based on the instruction, this may be used as the second input to the ALU or data written to memory."),
                    value: datapath_state.riscv.state.read_data_2,
                    bits: 64,
                },
                "register_write_data" => LineInformation {
                    title: String::from("Register Write Data"),
                    description: String::from("Data that will be written to a general-purpose register, given that RegWrite is set."),
                    value: datapath_state.riscv.state.register_write_data,
                    bits: 64,
                },
                "relative_pc_branch" => LineInformation {
                    title: String::from("Branch Address"),
                    description: String::from("The absolute address used in branch instructions. This is the sign-extended immediate value, shifted left by 2."),
                    value: datapath_state.riscv.state.relative_pc_branch,
                    bits: 64,
                },
                "rs1" => LineInformation {
                    title: String::from("Instruction [19-15] (rs1)"),
                    description: String::from("The rs1 field. Contains the first register to be read for an instruction."),
                    value: datapath_state.riscv.state.rs1 as u64,
                    bits: 5,
                },
                "rs2" => LineInformation {
                    title: String::from("Instruction [24-20] (rs2)"),
                    description: String::from("The rs2 field. Contains the second register to be read for an instruction."),
                    value: datapath_state.riscv.state.rs2 as u64,
                    bits: 5,
                },
                "shamt" => LineInformation {
                    title: String::from("Instruction [10-6] (shamt)"),
                    description: String::from("The shamt (\"shift amount\") field. Specifies the number of bits to shift for those instructions that perform bit-shifting."),
                    value: datapath_state.riscv.state.shamt as u64,
                    bits: 5,
                },
                "sign_extend" => LineInformation {
                    title: String::from("Sign-Extended Immediate"),
                    description: String::from("The immediate field, sign-extended to a 64-bit value."),
                    value: datapath_state.riscv.state.sign_extend,
                    bits: 64,
                },
                "sign_extend_shift_left_by_2" => LineInformation {
                    title: String::from("Sign-Extended Immediate << 2"),
                    description: String::from("The immediate field, sign-extended to a 64-bit value, then shifted left by 2."),
                    value: datapath_state.riscv.state.sign_extend_shift_left_by_2,
                    bits: 64,
                },
                "write_data" => LineInformation {
                    title: String::from("Memory Write Data"),
                    description: String::from("Given that the MemWrite control signal is set, this data will be written to memory."),
                    value: datapath_state.riscv.state.write_data,
                    bits: 64,
                },
                "write_register" => LineInformation {
                    title: String::from("Write Register"),
                    description: String::from("The register that will be written to, assuming RegWrite is set. Depending on the RegDst control signal, this will consist of the rs, rt, or rd register, or 31 (indicating the $ra register)."),
                    value: datapath_state.riscv.state.write_register_destination as u64,
                    bits: 5,
                },
                "zero_extended_immediate" => LineInformation {
                    title: String::from("Zero-Extended Immediate"),
                    description: String::from("The immediate field, zero-extended to a 64-bit value."),
                    value: datapath_state.riscv.state.imm as u64,
                    bits: 64,
                },
                _ => LineInformation {
                    title: String::from("[Title]"),
                    description: String::from("[Description]"),
                    value: 0,
                    bits: 0,
                },
            }
        }
    }
}
