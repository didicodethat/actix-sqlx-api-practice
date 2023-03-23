# Actix and SQLX practice
This is a practice application to learn a bit of rust.
It makes use of sqlx-cli to manage migrations, so if you want to install
the database you can use it's commands to apply them.

The only route made is the `POST /user` route, where you can register a user by sending a POST request to it with a json object containing a `phone_number` key.

## Conclusion:
This was a nice experiment, but I think Actix is a bit too difficult for me to use on a regular basis, had a lot of difficult trying to figure out how to send a different status code on error and send a json body along with it, ended up sending back a 200 with a json containing a error message.