-- Add up migration script here
CREATE TABLE IF NOT EXISTS recipe_ingredient(
    recipe_id INTEGER,
    ingredient_id INTEGER,
    CONSTRAINT recipe_ingredient_recipe foreign key (recipe_id) references recipe,
    CONSTRAINT recipe_ingredient_ingredient foreign key (ingredient_id) references ingredient,
    CONSTRAINT recipe_ingredient_unique unique (recipe_id, ingredient_id)
)