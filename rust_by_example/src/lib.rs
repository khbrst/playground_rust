mod hello_world;

pub fn run_hello_world() {
  hello_world::print_hello_world();
  hello_world::formatted_print();
  hello_world::debug_print();
  hello_world::pretty_print();
  hello_world::display_print();
  hello_world::display_print_for_list();
  hello_world::formatting_print();
}