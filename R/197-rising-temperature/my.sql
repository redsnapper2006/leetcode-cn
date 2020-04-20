SELECT
  C.Id
FROM
  Weather C,
  Weather P
WHERE
  DATE_SUB(C.RecordDate, INTERVAL 1 DAY) = P.RecordDate
  AND C.Temperature > P.Temperature
