select
  p1.project_id
from
  (
    SELECT
      project_id,
      count(*) as cnt
    FROM
      Project
    GROUP by
      project_id
  ) p1,
  (
    SELECT
      project_id,
      count(*) as cnt
    FROM
      Project
    GROUP by
      project_id
    order by
      cnt desc
    limit
      1
  ) p2
where
  p1.cnt = p2.cnt
