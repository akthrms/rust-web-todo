create table labels (
    id serial primary key,
    name text not null
);

create table todo_labels (
    id serial primary key,
    todo_id integer not null preferences todos (id) deferrable initially deferred,
    label_id integer not null preferences todos (id) deferrable initially deferred,
);
