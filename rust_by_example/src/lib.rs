mod conversion;
mod custom_types;
mod expressions;
mod flow_control;
mod functions;
mod hello_world;
mod primitives;
mod types;
mod variable_bindings;

pub fn run_hello_world() {
  hello_world::print_hello_world();
  hello_world::formatted_print();
  hello_world::debug_print();
  hello_world::pretty_print();
  hello_world::display_print();
  hello_world::display_print_for_list();
  hello_world::formatting_print();
}

pub fn run_primitives() {
  primitives::check_basics();
  primitives::check_literals_and_operators();
  primitives::check_tuples();
  primitives::check_arrays_and_slices();
}

pub fn run_custom_types() {
  custom_types::check_structs();
  custom_types::check_enums();
  custom_types::use_enums();
  custom_types::c_like_enums();
  custom_types::linked_list_enums();
  custom_types::check_constants();
}

pub fn run_variable_bindings() {
  variable_bindings::infer_type_by_compiler();
  variable_bindings::use_mutable_binding();
  variable_bindings::check_scope_and_shadowing();
  variable_bindings::declare_first();
}

pub fn run_types() {
  types::casting();
  types::literals();
  types::inference();
  types::aliasing();
}

pub fn run_conversion() {
  conversion::from_and_into();
  conversion::to_string();
  conversion::parsing_a_string();
}

pub fn run_expressions() {
  expressions::check();
}

pub fn run_flow_control() {
  flow_control::if_else();
  flow_control::do_loop();
  flow_control::returning_from_loop();
  flow_control::run_while();
  flow_control::for_and_ranges();
  flow_control::for_and_iterators();
  flow_control::do_match();
  flow_control::destructuring_tuples_use_match();
  flow_control::destructuring_enums_use_match();
  flow_control::destructuring_pointers_use_match();
  flow_control::destructuring_structs();
  flow_control::match_guards();
  flow_control::match_binding();
  flow_control::if_let();
  flow_control::while_let();
}

pub fn run_functions() {
  functions::call_functions();
  functions::call_methods();
  functions::closure();
  functions::closure_capturing();
  functions::closure_capturing_use_move();
  functions::closure_as_input_parameter();
  functions::closure_type_anonymity();
  functions::closure_input_function();
  functions::closure_as_output_parameter();
}
