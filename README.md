happy-lunch-mates
=================
#### _Slack App for organizing lunch groups_

The lunch crew at Sanctuary has gotten large enough to warrant lunch groups.
This is the repo for a Slack App that makes lunch groups and rotates members periodically,
also allowing group members to vote on where to eat.

Requirements
------------

* [`rust`](https://www.rust-lang.org/)

Installation and Setup
----------------------

You'll need an API Auth token to make requests via Slack-API.
Get the token from a present contributor and add the token to a `.env` file.
```text
AUTH_TOKEN=<your-token-here>
```

To compile, run
```
$ cargo build
```

Running
-------

Run
```
$ cargo run
```

