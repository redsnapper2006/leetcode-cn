SELECT
    pp.product_id,
    pp.quantity,
    pp.quantity * p.price AS price
FROM (
    SELECT
        SUM(pur.quantity * p.price) AS cost,
        pur.invoice_id
    FROM
        Purchases pur
        INNER JOIN Products p
            ON pur.product_id = p.product_id
    GROUP BY
        pur.invoice_id
    ORDER BY
        cost DESC,
        pur.invoice_id
    LIMIT 1
) ppp , Purchases pp, Products p
WHERE
    ppp.invoice_id = pp.invoice_id
    AND pp.product_id = p.product_id;

