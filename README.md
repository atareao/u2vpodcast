<div align="center">
<h1 align="center">u2vpodcast</h1>
<br />
<img alt="License: MIT" src="https://img.shields.io/badge/License-MIT-blue.svg" /><br>
<br>
A service to create your podcasts from your favourites YouTube Channels
</div>

---

### Installation

With docker-compose

* Change `docker-compose` and copy `sample.env` to `.env`. Change `.env` as you need.
* Create `cookies.txt` from your YouTube cookies.

### Create cookies.txt

#### How do you cookies to work

In order to extract cookies from browser use any conforming browser extension for exporting cookies.

For example,

* Chrome => https://chrome.google.com/webstore/detail/get-cookiestxt/bgaddhkoddajcdgocldbbfleckgcbcid/
* Firefox => https://addons.mozilla.org/en-US/firefox/addon/cookies-txt/

### Configuration

You need to modify `config.yml`. Change the params as you need, and add all the channels and YouTube list that you want

### Usage

```
docker-compose up -d
```
If you need to run [u2vpodcast](https://github.com/atareao/u2vpodcast) behind reverse proxy, like [caddy](https://github.com/caddyserver/caddy), run:

```
docker-compose -f docker-compose.yml -f docker-compose.caddy.yml up -d
```

After that, go to `https://u2vpodcast.tuservidor.com` and you can find a list of the channels. In this page, you can find every channel you added to the configuration file. For example, with **Linux y Tapas**, you  can find,

1. The channel: https://u2vpodcast.tuservidor.com/linux_y_tapas?page=1
2. The feed: https://u2vpodcast.tuservidor.com/linux_y_tapas/feed.xml

### Contributing

### License

This project is licensed under the MIT license

### Show your support

Leave a ⭐ if you like this project

***
Readme made with 💖 using [README Generator by Dhravya Shah](https://github.com/Dhravya/readme-generator)
