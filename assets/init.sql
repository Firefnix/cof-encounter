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

create table artifact (
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
    def integer,
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

create table npc_tags (
    npc_id integer not null,
    tag_id integer not null,
    primary key (npc_id, tag_id),
    foreign key (npc_id) references npc(id) on delete cascade on update cascade,
    foreign key (tag_id) references tag(id) on delete cascade on update cascade
);

create table artifact_tags (
    artifact_id integer not null,
    tag_id integer not null,
    primary key (artifact_id, tag_id),
    foreign key (artifact_id) references artifact(id) on delete cascade on update cascade,
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

create table place_artifacts (
    place_id integer not null,
    artifact_id integer not null,
    primary key (place_id, artifact_id),
    foreign key (place_id) references place(id) on delete cascade on update cascade,
    foreign key (artifact_id) references artifact(id) on delete cascade on update cascade
);
