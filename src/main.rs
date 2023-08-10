mod env;
mod space;

use env::Env;

fn main() {
  let mut env: Env = Env::new("Game", 800, 600);
  env.run();
}