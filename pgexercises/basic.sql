SELECT * FROM cd.facilities;

SELECT name, membercost FROM cd.facilities;

SELECT * FROM cd.facilities where membercost > 0;

SELECT facid, name, membercost, monthlymaintenance FROM cd.facilities
WHERE membercost > 0 AND membercost < monthlymaintenance / 50;

SELECT * from cd.facilities WHERE name like '%Tennis%';

SELECT * from cd.facilities WHERE facid = 1 or facid = 5;

SELECT name,
	   CASE WHEN monthlymaintenance > 100 THEN 'expensive'
	   ELSE 'cheap'
	   END as cost
	   FROM cd.facilities;

SELECT memid, surname, firstname, joindate FROM cd.members where joindate >= '2012-09-01 00:00:00';

SELECT DISTINCT(surname) from cd.members ORDER BY surname LIMIT 10;

SELECT surname FROM cd.members UNION SELECT name FROM cd.facilities;

SELECT MAX(joindate) FROM cd.members;

SELECT firstname, surname, joindate FROM cd.members WHERE joindate = (SELECT MAX(joindate) FROM cd.members);




