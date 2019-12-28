const axios = require('axios');

if (process.env.TEST == 'true') {
  const MockAdapter = require('axios-mock-adapter');
  const mock = new MockAdapter(axios);
  const fs = require('fs');
  const response = fs.readFileSync('__mocks__/example.json', 'utf-8');
  mock.onGet('https://example.com').reply(200, response);
}


exports.httpGet = function () {
  return axios.get('https://example.com').then((results) => {
    return results;
  });
}
