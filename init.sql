create table if not exists person (
  id integer primary key,
  department text not null,
  name text not null unique,
  salary integer not null
);

insert into person values(1,'development','Taro',120);
insert into person values(2,'development','Hanako',200);
insert into person values(3,'development','Syuta',260);
insert into person values(4,'sales','Mike',160);
insert into person values(5,'sales','Takeshi',240);
insert into person values(6,'sales','Kyousuke',100);
insert into person values(7,'sales','Teru',220);
insert into person values(8,'management','Hide',400);
insert into person values(9,'management','Marry',200);
insert into person values(10,'management','Kim',300);
