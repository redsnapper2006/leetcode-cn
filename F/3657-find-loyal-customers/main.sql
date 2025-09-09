select
  t.customer_id
from
  (
    select
      customer_id,
      count(1) as ct_cnt,
      sum(if (transaction_type = 'refund', 1, 0)) as rf_cnt,
      datediff (max(transaction_date), min(transaction_date)) as diff
    from
      customer_transactions
    group by
      customer_id
    order by
      customer_id
  ) t
where
  t.diff >= 30
  and t.ct_cnt >= 3
  and t.rf_cnt * 100 / t.ct_cnt < 20
order by
  t.customer_id
