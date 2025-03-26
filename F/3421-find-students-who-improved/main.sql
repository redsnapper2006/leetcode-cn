select
  mx.student_id,
  mx.subject,
  mn.score as first_score,
  mx.score as latest_score
from
  (
    select
      s.student_id,
      s.subject,
      s.score,
      s.exam_date
    from
      Scores s,
      (
        select
          student_id,
          subject,
          max(exam_date) as max_exam_date
        from
          Scores
        group by
          student_id,
          subject
      ) r
    where
      s.student_id = r.student_id
      and s.subject = r.subject
      and s.exam_date = r.max_exam_date
  ) mx,
  (
    select
      s.student_id,
      s.subject,
      s.score,
      s.exam_date
    from
      Scores s,
      (
        select
          student_id,
          subject,
          min(exam_date) as min_exam_date
        from
          Scores
        group by
          student_id,
          subject
      ) r
    where
      s.student_id = r.student_id
      and s.subject = r.subject
      and s.exam_date = r.min_exam_date
  ) mn
where
  mx.student_id = mn.student_id
  and mx.subject = mn.subject
  and mx.exam_date > mn.exam_date
  and mx.score > mn.score
order by
  mx.student_id,
  mx.subject
