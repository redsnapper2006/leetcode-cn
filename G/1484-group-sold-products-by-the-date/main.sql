SELECT
	t.sell_date,
	count(*) as num_sold,
	GROUP_CONCAT(
		DISTINCT t.product
		ORDER BY
			t.product ASC SEPARATOR ','
	) as products
FROM
	(
		SELECT
			DISTINCT sell_date,
			product
		FROM
			Activities
		ORDER BY
			sell_date,
			product
	) t
GROUP BY
	t.sell_date
