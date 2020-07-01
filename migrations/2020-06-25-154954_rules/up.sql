create type rule_type as enum ('authenticated', 'subject');

create table rules (
        id uuid primary key default uuid_generate_v4(),
        recipe_id uuid not null references recipes,
        rule_type rule_type not null,
        key_path varchar null,
        subject varchar null
);

insert into rules (recipe_id, rule_type, key_path) select r.id, 'authenticated', '/foo' from recipes r where r.url  = 'http://test.local/api/foo';
insert into rules (recipe_id, rule_type, subject) select r.id, 'subject', 'test' from recipes r where r.url  = 'http://test.local/api/foo';
