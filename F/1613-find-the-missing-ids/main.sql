WITH RECURSIVE nums AS (
  SELECT
    1 AS value
  UNION
  ALL
  SELECT
    value + 1 AS value
  FROM
    nums
  WHERE
    nums.value < (
      select
        max(customer_id)
      from
        Customers
    )
)
select
  value as ids
from
  nums
where
  value not in (
    select
      customer_id
    from
      customers
  )
