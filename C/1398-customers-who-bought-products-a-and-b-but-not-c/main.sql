
Select customer_id, customer_name
FROM
Customers c,
(SELECT distinct customer_id as id
FROM Orders
where product_name = 'A'
) A,
(SELECT distinct customer_id as id
FROM Orders
where product_name = 'B'
) B
WHERE A.id = B.id
AND A.id NOT IN (
SELECT customer_id as id
FROM Orders
where product_name = 'C'
)
AND c.customer_id = A.id
ORDER BY A.id
