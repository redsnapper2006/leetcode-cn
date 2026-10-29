select
  m.user_id,
  m.reaction as dominant_reaction,
  ROUND(m.cnt * 1.0 / m.total_cnt, 2) as reaction_ratio
from
  (
    select
      user_id,
      reaction,
      cnt,
      sum(cnt) over (
        partition by
          user_id
      ) as total_cnt
    from
      (
        select
          user_id,
          reaction,
          count(*) as cnt
        from
          reactions
        group by
          user_id,
          reaction
      ) as pre
  ) as m
where
  m.total_cnt >= 5
  and m.total_cnt * 3 <= m.cnt * 5
order by
  reaction_ratio desc,
  user_id
