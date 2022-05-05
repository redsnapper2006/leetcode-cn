select
  distinct title
from
  Content c,
  TVProgram tp
where
  c.content_id = tp.content_id
  and DATE_FORMAT(tp.program_date, "%Y-%m") = "2020-06"
  and c.content_type = 'Movies'
  and c.Kids_content = 'Y'
