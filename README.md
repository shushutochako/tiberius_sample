# tiberius_exmaple

## Usage

### Run Container

```
docker-compose up -d
```

### Add Sample Data

```
docker-compose exec mssql_cil_client /bin/sh
```

```
npm install -g sql-cli
```

```
mssql -s db -u sa -p P@ssw0rd!
```

```
create database test;
use test;
```

```
create table users \
( \
    id int not null, \
    name nvarchar(128), \
    constraint pk_users primary key (id) \
);
```

```
insert into users(id,name) values(1,'Taro');
insert into users(id,name) values(2,'Jiro');
insert into users(id,name) values(3,'Hanako');
```

```
.quit
exit
```


### Run Application

```
docker-compose exec app cargo run
```