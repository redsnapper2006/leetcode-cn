select
  transaction_date,
  sum(
    CASE
      WHEN mod(amount, 2) = 0 THEN 0
      ELSE amount
    END
  ) odd_sum,
  sum(
    CASE
      WHEN mod(amount, 2) = 1 THEN 0
      ELSE amount
    END
  ) even_sum
from
  transactions
group by
  transaction_date
order by
  transaction_date
