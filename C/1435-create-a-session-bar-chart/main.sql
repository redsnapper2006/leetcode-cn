SELECT
  '[0-5>' as bin,
  (
    SELECT
      count(*)
    FROM
      Sessions
    WHERE
      duration < 300
  ) as total
UNION
SELECT
  '[5-10>' as bin,
  (
    SELECT
      count(*)
    FROM
      Sessions
    WHERE
      duration >= 300
      and duration < 600
  ) as total
UNION
SELECT
  '[10-15>' as bin,
  (
    SELECT
      count(*)
    FROM
      Sessions
    WHERE
      duration >= 600
      and duration < 900
  ) as total
UNION
SELECT
  '15 or more' as bin,
  (
    SELECT
      count(*)
    FROM
      Sessions
    WHERE
      duration >= 900
  ) as total
