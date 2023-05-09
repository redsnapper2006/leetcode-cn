SELECT SUM(b.apple_count + IF(c.apple_count IS NULL, 0, c.apple_count)) AS apple_count,
       SUM(b.orange_count + IF(c.orange_count IS NULL, 0, c.orange_count)) AS orange_count
FROM Boxes b
LEFT JOIN Chests c ON b.chest_id = c.chest_id
