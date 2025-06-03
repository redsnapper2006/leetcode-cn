select
  paid_ua.user_id,
  round(free_ua.avg_ad, 2) as trial_avg_duration,
  round(paid_ua.avg_ad, 2) as paid_avg_duration
from
  (
    select
      ua.user_id,
      avg(ua.activity_duration) as avg_ad
    from
      UserActivity ua
    where
      ua.activity_type = 'free_trial'
    group by
      ua.user_id
  ) free_ua,
  (
    select
      ua.user_id,
      avg(ua.activity_duration) as avg_ad
    from
      UserActivity ua
    where
      ua.activity_type = 'paid'
    group by
      ua.user_id
  ) paid_ua
where
  free_ua.user_id = paid_ua.user_id
order by
  paid_ua.user_id
