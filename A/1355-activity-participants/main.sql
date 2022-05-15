WITH ac as (
  SELECT
    activity,
    count(*) as cnt
  FROM
    Friends
  group by
    activity
)
SELECT
  activity
FROM
  ac
where
  ac.cnt < (
    select
      max(cnt)
    from
      ac
  )
  and ac.cnt > (
    select
      min(cnt)
    from
      ac
  )
