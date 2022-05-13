select
  max(t.num) as num
from
  (
    SELECT
      num,
      count(*) as cnt
    from
      MyNumbers
    group by
      num
    having
      cnt = 1
  ) t
