-- Active: 1676900194062@@127.0.0.1@5433@tutorfront_web_app@public

drop table if exists user_table;

create table
    user_table(
        username varchar(20) primary key,
        tutor_id int,
        user_password char(100) not null
    );
