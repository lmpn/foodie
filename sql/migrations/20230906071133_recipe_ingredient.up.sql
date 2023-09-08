-- Add up migration script here
CREATE TABLE IF NOT EXISTS recipe_ingredient(
    recipe_uuid VARCHAR(16),
    ingredient_uuid VARCHAR(16),
    CONSTRAINT recipe_ingredient_recipe foreign key (recipe_uuid) references recipe,
    CONSTRAINT recipe_ingredient_ingredient foreign key (ingredient_uuid) references ingredient,
    CONSTRAINT recipe_ingredient_unique unique (recipe_uuid, ingredient_uuid),
    CONSTRAINT fk_recipe foreign key (recipe_uuid) references recipe(uuid) on delete cascade,
    CONSTRAINT fk_ingredient foreign key (ingredient_uuid) references ingredient(uuid) on delete cascade
)