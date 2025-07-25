
<h1 align="center">
  <br>
  suricata-notify
  <br>
</h1>

<h4 align="center">✨ A tool to send notifications from Suricata to anywhere </h4>

> [!WARNING]
> This software is still in development and probably is not ready for production use & may contain bugs. It may be not working at the moment. I built it for my own use and decided to share it with the world.

## Current Features
- Watch Suricata `eve.json` file for new alerts and parse them
- Send alerts to HTTP endpoints (basic functionality)

## Installation
Clone the repository:
```bash
git clone https://github.com/w1ltshire/suricata-notify.git
```

Copy the `config.example.toml` to `config.toml` and edit it to your needs. Template section should be self-explanatory.

```bash
cp config.example.toml config.toml
```

Edit `docker-compose.yml` and set the path to your `eve.json` file and then run the container:
```bash
docker compose up -d
```
Everything should be up and running now.

## Roadmap
- [ ] Add support for multiple backends of the same type
- [x] Add configuration system
- [ ] Tide up the code

## License

MIT

---



