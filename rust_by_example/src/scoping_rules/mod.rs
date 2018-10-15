mod raii;
mod ownership_and_moves;

pub fn run_raii() {
  raii::run();
  raii::destructor();
}

pub fn run_ownership_and_moves() {
  ownership_and_moves::basic();
  ownership_and_moves::mutability();
}