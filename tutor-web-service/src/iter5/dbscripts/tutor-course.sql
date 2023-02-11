-- Active: 1674222411971@@127.0.0.1@5432@tutorfront@public
drop table if exists tutor_course cascade;

drop table if exists tutor;

create table
    tutor (
        tutor_id serial primary key,
        tutor_name varchar(200) not null,
        tutor_pic_url varchar(200) not null,
        tutor_profile varchar(2000) not null
    );

create table
    tutor_course (
        course_id serial primary key,
        tutor_id int not null,
        course_name varchar(140) not null,
        course_description varchar(2000),
        course_format varchar(30),
        course_structure varchar(200),
        course_duration varchar(30),
        course_price INT,
        course_language varchar(30),
        course_level varchar(30),
        posted_time TIMESTAMP default now(),
        constraint fk_tutor foreign key (tutor_id) references tutor(tutor_id) on delete cascade
    );

grant all privileges on table tutor to "user";

grant all privileges on table tutor_course to "user";

grant all privileges on all sequences in schema public to "user";

/* Load seed data for testing */

insert into
    tutor(
        tutor_id,
        tutor_name,
        tutor_pic_url,
        tutor_profile
    )
values (
        1,
        'Merlene',
        'http://s3.amazon.aws.com/pic1',
        'Merlene is an experienced finance professional'
    );

insert into
    tutor(
        tutor_id,
        tutor_name,
        tutor_pic_url,
        tutor_profile
    )
values (
        2,
        'Frank',
        'http://s3.amazon.aws.com/pic2',
        'Frank is an expert nuclear engineer'
    );

insert into
    tutor_course (
        course_id,
        tutor_id,
        course_name,
        course_level,
        posted_time
    )
values (
        1,
        1,
        'First course',
        'Beginner',
        '2021-04-12 05:40:00'
    );

insert into
    tutor_course (
        course_id,
        tutor_id,
        course_name,
        course_format,
        posted_time
    )
values (
        2,
        1,
        'Second course',
        'ebook',
        '2021-04-12 05:45:00'
    );
