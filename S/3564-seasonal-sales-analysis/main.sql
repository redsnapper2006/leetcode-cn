select
  ttt.season,
  ttt.category,
  ttt.total_quantity,
  ttt.total_revenue
from
  (
    select
      tt.season,
      tt.category,
      tt.total_quantity,
      tt.total_revenue,
      ROW_NUMBER() OVER (
        PARTITION BY
          tt.season
      ) as row_num
    from
      (
        select
          t.season,
          t.category,
          sum(t.quantity) as total_quantity,
          sum(t.total) as total_revenue
        from
          (
            select
              case MID (s.sale_date, 6, 2)
                when '12' then 'Winter'
                when '01' then 'Winter'
                when '02' then 'Winter'
                when '03' then 'Spring'
                when '04' then 'Spring'
                when '05' then 'Spring'
                when '06' then 'Summer'
                when '07' then 'Summer'
                when '08' then 'Summer'
                when '09' then 'Fall'
                when '10' then 'Fall'
                when '11' then 'Fall'
              END as season,
              p.category,
              s.quantity,
              s.quantity * s.price as total
            from
              sales s,
              products p
            where
              s.product_id = p.product_id
          ) t
        group by
          t.season,
          t.category
        order by
          t.season,
          total_quantity desc,
          total_revenue desc
      ) tt
  ) ttt
where
  ttt.row_num = 1
