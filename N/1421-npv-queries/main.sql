select
  q.id,
  q.year,
  ifnull(n.npv, 0) as npv
from
  Queries q
  left outer join NPV n on n.id = q.id
  and n.year = q.year
order by
  q.id,
  q.year
