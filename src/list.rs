pub const LEVELS_2D: [&str; 35] = [
    "alu_1",
    "alu_2",
    "and_gate_3",
    "any_doubles",
    "bit_adder",
    "bit_inverter",
    "byte_adder",
    "byte_constant",
    "byte_equal",
    "byte_less",
    "byte_less_i",
    "byte_mux",
    "byte_not",
    "byte_or",
    "byte_shift",
    "byte_xor",
    "conditions",
    "counting_signals",
    "decoder",
    "decoder1",
    "decoder3",
    "full_adder",
    "multiply",
    "not_gate",
    "odd_number_of_signals",
    "odd_ticks",
    "or_gate_3",
    "ram_component",
    "saving_bytes",
    "saving_gracefully",
    "second_tick",
    "signed_negator",
    "stack",
    "xnor",
    "xor_gate",
];

pub const LEVELS_3D: [&str; 16] = [
    "ai_showdown",
    "binary_programming",
    "binary_search",
    "capitalize",
    "circumference",
    "compute_xor",
    "dance",
    "divide",
    "flood_predictor",
    "maze",
    "mod_4",
    "ram",
    "robot_racing",
    "sorter",
    "tick_tock",
    "tower",
];

pub fn handle() {
    println!("Available circuit levels:");
    for level in LEVELS_2D {
        println!("- {level}");
    }
    println!();
    println!("Available programming levels:");
    for level in LEVELS_3D {
        println!("- {level}");
    }
}
