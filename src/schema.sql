drop table HealthTrackers;
create table HealthTrackers(
	tid uniqueidentifier primary key default newid(),
	appname varchar(32),
	apiname varchar(32),
	url varchar(2000) not null,
	headers varchar(8000) not null default '{}',
	interval int not null default 120,
	timeout int not null default 5,
	validation varchar(5) not null default 'None',
	criteria varchar(8000),
	condition varchar(2000),
	active bit default 1
);

drop table HealthHistory;
create table HealthHistory(
	tid uniqueidentifier not null references HealthTrackers(tid),
	checktime datetime2 not null default current_timestamp,
	duration int,
	health bit not null,
	code smallint,
	logs varchar(8000) 
);

insert into HealthTrackers(appname,apiname,url) values('HttpBin','IP','https://httpbin.org/ip');
insert into HealthTrackers(appname,apiname,url) values('HttpBin','Get','https://httpbin.org/get');

select * from HealthTrackers;
select * from HealthHistory;