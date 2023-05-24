## replace-employee-id-with-the-unique-identifier

## REFERENCE

- https://leetcode.com/problems/replace-employee-id-with-the-unique-identifier/

## SOLUTION

``` MySQL
# Write your MySQL query statement below
SELECT unique_id, name FROM Employees LEFT JOIN EmployeeUNI ON Employees.id = EmployeeUNI.id
```


## TIMESTAMP

- 5/24/2023 00:04:29
- 5/25/2023 00:02:02
