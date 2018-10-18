mod threads;

pub fn run_threads() {
  threads::basic();
  threads::map_reduce();
  threads::improved_map_reduce();
}
