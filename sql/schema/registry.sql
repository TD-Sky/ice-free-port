create table registry
(
    id int8 primary key,                -- 唯一索引
    username text not NULL unique,      -- 用户名
    pswd text not NULL                  -- 密码
);
