mod custom_types;
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
