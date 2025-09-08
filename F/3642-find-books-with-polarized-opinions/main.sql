select
  b.book_id,
  b.title,
  b.author,
  b.genre,
  b.pages,
  (rs4.mx - rs2.mn) as rating_spread,
  round(rcp.read_cnt / rc.read_cnt, 2) as polarization_score
from
  books b,
  (
    select
      book_id,
      count(*) as read_cnt
    from
      reading_sessions
    where
      session_rating <= 2
      or session_rating >= 4
    group by
      book_id
  ) rcp,
  (
    select
      book_id,
      count(*) as read_cnt
    from
      reading_sessions
    group by
      book_id
  ) rc,
  (
    select
      book_id,
      max(session_rating) as mx
    from
      reading_sessions
    group by
      book_id
  ) rs4,
  (
    select
      book_id,
      min(session_rating) as mn
    from
      reading_sessions
    group by
      book_id
  ) rs2
where
  rc.book_id = rs4.book_id
  and rs4.book_id = rs2.book_id
  and rs2.book_id = rcp.book_id
  and rcp.book_id = b.book_id
  and rs4.mx >= 4
  and rs2.mn <= 2
  and rc.read_cnt >= 5
  and rcp.read_cnt / rc.read_cnt >= 0.6
order by
  polarization_score desc,
  b.title desc
