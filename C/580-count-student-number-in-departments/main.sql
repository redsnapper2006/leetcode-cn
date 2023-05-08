select t.dept_name, 
  (select count(*) 
   from Student s 
   where s.dept_id = t.dept_id) student_number
from Department t
order by student_number desc, t.dept_name

