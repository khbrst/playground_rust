mod concurrency;
mod oop;
mod patterns;
mod smart_pointers;

pub fn test_smart_pointers() {
  smart_pointers::box_save_to_heap();
  smart_pointers::implement_cons_list_use_box();
  smart_pointers::implement_custom_smart_pointer();
}

pub fn test_concurrency() {
  concurrency::threads::value_move_into_closure();
  concurrency::message_passing::send_single_message();
  concurrency::message_passing::send_multiple_messages();
  concurrency::message_passing::send_messages_from_multiple_producer();
  concurrency::share::lock_use_mutex();
}

pub fn test_oop() {
  oop::implementing_the_trait();
  oop::implementing_state_design_pattern_as_trait();
  oop::implementing_state_design_pattern_as_type();
}

pub fn test_patterns() {
  patterns::destructure_structs();
  patterns::destructure_structs_in_match();
  patterns::destructure_enums();
}