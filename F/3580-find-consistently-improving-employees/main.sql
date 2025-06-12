with
  prrn as (
    select
      employee_id,
      review_date,
      rating,
      ROW_NUMBER() OVER (
        PARTITION BY
          employee_id
        order by
          review_date desc
      ) as rn
    from
      performance_reviews
  )
select
  e.employee_id,
  e.name,
  pr1.rating - pr3.rating as improvement_score
from
  performance_reviews pr,
  employees e,
  prrn pr1,
  prrn pr2,
  prrn pr3
where
  pr.employee_id = e.employee_id
  and pr.review_date = pr1.review_date
  and pr.employee_id = pr1.employee_id
  and pr.employee_id = pr2.employee_id
  and pr.employee_id = pr3.employee_id
  and pr1.rn = 1
  and pr2.rn = 2
  and pr3.rn = 3
  and pr1.rating > pr2.rating
  and pr2.rating > pr3.rating
order by
  improvement_score desc,
  name asc
