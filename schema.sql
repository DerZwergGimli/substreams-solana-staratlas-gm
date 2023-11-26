create table sa_trades
(
    id        text not null
        constraint sa_trades_pk primary key,
    block     integer,
    timestamp timestamp,
    signature text,
    maker     text,
    taker     text,
    seller     text,
    currency  text,
    asset     text,
    price     float,
    size      integer,
    volume    float,
    side      text,
    fee       float
);

create table sa_order_book
(
    id             text not null
        constraint sa_order_book_pk primary key,
    order_accounts text,
    currency       text,
    asset          text,
    price          float,
    size           integer,
    volume         float,
    side           text

);

create table cursors
(
    id        text not null
        constraint cursor_pk primary key,
    cursor    text,
    block_num bigint,
    block_id  text
);