create table illustration (
    id integer primary key autoincrement,
    width integer,
    height integer,
    path text not null
);

create table tag (
    id integer primary key autoincrement,
    name text not null,
    desc text
);

create table object (
    id integer primary key autoincrement,
    name text not null,
    desc text,
    price integer,
    illustration integer,
    foreign key (illustration) references illustration(id)
);

create table npc (
    id integer primary key autoincrement,
    name text not null,
    force integer,
    dex integer,
    con integer,
    int integer,
    sag integer,
    cha integer,
    pv integer,
    init integer,
    illustration integer,
    foreign key (illustration) references illustration(id)
);

create table place (
    id integer primary key autoincrement,
    name text not null,
    desc text,
    illustration integer,
    foreign key (illustration) references illustration(id)
);

create table illustration_tags (
    illustration_id integer not null,
    tag_id integer not null,
    primary key (illustration_id, tag_id),
    foreign key (illustration_id) references illustration(id) on delete cascade on update cascade,
    foreign key (tag_id) references tag(id) on delete cascade on update cascade
);

create table object_tags (
    object_id integer not null,
    tag_id integer not null,
    primary key (object_id, tag_id),
    foreign key (object_id) references object(id) on delete cascade on update cascade,
    foreign key (tag_id) references tag(id) on delete cascade on update cascade
);

create table place_tags (
    place_id integer not null,
    tag_id integer not null,
    primary key (place_id, tag_id),
    foreign key (place_id) references place(id) on delete cascade on update cascade,
    foreign key (tag_id) references tag(id) on delete cascade on update cascade
);

create table place_npcs (
    place_id integer not null,
    npc_id integer not null,
    primary key (place_id, npc_id),
    foreign key (place_id) references place(id) on delete cascade on update cascade,
    foreign key (npc_id) references npc(id) on delete cascade on update cascade
);

create table place_objects (
    place_id integer not null,
    object_id integer not null,
    primary key (place_id, object_id),
    foreign key (place_id) references place(id) on delete cascade on update cascade,
    foreign key (object_id) references object(id) on delete cascade on update cascade
);
