SELECT
  c.candidate_id
FROM
  Candidates c,
  (
    SELECT
      c1.candidate_id,
      sum(r.score) as sumscore
    FROM
      Candidates c1,
      Rounds r
    WHERE
      c1.years_of_exp > 1
      AND r.interview_id = c1.interview_id
    GROUP BY
      c1.candidate_id
  ) ss
WHERE
  c.years_of_exp > 1
  AND c.candidate_id = ss.candidate_id
  AND ss.sumscore > 15
