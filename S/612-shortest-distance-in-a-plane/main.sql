SELECT ROUND(SQRT(t.result), 2) AS shortest
FROM (
  SELECT (p1.x - p2.x) * (p1.x - p2.x) + (p1.y - p2.y) * (p1.y - p2.y) result
  FROM (
    SELECT ROW_NUMBER() OVER (ORDER BY x, y) row_num, x, y
    FROM Point2D
  ) p1,
  (
    SELECT ROW_NUMBER() OVER (ORDER BY x, y) row_num, x, y
    FROM Point2D
  ) p2
  WHERE p1.row_num > p2.row_num
  ORDER BY result 
  LIMIT 1
) t

