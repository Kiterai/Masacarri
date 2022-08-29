# Masacarri

Masacarri is lightwight comment system. It can be embedded to your pages.

## start with docker

1. Install docker and docker-compose.
2. Prepare `docker-compose.yml`: https://github.com/FoolsLab/Masacarri/blob/master/docker-compose.yml
3. set the environment variables.
    - `DATABASE_URL`: a valid url to postgreSQL DB
    - `SESSION_DATABASE_URL`: a valid url to redis DB
    - `HOST`: server host intended to use
    - `PORT`: server port intended to use
    - `SITE_NAME`: the name of your site. (This string will be used for notifing mail.)
    - `MASACARRI_USER`: initial admin username
    - `MASACARRI_PASSWORD`: initial admin password
    - `SMTP_HOST`: smtp server host(for notifing mail)
    - `SMTP_PORT`: smtp server port(for notifing mail)
    - `SMTP_USER`: smtp account name(for notifing mail)
    - `SMTP_PASSWORD`: smtp account password(for notifing mail)
    - `SMTP_ENCRYPTION`: smtp encryption mode, `tls`/`starttls`/`plain` are available
    - `SMTP_MAILADDR`: mail addr(for notifing mail)
4. Execute `docker-compose up -d`.
