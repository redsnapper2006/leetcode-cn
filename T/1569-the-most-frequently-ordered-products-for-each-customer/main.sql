SELECT
  pp.customer_id,
  pp.product_id,
  p.product_name
FROM
  (
    SELECT
      customer_id,
      product_id,
      COUNT(*) as cnt
    from
      Orders
    group by
      customer_id,
      product_id
  ) pp,
  (
    select
      p.customer_id,
      max(cnt) as maxcnt
    from
      (
        SELECT
          customer_id,
          product_id,
          COUNT(*) as cnt
        from
          Orders
        group by
          customer_id,
          product_id
      ) p
    group by
      p.customer_id
  ) ppp,
  Products p
WHERE
  pp.customer_id = ppp.customer_id
  AND pp.cnt = ppp.maxcnt
  AND pp.product_id = p.product_id
