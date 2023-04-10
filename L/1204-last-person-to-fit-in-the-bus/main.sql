SELECT tt.person_name
FROM
(
  SELECT t.person_name, @ss:=@ss+t.weight AS cumu
  FROM
  (
    SELECT person_name, weight
    FROM queue
    ORDER BY turn
  ) t
  JOIN (SELECT @ss:=0) r
) tt
WHERE tt.cumu <= 1000
ORDER BY tt.cumu DESC
LIMIT 1;
