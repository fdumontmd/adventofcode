extern crate regex;

use std::collections::HashMap;
use std::io::{self, Read};
use std::str::FromStr;

use regex::Regex;

struct Ingredient {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

impl Ingredient {
    fn new(capacity: i64, durability: i64, flavor: i64, texture: i64, calories: i64) -> Self {
        Ingredient{
            capacity: capacity,
            durability: durability,
            flavor: flavor,
            texture: texture,
            calories: calories,
        }
    }
}

struct Cookie<'s> {
    ingredients: HashMap<&'s str, Ingredient>,
}

impl<'s> Cookie<'s> {
    fn new() -> Self {
        Cookie{
            ingredients: HashMap::new(),
        }
    }

    fn add_ingredient(&mut self, name: &'s str, ingredient: Ingredient) {
        self.ingredients.insert(name, ingredient);
    }

    fn score(&self, recipe: &HashMap<&'s str, i64>) -> i64 {
        let mut capacity = 0;
        let mut durability = 0;
        let mut flavor = 0;
        let mut texture = 0;

        for (ingredient, teaspoon) in recipe {
            capacity += self.ingredients.get(ingredient).unwrap().capacity * teaspoon;
            durability += self.ingredients.get(ingredient).unwrap().durability * teaspoon;
            flavor += self.ingredients.get(ingredient).unwrap().flavor * teaspoon;
            texture += self.ingredients.get(ingredient).unwrap().texture * teaspoon;
        }

        if capacity < 0 || durability < 0 || flavor < 0 || texture < 0 {
            return 0;
        }

        capacity * durability * flavor * texture
    }

    fn calories(&self, recipe: &HashMap<&'s str, i64>) -> i64 {
        let mut calories = 0;
        for (ingredient, teaspoon) in recipe {
            calories += self.ingredients.get(ingredient).unwrap().calories * teaspoon;
        }
        calories
    }
}

struct Search<'s> {
    cookie: Cookie<'s>,
    max_teaspoon: i64,
}

impl<'s> Search<'s> {
    fn new(cookie: Cookie<'s>, max_teaspoon: i64) -> Self {
        Search{
            cookie: cookie,
            max_teaspoon: max_teaspoon,
        }
    }

    fn build_recipe(&self, ingredients: &[&'s str], remaining: i64, current: &mut HashMap<&'s str, i64>, best_score: &mut Option<i64>, best: &mut HashMap<&'s str, i64>) {
        if ingredients.len() == 1 {
            current.insert(ingredients[0], remaining);

            let current_score = self.cookie.score(current);
            if best_score.is_none() || best_score.unwrap() < current_score {
                *best_score = Some(current_score);
                *best = current.clone();
            }
        } else {
            for ts in 0..remaining+1 {
                current.insert(ingredients[0], ts);
                self.build_recipe(&ingredients[1..], remaining - ts, current, best_score, best);
            }
        }
    }

    fn build_recipe_under(&self, ingredients: &[&'s str], remaining: i64, calories: i64, current: &mut HashMap<&'s str, i64>, best_score: &mut Option<i64>, best: &mut HashMap<&'s str, i64>) {
        if ingredients.len() == 1 {
            current.insert(ingredients[0], remaining);

            if self.calories(current) == calories {
                let current_score = self.cookie.score(current);
                if best_score.is_none() || best_score.unwrap() < current_score {
                    *best_score = Some(current_score);
                    *best = current.clone();
                }
            }
        } else {
            for ts in 0..remaining+1 {
                current.insert(ingredients[0], ts);
                self.build_recipe_under(&ingredients[1..], remaining - ts, calories, current, best_score, best);
            }
        }
    }

    fn best_recipe(&self) -> HashMap<&'s str, i64> {
        let ingredients: Vec<&str> = self.cookie.ingredients.keys().cloned().collect();
        let mut best_score = None;
        let mut best = HashMap::new();

        self.build_recipe(ingredients.as_slice(), self.max_teaspoon, &mut HashMap::new(), &mut best_score, &mut best);

        best
    }

    fn best_recipe_under(&self, calories: i64) -> HashMap<&'s str, i64> {
        let ingredients: Vec<&str> = self.cookie.ingredients.keys().cloned().collect();
        let mut best_score = None;
        let mut best = HashMap::new();

        self.build_recipe_under(ingredients.as_slice(), self.max_teaspoon, calories, &mut HashMap::new(), &mut best_score, &mut best);

        best
    }

    fn score(&self, recipe: &HashMap<&'s str, i64>) -> i64 {
        self.cookie.score(recipe)
    }

    fn calories(&self, recipe: &HashMap<&'s str, i64>) -> i64 {
        self.cookie.calories(recipe)
    }
}

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer).unwrap();

    let re = Regex::new(r"(\w+): capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)").unwrap();

    let mut cookie = Cookie::new();

    for line in buffer.lines() {
        if let Some(ref caps) = re.captures(line) {
            let ingredient = caps.at(1).unwrap();
            let capacity = i64::from_str(caps.at(2).unwrap()).unwrap();
            let durability = i64::from_str(caps.at(3).unwrap()).unwrap();
            let flavor = i64::from_str(caps.at(4).unwrap()).unwrap();
            let texture = i64::from_str(caps.at(5).unwrap()).unwrap();
            let calories = i64::from_str(caps.at(6).unwrap()).unwrap();

            cookie.add_ingredient(ingredient, Ingredient::new(capacity, durability, flavor, texture, calories));
        }
    }

    let search = Search::new(cookie, 100);

    let best = search.best_recipe();
    println!("Best recipe: {:?}, with score: {}", best, search.score(&best));

    let best = search.best_recipe_under(500);
    println!("Best diet recipe: {:?}, with score: {}", best, search.score(&best));
    println!("Calories for recipe: {}", search.calories(&best));
}

#[test]
fn test() {
    let butterscotch = Ingredient::new(-1, -2, 6, 3, 8);
    let cinnamon = Ingredient::new(2, 3, -2, -1, 3);
    let mut cookie = Cookie::new();
    cookie.add_ingredient("Butterscotch", butterscotch);
    cookie.add_ingredient("Cinnamon", cinnamon);

    let mut recipe = HashMap::new();
    recipe.insert("Butterscotch", 44);
    recipe.insert("Cinnamon", 56);

    assert_eq!(62842880, cookie.score(&recipe));
    let search = Search::new(cookie, 100);

    let best = search.best_recipe();
    let best_score = search.score(&best);
    assert_eq!(62842880, best_score);

    let best = search.best_recipe_under(500);
    let best_score = search.score(&best);
    assert_eq!(57600000, best_score);
}
