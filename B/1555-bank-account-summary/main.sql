SELECT
  u.user_id,
  u.user_name,
  (
    u.credit - ifnull(
      (
        SELECT
          sum(amount)
        from
          Transactions t
        where
          t.paid_by = u.user_id
      ),
      0
    ) + ifnull(
      (
        SELECT
          sum(amount)
        from
          Transactions t
        where
          t.paid_to = u.user_id
      ),
      0
    )
  ) as credit,
  if(
    (
      u.credit - ifnull(
        (
          SELECT
            sum(amount)
          from
            Transactions t
          where
            t.paid_by = u.user_id
        ),
        0
      ) + ifnull(
        (
          SELECT
            sum(amount)
          from
            Transactions t
          where
            t.paid_to = u.user_id
        ),
        0
      )
    ) >= 0,
    'No',
    'Yes'
  ) as credit_limit_breached
FROM
  Users u
