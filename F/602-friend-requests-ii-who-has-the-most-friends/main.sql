SELECT an.id, count(*) num
FROM
  (
    (SELECT ra1.requester_id id FROM RequestAccepted ra1)
    union ALL
    (SELECT ra1.accepter_id id FROM RequestAccepted ra1)
  ) an
group by an.id
order by num desc
limit 1
