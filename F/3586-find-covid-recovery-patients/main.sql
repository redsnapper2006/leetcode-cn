select
  ct.patient_id,
  p.patient_name,
  p.age,
  DATEDIFF (
    min(ct.test_date),
    positive_covid_tests.min_test_date
  ) as recovery_time
from
  covid_tests ct,
  patients p,
  (
    select
      patient_id,
      min(test_date) as min_test_date
    from
      covid_tests
    where
      result = 'Positive'
    group by
      patient_id
  ) positive_covid_tests
where
  ct.patient_id = p.patient_id
  and ct.patient_id = positive_covid_tests.patient_id
  and ct.result = 'Negative'
  and ct.test_date > positive_covid_tests.min_test_date
group by
  ct.patient_id
order by
  recovery_time,
  p.patient_name
