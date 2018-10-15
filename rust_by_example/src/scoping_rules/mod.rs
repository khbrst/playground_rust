mod raii;

pub fn run_raii() {
  raii::run();
  raii::destructor();
}
