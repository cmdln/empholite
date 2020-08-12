alter type rule_type add value if not exists 'http_method';

create type http_verb as enum ('get', 'post', 'put', 'delete');

alter table rules add column http_method http_verb null;
