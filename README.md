Task manager api written in Rust!

To run you need to create PostgreSQL docker container

<pre>$ docker run -e POSTGRES_PASSWORD={pass} -e POSTGRES_USER={user} -e POSTGRES_DB={db_name} -p {port}:5432 postgres</pre>

then create .env file inside of root folder where you need to specify google client id, secret and db connection string
<pre>GOOGLE_CLIENT_ID="*"
GOOGLE_CLIENT_SECRET="*"
DB_CONNECTION="postgres://{user}:{pass}@localhost:{port}/{db_name}"</pre>

then start web app using cargo

<pre>$ cargo run</pre>
