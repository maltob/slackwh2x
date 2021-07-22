## SlackWH2X

Service to take in Slack incoming webhooks and forward/convert them to other other systems for notification. Such as taking ina Slack webhook then sending it to both Slack and Microsoft Teams incoming webhooks with conversion of emojis and links to Teams format.

#### Services Planned
 * Slack
 * Teams
 * SMS/MMS

 #### How to use
  1. Copy docker-compose.yml down to a docker host
  1. Copy slack2x.example.yaml and save it as slack2x.yaml in the same directory as docker-compose.yaml
  1. Edit the slack2x.yaml with the URLs and config needed. The keys are suffixes so for example the below file has two URL webhooks that can be used seperately
```
Hook1:
- https://slack.com/webhook/URL
- https://teams.outlook.com/webhook/URL
Hook2:
- https://slack.com/webhook/URL3
```
  1. Run docker-compose up -d
  1. Enter in the URL to your server with the webhook in the Slack webhook URL of the application, so for example https://wh.example.url/Hook1