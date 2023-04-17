WITH vv AS (
  SELECT 
    d.customer_id, 
    d.order_date, 
    d.customer_pref_delivery_date
  FROM 
    Delivery d 
    INNER JOIN (
      SELECT customer_id, MIN(order_date) min_order_date
      FROM Delivery 
      GROUP BY customer_id 
    ) m ON d.order_date = m.min_order_date and d.customer_id = m.customer_id
)
SELECT 
  ROUND(
    (
      SELECT COUNT(*) 
      FROM vv 
      WHERE order_date = customer_pref_delivery_date 
    ) / 
    (
      SELECT COUNT(*) 
      FROM vv
    ) * 100, 
    2
  ) immediate_percentage;

