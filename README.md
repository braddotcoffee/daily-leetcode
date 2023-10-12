# Daily LeetCode Bot

Creates a new thread in a specified Discord Channel for the daily leetcode
challenge. **NOTE:** This bot does not handle being scheduled for once a day.
To accomplish this, I highly recommend [systemd timers](https://opensource.com/article/20/7/systemd-timers).

## config.yaml
```yaml
Discord:
  ChannelID: 0000000000000000000
```

## secrets.yaml
```yaml
Discord:
  Token: "Your Discord Auth Token"
```