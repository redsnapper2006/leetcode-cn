SELECT
  u.name,
  sum(t.amount) as balance
FROM
  Users u,
  Transactions t
WHERE
  u.account = t.account
GROUP BY
  u.account
HAVING
  balance > 10000
