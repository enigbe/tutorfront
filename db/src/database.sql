-- Active: 1674222411971@@127.0.0.1@5432@tutorfront@public

drop table if exists tutor_course;

create table
    tutor_course (
        course_id serial primary key,
        tutor_id int not null,
        course_name varchar(140) not null,
        posted_time timestamp default now()
    );

insert into
    tutor_course (
        course_id,
        tutor_id,
        course_name,
        posted_time
    )
values (
        1,
        1,
        'Advanced Calculus',
        '2023-01-17 05:40:00'
    );

insert into
    tutor_course (
        course_id,
        tutor_id,
        course_name,
        posted_time
    )
values (
        2,
        1,
        'Advanced Calculus II',
        '2023-01-18 06:40:00'
    );