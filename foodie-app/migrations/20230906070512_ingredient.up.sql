-- Add up migration script here
CREATE TABLE IF NOT EXISTS ingredient (
    uuid VARCHAR(16) PRIMARY KEY NOT NULL,
    name VARCHAR(255) NOT NULL,
    amount DOUBLE NOT NULL,
    unit VARCHAR(10) NOT NULL,
    recipe_uuid VARCHAR(16) NOT NULL,
    CONSTRAINT ingredient_recipe foreign key (recipe_uuid) references recipe,
    CONSTRAINT fk_recipe foreign key (recipe_uuid) references recipe(uuid) on delete cascade,
    CONSTRAINT ingredient_unique unique (uuid)
);