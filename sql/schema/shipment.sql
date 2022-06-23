-- freight_forwarder 1 -> shipping_order M -> shipping_item N

create table freight_forwarder
(
    id int8 primary key,                        -- 雪花ID
    company_name text not NULL unique,          -- 公司名
    telephone_number varchar(11) not NULL       -- 联系方式
);

create table shipping_order
(
    num int8 primary key,                       -- 订单号，雪花ID
    company_id int8 not NULL,                   -- 货代公司ID
    --------------------------------------------
    constraint company_id_fk foreign key (company_id)
        references freight_forwarder(id)
);

create table shipping_item
(
    order_num int8 not NULL,                    -- 订单号
    nth int2 not NULL,                          -- 条目序号
    shipment_date date not NULL,                -- 发货日期
    quantity int4 not NULL,                     -- 件数
    ton float8 not NULL,                        -- 吨数
    --------------------------------------------
    primary key(order_num, nth),                -- 订单与序号共同确定
    constraint order_num_fk foreign key (order_num)
        references shipping_order(num)
);
