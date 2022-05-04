with p as (
  select
    seller_id,
    sum(price) as sumprice
  from
    Sales
  group by
    seller_id
)
select
  t.seller_id
from
  p t
where
  t.sumprice = (
    select
      max(p.sumprice) as maxsumprice
    from
      p
  )
