with
  tt as (
    select
      driver_id,
      floor((month (trip_date) -1) / 6) as month,
      avg(distance_km / fuel_consumed) as avg_cost
    from
      trips
    group by
      driver_id,
      month
  )
select
  t1.driver_id,
  d.driver_name,
  round(t1.avg_cost, 2) as first_half_avg,
  round(t2.avg_cost, 2) as second_half_avg,
  round(t2.avg_cost - t1.avg_cost, 2) as efficiency_improvement
from
  tt t1,
  tt t2,
  drivers d
where
  t1.driver_id = t2.driver_id
  and t1.driver_id = d.driver_id
  and t1.month = 0
  and t2.month = 1
  and t1.avg_cost < t2.avg_cost
order by
  efficiency_improvement desc,
  d.driver_name
