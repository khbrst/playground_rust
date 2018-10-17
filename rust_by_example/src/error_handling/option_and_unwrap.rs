pub fn basic() {
  // The commoner has seen it all, and can handle any gift well.
  // All gifts are handled explicitly using `match`.
  fn give_commoner(gift: Option<&str>) {
    // Specify a course of action for each case.
    match gift {
      Some("snake") => println!("Yuck! I'm throwing that snake in a fire."),
      Some(inner) => println!("{}? How nice.", inner),
      None => println!("No gift? Oh well."),
    }
  }

  // Our sheltered princess will `panic` at the sight of snakes.
  // All gifts are handled implicitly using `unwrap`.
  fn give_princess(gift: Option<&str>) {
    // `unwrap` returns a `panic` when it receives a `None`.
    let inside = gift.unwrap();
    if inside == "snake" {
      panic!("AAAaaaaa!!!!");
    }

    println!("I love {}s!!!!!", inside);
  }

  let food = Some("cabbage");
  let snake = Some("snake");
  let void = None;

  give_commoner(food);
  give_commoner(snake);
  give_commoner(void);

  let bird = Some("robin");
  let _nothing: Option<&str> = None;

  give_princess(bird);
  // give_princess(_nothing);
}

pub fn combinators_map() {
  #[allow(dead_code)]
  #[derive(Debug)]
  enum Food {
    Apple,
    Carrot,
    Potato,
  }

  #[derive(Debug)]
  struct Peeled(Food);
  #[derive(Debug)]
  struct Chopped(Food);
  #[derive(Debug)]
  struct Cooked(Food);

  // Peeling food. If there isn't any, then return `None`.
  // Otherwise, return the peeled food.
  fn peel(food: Option<Food>) -> Option<Peeled> {
    match food {
      Some(food) => Some(Peeled(food)),
      None => None,
    }
  }

  // Chopping food. If there isn't any, then return `None`.
  // Otherwise, return the chopped food.
  fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
    match peeled {
      Some(Peeled(food)) => Some(Chopped(food)),
      None => None,
    }
  }

  // Cooking food. Here, we showcase `map()` instead of `match` for case handling.
  fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
    chopped.map(|Chopped(food)| Cooked(food))
  }

  // A function to peel, chop, and cook food all in sequence.
  // We chain multiple uses of `map()` to simplify the code.
  fn process(food: Option<Food>) -> Option<Cooked> {
    food
      .map(|f| Peeled(f))
      .map(|Peeled(f)| Chopped(f))
      .map(|Chopped(f)| Cooked(f))
  }

  // Check whether there's food or not before trying to eat it!
  fn eat(food: Option<Cooked>) {
    match food {
      Some(food) => println!("Mmm. I love {:?}", food),
      None => println!("Oh no! It wasn't edible."),
    }
  }

  let apple = Some(Food::Apple);
  let carrot = Some(Food::Carrot);
  let potato = None;

  let cooked_apple = cook(chop(peel(apple)));
  let cooked_carrot = cook(chop(peel(carrot)));
  // Let's try the simpler looking `process()` now.
  let cooked_potato = process(potato);

  eat(cooked_apple);
  eat(cooked_carrot);
  eat(cooked_potato);
}

pub fn combinators_and_then() {
  #[derive(Debug)]
  enum Food {
    CordonBleu,
    Steak,
    Sushi,
  }
  #[derive(Debug)]
  enum Day {
    Monday,
    Tuesday,
    Wednesday,
  }

  // We don't have the ingredients to make Sushi.
  fn have_ingredients(food: Food) -> Option<Food> {
    match food {
      Food::Sushi => None,
      _ => Some(food),
    }
  }

  // We have the recipe for everything except Cordon Bleu.
  fn have_recipe(food: Food) -> Option<Food> {
    match food {
      Food::CordonBleu => None,
      _ => Some(food),
    }
  }

  #[allow(dead_code)]
  // To make a dish, we need both the ingredients and the recipe.
  // We can represent the logic with a chain of `match`es:
  fn cookable_v1(food: Food) -> Option<Food> {
    match have_ingredients(food) {
      None => None,
      Some(food) => match have_recipe(food) {
        None => None,
        Some(food) => Some(food),
      },
    }
  }

  // This can conveniently be rewritten more compactly with `and_then()`:
  fn cookable_v2(food: Food) -> Option<Food> {
    have_ingredients(food).and_then(have_recipe)
  }

  fn eat(food: Food, day: Day) {
    match cookable_v2(food) {
      Some(food) => println!("Yay! On {:?} we get to eat {:?}.", day, food),
      None => println!("Oh no. We don't get to eat on {:?}?", day),
    }
  }

  let (cordon_bleu, steak, sushi) = (Food::CordonBleu, Food::Steak, Food::Sushi);

  eat(cordon_bleu, Day::Monday);
  eat(steak, Day::Tuesday);
  eat(sushi, Day::Wednesday);
}
