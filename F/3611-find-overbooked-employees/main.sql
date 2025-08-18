select
  e.employee_id,
  e.employee_name,
  e.department,
  aggr_meet.meeting_heavy_weeks
from
  employees e,
  (
    select
      sum_meet.employee_id,
      count(*) as meeting_heavy_weeks
    from
      (
        select
          DATE_ADD(m.meeting_date, INTERVAL(-WEEKDAY(m.meeting_date)) DAY) as WEEK,
          SUM(m.duration_hours) as SUM_HOURS,
          m.employee_id
        from
          meetings m
        group by
          m.employee_id,
          WEEK
        having
          SUM_HOURS > 20.0
      ) sum_meet
    group by
      sum_meet.employee_id
  ) aggr_meet
where
  aggr_meet.employee_id = e.employee_id
  and aggr_meet.meeting_heavy_weeks >= 2
order by
  aggr_meet.meeting_heavy_weeks desc,
  e.employee_name
