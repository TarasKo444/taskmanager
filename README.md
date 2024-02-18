Task manager api written with Rust!

To run you need to create docker container

<pre>$ docker run -e POSTGRES_PASSWORD=root -e POSTGRES_USER=postgres -e POSTGRES_DB=taskmanagerstore  -p 5342:5432 postgres</pre>
and start rust binary with
<pre>$ cargo run</pre>
