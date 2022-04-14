SELECT
    account_id,
    day,
    SUM(
        CASE
            WHEN type = 'Deposit' THEN amount
            WHEN type = 'Withdraw' THEN - amount
        END
    ) OVER(
        PARTITION BY account_id
        ORDER BY
            day
    ) balance
FROM
    Transactions
ORDER BY
    account_id,
    day
