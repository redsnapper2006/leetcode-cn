SELECT t.id, @prev := IFNULL(t.drink, @prev) drink
FROM CoffeeShop t

