<div align="center">
  <h1>Slacordbot</h1>
  Slackbot Auto Response, but for Discord.
  <br>
  <br>
  <img width=192px src="./assets/logo.png">
</div>


## Why

Although many popular Discord bots have a trigger feature, they typically do not support the functionality of providing a random response from a list of multiple choices, which is a feature that Slackbot offers.

This bot is for those who want to migrate the Slackbot auto response to Discord as-is.


## How to run

```
$ DISCORD_TOKEN=... cargo run
```

 - This is a self-hosted bot
 - Enable the Message Content intent
 - Allow the bot to read/send messages
 - Define keyword:response pairs in config.json
 - Use the icon in the assets directory if you like :blush:


## Appendix

`convert_slackbot.py` converts the response from `https://{workspace}.slack.com/api/slackbot.responses.list` into the config.json.
