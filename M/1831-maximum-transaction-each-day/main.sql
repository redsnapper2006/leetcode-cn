select
  t.transaction_id
from
  Transactions t,
  (
    select
      day(day) as d,
      max(amount) as max
    from
      Transactions
    group by
      d
  ) sub
where
  day(day) = sub.d
  and t.amount = sub.max
order by
  t.transaction_id
