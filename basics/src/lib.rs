mod advanced;
mod concurrency;
mod oop;
mod patterns;
mod smart_pointers;

pub fn test_smart_pointers() {
  println!("* test_smart_pointers");
  smart_pointers::box_save_to_heap();
  smart_pointers::implement_cons_list_use_box();
  smart_pointers::implement_custom_smart_pointer();
  println!("");
}

pub fn test_concurrency() {
  println!("* test_concurrency");
  concurrency::threads::value_move_into_closure();
  concurrency::message_passing::send_single_message();
  concurrency::message_passing::send_multiple_messages();
  concurrency::message_passing::send_messages_from_multiple_producer();
  concurrency::share::lock_use_mutex();
  println!("");
}

pub fn test_oop() {
  println!("* test_oop");
  oop::implementing_the_trait();
  oop::implementing_state_design_pattern_as_trait();
  oop::implementing_state_design_pattern_as_type();
  println!("");
}

pub fn test_patterns() {
  println!("* test_patterns");
  patterns::destructure_structs();
  patterns::destructure_structs_in_match();
  patterns::destructure_enums();
  patterns::destructure_reference();
  patterns::destructure_structs_and_tuples();
  patterns::create_reference();
  patterns::create_mutable_reference();
  patterns::match_guards();
  patterns::at_operator_bindings();
  println!("");
}

pub fn test_advanced() {
  println!("* test_advanced");
  advanced::unsafe_rust::create_raw_pointer();
  advanced::unsafe_rust::dereference_raw_pointer();
  advanced::unsafe_rust::call_the_dangerous_function();
  advanced::unsafe_rust::create_safe_abstract_on_unsafe_code();
  advanced::unsafe_rust::use_the_ffi();
  advanced::unsafe_rust::access_to_static_mutable_variable();
  advanced::lifetime::lifetime_subtyping();
  advanced::lifetime::lifetime_bound();
  advanced::lifetime::lifetime_of_trait_object();
  advanced::traits::default_generic_type_parameter_and_operator_overloading();
  advanced::traits::use_fully_qualified_syntax();
  advanced::traits::use_super_trait();
  advanced::traits::use_newtype_pattern();
  advanced::types::use_type_alias();
  advanced::types::allow_dynamically_sized_type_with_generic();
  println!("");
}
