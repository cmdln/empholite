FROM rust

RUN cargo install --no-default-features diesel_cli --features postgres
RUN apt update && apt install -y postgresql-client

WORKDIR /opt/code

ADD migrations/00000000000000_diesel_initial_setup/ /opt/code/migrations/00000000000000_diesel_initial_setup/
ADD migrations/2020-06-18-142633_create_recipes/ /opt/code/migrations/2020-06-18-142633_create_recipes
ADD migrations/2020-06-25-154954_rules /opt/code/migrations/2020-06-25-154954_rules
ADD migrations/2020-08-12-203728_add_http_verb_rule /opt/code/migrations/2020-08-12-203728_add_http_verb_rule
ADD database/entry.sh /opt/code/

CMD [ "./entry.sh" ]
