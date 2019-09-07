#[allow(dead_code)]
pub fn lifetime_subtyping() {
  struct Context<'s>(&'s str);

  struct Parser<'c, 's: 'c> {
    context: &'c Context<'s>,
  }

  impl<'c, 's> Parser<'c, 's> {
    fn parse(&self) -> Result<(), &'s str> {
      Err(&self.context.0[1..])
    }
  }

  fn parse_context(context: Context) -> Result<(), &str> {
    Parser { context: &context }.parse()
  }
}

#[allow(dead_code)]
pub fn lifetime_bound() {
  struct Ref<'a, T: 'a>(&'a T);
}

#[allow(dead_code, unused_variables)]
pub fn lifetime_of_trait_object() {
  trait Red {}

  struct Ball<'a> {
    diameter: &'a i32,
  }

  impl<'a> Red for Ball<'a> {}

  let num = 5;

  let obj = Box::new(Ball { diameter: &num }) as Box<dyn Red>;
}
