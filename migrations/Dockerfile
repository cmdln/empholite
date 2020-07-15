FROM rust

RUN cargo install --no-default-features diesel_cli --features postgres
RUN apt update && apt install -y postgresql-client

WORKDIR /opt/code

ADD 00000000000000_diesel_initial_setup/ /opt/code/migrations/00000000000000_diesel_initial_setup/
ADD 2020-06-18-142633_create_recipes/ /opt/code/migrations/2020-06-18-142633_create_recipes
ADD 2020-06-25-154954_rules /opt/code/migrations/2020-06-25-154954_rules
ADD entry.sh /opt/code/

CMD [ "./entry.sh" ]
