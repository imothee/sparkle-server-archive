# Sparkle Server

Sparkle-server is your one stop shop for AppCast hosting and privacy first sparkle-profile metrics.

## Features

- API and UI access to manage your Appcast
- Profile opted-in users in a privacy first way - user specific profiles are stored for less than an hour
- Hourly profile metrics
- An aggregated landing page for your personal or company apps

## Getting started

Install or deploy the phoenix app to any hosting provider.

### Optional: Deploy to Heroku

[![Deploy](https://www.herokucdn.com/deploy/button.svg)](https://heroku.com/deploy?template=https://github.com/imothee/sparkle-server)

### Setup an Admin account

Once you've installed the elixir app and gotten it running, you'll need to create an Admin user using the cli task, it will echo out a password to use for the account. You can create multiple users.

```
mix user.create your@email.com
```

### Login to the admin account and create your apps

Go to $your_url/admin
