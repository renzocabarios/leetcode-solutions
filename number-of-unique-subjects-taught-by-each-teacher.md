## number-of-unique-subjects-taught-by-each-teacher

## REFERENCE

- https://leetcode.com/problems/number-of-unique-subjects-taught-by-each-teacher/

## SOLUTION

``` MySql
# Write your MySQL query statement below

SELECT teacher_id, COUNT(DISTINCT subject_id) AS cnt
FROM Teacher
GROUP BY teacher_id
ORDER BY NULL;
```

## TIMESTAMP

- 6/03/2023 00:03:09
- 6/04/2023 00:01:59
- 6/05/2023 00:02:26
