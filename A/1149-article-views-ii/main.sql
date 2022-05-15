With vv as (
  select
    distinct article_id,
    author_id,
    viewer_id,
    view_date
  from
    Views
  group by
    article_id,
    author_id,
    viewer_id,
    view_date
)
select
  distinct v.viewer_id as id
from
  (
    select
      viewer_id,
      count(*) as cnt
    from
      vv
    group by
      view_date,
      viewer_id
    having
      cnt >= 2
  ) v
order by
  viewer_id
