<div align="center">
<h1 align="center">ytpodcast</h1>
<br />
<img alt="License: MIT" src="https://img.shields.io/badge/License-MIT-blue.svg" /><br>
<br>
A service to create your podcasts from your favourites YouTube Channels
</div>

---

### Installation

With docker-compose

```
mkdir audios db
```

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

### Contributing

### License

This project is licensed under the MIT license

### Show your support

Leave a ⭐ if you like this project

***
Readme made with 💖 using [README Generator by Dhravya Shah](https://github.com/Dhravya/readme-generator)
