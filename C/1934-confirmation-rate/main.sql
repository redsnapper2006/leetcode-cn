SELECT
  s.user_id,
  round(
    ifnull(
      (
        select
          count(action)
        from
          Confirmations c1
        where
          c1.user_id = s.user_id
          and c1.action = 'confirmed'
      ) / (
        select
          count(action)
        from
          Confirmations c2
        where
          c2.user_id = s.user_id
      ),
      0
    ),
    2
  ) as confirmation_rate
from
  Signups s
