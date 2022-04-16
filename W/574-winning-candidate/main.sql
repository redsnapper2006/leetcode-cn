SELECT
  name
FROM
  Candidate c,
  (
    SELECT
      candidateId,
      count(*) as cnt
    FROM
      Vote
    GROUP BY
      candidateId
    ORDER BY
      cnt DESC
    LIMIT
      1
  ) pp
WHERE
  c.id = pp.candidateId
