select
  se.user_id,
  se.plan_name as current_plan,
  se.monthly_amount as current_monthly_amount,
  se_date.mx_ma as max_historical_amount,
  se_date.diff as days_as_subscriber
from
  (
    select
      user_id,
      min(event_date) as mn_ed,
      max(event_date) as mx_ed,
      max(monthly_amount) as mx_ma,
      sum(if (event_type = 'downgrade', 1, 0)) as cnt_dg,
      datediff (max(event_date), min(event_date)) as diff
    from
      subscription_events
    group by
      user_id
    having
      diff > 60
      and cnt_dg > 0
  ) as se_date,
  subscription_events se
where
  se.event_date = se_date.mx_ed
  and se.user_id = se_date.user_id
  and se.event_type != 'cancel'
  and se.monthly_amount * 2 < se_date.mx_ma
order by
  se_date.diff desc,
  se.user_id
