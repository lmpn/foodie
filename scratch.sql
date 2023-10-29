-- database: /Users/luisneto/Documents/dev/foodie/sql/test.db

-- Use the ▷ button in the top right corner to run the entire file.

UPDATE ingredient SET name = "ingredient2", amount = 1.0, unit = "KG"
WHERE uuid = "71369831-36a5-49c2-adb0-96368bd7a02b" AND recipe_uuid = "d6c409f8-585a-43f0-9e12-eeab634ff226";
