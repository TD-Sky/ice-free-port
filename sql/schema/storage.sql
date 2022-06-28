-- warehouse 1 -> storage N

create table warehouse
(
    id int8 primary key,                        -- 雪花ID
    house_name text not NULL unique,            -- 场地名称
    address text not NULL,                      -- 地址
    area int4 not NULL                          -- 面积(平方米)
);

create table storage
(
    id int8 primary key,                        -- 雪花ID
    warehouse_id int8 not NULL,                 -- 仓库ID
    store_date date not NULL,                   -- 入库日期
    license_plate_number char(7) not NULL,      -- 车牌号
    quantity int4 not NULL,                     -- 件数
    ton float8 not NULL,                        -- 吨数
    duration int default 0 not NULL,            -- 存放天数，每日0点更新
    --------------------------------------------
    constraint warehouse_id_fk foreign key (warehouse_id)
        references warehouse(id)
);
