
SELECT endact.machine_id, ROUND(AVG(endact.timestamp - startact.timestamp), 3) AS processing_time
FROM 
(
  SELECT machine_id, process_id, timestamp
  FROM activity
  WHERE activity_type = 'end'
) endact
INNER JOIN 
(
  SELECT machine_id, process_id, timestamp
  FROM activity
  WHERE activity_type = 'start'
) startact
ON endact.machine_id = startact.machine_id
AND endact.process_id = startact.process_id
GROUP BY endact.machine_id;

