select
  gender,
  day,
  sum(score_points) over (
    order by
      gender,
      day
  ) as total
from
  Scores
where
  gender = 'F'
UNION
ALL
select
  gender,
  day,
  sum(score_points) over (
    order by
      gender,
      day
  ) as total
from
  Scores
where
  gender = 'M'
