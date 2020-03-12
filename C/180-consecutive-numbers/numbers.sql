SELECT
  DISTINCT l1.Num AS ConsecutiveNums
FROM
  LOGS l1,
  LOGS l2,
  LOGS l3
WHERE
  l1.Num = l2.Num
  AND l2.Num = l3.Num
  AND l1.Id + 1 = l2.Id
  AND l2.Id + 1 = l3.Id
