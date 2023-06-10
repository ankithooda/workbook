SELECT cd.bookings.starttime
FROM cd.members JOIN cd.bookings
ON cd.members.memid = cd.bookings.memid
WHERE cd.members.firstname = 'David' AND cd.members.surname = 'Farrell'; 

-----------

SELECT cd.bookings.starttime, cd.facilities.name
FROM cd.bookings JOIN cd.facilities
ON cd.bookings.facid = cd.facilities.facid
WHERE cd.bookings.starttime >= '2012-09-21 00:00:00'
	AND cd.bookings.starttime <= '2012-09-21 23:59:59'
	AND cd.facilities.name like 'Tennis Court%'
ORDER by cd.bookings.starttime;

------------

SELECT distinct members_2.firstname as firstname, members_2.surname as surname
FROM cd.members members_1 INNER JOIN cd.members members_2
ON members_1.recommendedby = members_2.memid ORDER BY surname, firstname;

------------

SELECT mem.firstname as memfname, mem.surname memsname, 
       recs.firstname as recfname, recs.surname recsname

FROM cd.members mem LEFT JOIN cd.members recs ON mem.recommendedby = recs.memid
ORDER BY memsname, memfname;

------------


SELECT DISTINCT concat(cd.members.firstname, ' ',cd.members.surname) AS member_name,
	   cd.facilities.name 
	   FROM cd.members INNER JOIN cd.bookings on cd.members.memid = cd.bookings.memid 
	   INNER JOIN cd.facilities ON cd.bookings.facid = cd.facilities.facid 
	   WHERE cd.facilities.name LIKE 'Tennis Court%' 
	   ORDER BY member_name, cd.facilities.name;

------------

SELECT concat(cd.members.firstname, ' ', cd.members.surname), 
       cd.facilities.name,
	   (CASE WHEN cd.members.memid = 0 THEN cd.facilities.guestcost * cd.bookings.slots ELSE cd.facilities.membercost * cd.bookings.slots END) AS total_cost 
	   FROM cd.bookings INNER JOIN cd.members 
	   	ON cd.bookings.memid = cd.members.memid 
	INNER JOIN cd.facilities on cd.bookings.facid = cd.facilities.facid where DATE(cd.bookings.starttime) = '2012-09-14' AND ((cd.members.memid = 0 AND cd.facilities.guestcost * cd.bookings.slots > 30) OR (cd.members.memid != 0 AND cd.facilities.membercost * cd.bookings.slots > 30));

------------

SELECT DISTINCT concat(mems.firstname, ' ', mems.surname),
                (SELECT concat (recs.firstname, ' ', recs.surname) FROM cd.members recs 
				WHERE recs.memid = mems.recommendedby) FROM cd.members mems;


