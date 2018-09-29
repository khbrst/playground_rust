mod smart_pointers;

pub fn test_smart_pointers() {
  smart_pointers::box_save_to_heap();
  smart_pointers::implement_cons_list_use_box();
  smart_pointers::implement_custom_smart_pointer();
}
