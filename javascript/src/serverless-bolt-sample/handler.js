'use strict';

// ------------------------------
// AWS Lambda handler
// ------------------------------
require('dotenv').config();
const { App, LogLevel, ExpressReceiver } = require('@slack/bolt');

const expressReceiver = new ExpressReceiver({
  signingSecret: process.env.SLACK_SIGNING_SECRET
});

const app = new App({
  token: process.env.SLACK_BOT_TOKEN,
  receiver: expressReceiver,
  logLevel: LogLevel.DEBUG
});

const awsServerlessExpress = require('aws-serverless-express');
const server = awsServerlessExpress.createServer(expressReceiver.app);

module.exports.app = (event, context) => {
  console.log('⚡️ Bolt app is running');
  console.log(event);
  awsServerlessExpress.proxy(server, event, context)
}

app.error(printCompleteJSON);

// ------------------------------
// Application Logic
// ------------------------------
app.command('/build', async ({ ack, body, context }) => {
  ack();
  const result = app.client.views.open({
    "token": context.botToken,
    "trigger_id": body.trigger_id,
    "view": {
      "type": "modal",
      "callback_id": "task-modal",
      "private_metadata": JSON.stringify(body),
      "title": {
        "type": "plain_text",
        "text": "Github Actions",
        "emoji": true
      },
      "submit": {
        "type": "plain_text",
        "text": "Submit",
        "emoji": true
      },
      "close": {
        "type": "plain_text",
        "text": "Cancel",
        "emoji": true
      },
      "blocks": [
        {
          "type": "input",
          "block_id": "input-title",
          "element": {
            "type": "plain_text_input",
            "action_id": "input",
            "initial_value": body.text
          },
          "label": {
            "type": "plain_text",
            "text": "branch",
            "emoji": true
          },
          "optional": false
        },
        {
          "type": "input",
          "block_id": "input-description",
          "element": {
            "type": "plain_text_input",
            "action_id": "input",
            "multiline": true
          },
          "label": {
            "type": "plain_text",
            "text": "echo string",
            "emoji": true
          },
          "optional": true
        }
      ]
    }
  });

  console.log( await result);
});

app.view('task-modal', async ({ body, ack, context }) => {
  // モーダルビューでのデータ送信イベントを確認
  ack();

  console.log(body.view.state.values['input-title'].input.value);
  console.log(body.view.state.values['input-description'].input.value);

  // ユーザーにメッセージを送信
  try {
    app.client.chat.postMessage({
      token: context.botToken,
      channel: body['user']['id'],
      text: 'Build request received!'
    });
  }
  catch (error) {
    console.error(error);
  }


  // ------------------------------
  // axios flow
  // ------------------------------
  const axiosBase = require('axios');
  const axios = axiosBase.create({
    baseURL: 'https://api.github.com',
    headers: {
      "Authorization": `token ${process.env.GITHUB_TOKEN}`,
      "Accept": "application/vnd.github.everest-preview+json"
    }
  });

  const requestBody = JSON.stringify({
    event_type: "hoge",
    client_payload: {
      ref: body.view.state.values['input-title'].input.value,
      echo_string: body.view.state.values['input-description'].input.value
    }
  });

  axios.post(`/repos/${process.env.GITHUB_OWNER}/${process.env.GITHUB_REPO}/dispatches`, requestBody)
    .then(response => (this.info = response))
    .catch(function(e) {
      console.log(e);
  });
});

function printCompleteJSON(error) {
  console.log(JSON.stringify(error));
}
