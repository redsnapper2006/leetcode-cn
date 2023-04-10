SELECT u.t AS category, u.accounts_count
FROM
(
  SELECT 'Low Salary' AS t, (
    SELECT COUNT(*) AS cnt
    FROM Accounts
    WHERE income < 20000
  ) AS accounts_count
  UNION
  SELECT 'Average Salary' AS t, (
    SELECT COUNT(*) AS cnt
    FROM Accounts
    WHERE income >= 20000
    AND income <= 50000
  ) AS accounts_count
  UNION
  SELECT 'High Salary' AS t, (
    SELECT COUNT(*) AS cnt
    FROM Accounts
    WHERE income > 50000
  ) AS accounts_count
) u;
