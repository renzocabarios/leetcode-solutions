## find-total-time-spent-by-each-employee

## REFERENCE

- https://leetcode.com/problems/find-total-time-spent-by-each-employee/

## SOLUTION

``` mysql
# Write your MySQL query statement below

SELECT event_day as day, emp_id, SUM(out_time - in_time) as total_time FROM Employees GROUP BY day, emp_id;
```


## TIMESTAMP

- 5/24/2023 00:06:54