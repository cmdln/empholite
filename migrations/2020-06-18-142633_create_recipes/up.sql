create table recipes (
        id uuid primary key default uuid_generate_v4(),
        url varchar not null,
        payload text not null,
        created_at timestamp not null default now(),
        updated_at timestamp not null default now()
);

select diesel_manage_updated_at('recipes');

insert into recipes (url, payload) values ('http://test.local/api/foo', '{"foo":"bar"}');
