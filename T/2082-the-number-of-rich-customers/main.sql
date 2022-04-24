select
  count(*) as rich_count
from
  (
    SELECT
      distinct customer_id
    FROM
      Store
    where
      amount > 500
  ) p
