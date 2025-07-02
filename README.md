# rustidemy-bot

`rustidemy-bot` is a Telegram bot that fetches free Udemy course listings from an RSS feed and posts them to a specified Telegram channel.

## Configuration

Set the following environment variables:

- `BOT_TOKEN` (required): Your Telegram bot token.
- `CHANNEL_ID` (required): ID of the channel where messages will be sent.

## Deploy

<div align="left">
  <a href="https://render.com/deploy">
    <img src="images/render.svg" alt="Deploy to Render">
  </a>
  <br>
  <a href="https://heroku.com/deploy?template=https://github.com/5hojib/rustidemy-bot">
    <img src="images/heroku.svg" alt="Deploy to Heroku">
  </a>
</div><br>

> [!Note]
> If you're using Heroku eco dynos or Render free plan, use a cron job to keep the bot alive:
https://cron-job.org