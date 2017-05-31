FROM frolvlad/alpine-glibc
MAINTAINER Bastian Köcher <codeclimate@kchr.de>

RUN     mkdir -p /app/

ADD     ./bin/codeclimate-rustfmt /app/
ADD     ./bin/rustfmt /app/

ADD     ./main.sh /app/

RUN     adduser -u 9000 -D -h /app app
RUN     chown -R app:app /app

USER    app

VOLUME  /code
WORKDIR /code

CMD     ["/app/main.sh"]