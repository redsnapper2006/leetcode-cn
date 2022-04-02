SELECT tt.customer_number FROM (
SELECT customer_number, COUNT(*) as cnt FROM Orders
GROUP BY customer_number
ORDER BY cnt DESC
) tt 
WHERE tt.cnt = (
SELECT MAX(t.cnt) as MAX_CNT FROM (
SELECT COUNT(*) as cnt FROM Orders
GROUP BY customer_number
ORDER BY cnt DESC
) t)
