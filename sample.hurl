POST http://localhost:8080/api/v1/users
{
        "username": "username",
        "password": "password"
}

# Add new channel
POST http://localhost:8080/api/v1/channels
Authorization: Token eyJhbGciOiJIUzM4NCJ9.eyJpZCI6OSwiZXhwIjoxNjcxNTY1Nzc2fQ.jzzior7WdeMNl-UW13zUYQLLkVnRGkgfNqkn0UwH1VHGhnNJZiZ3JYvbp1GEuwgC
Content-Type: application/json
{
    "url": "https://www.youtube.com/@linuxytapas2840/videos",
    "path": "linux_y_tapas",
    "title": "Linux y Tapas",
    "description": "Linux y Tapas",
    "last": "1970-01-01T10:20:00Z"
}

# GENERAL
GET http://localhost:8080/api/v1/channels

HTTP/* 200
[Asserts]
jsonpath "$" count > 0

