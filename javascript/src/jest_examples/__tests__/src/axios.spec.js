const Axs = require('../../src/axios');

describe('#httpGet', () => {
  it('モックの内容が返される', async () => {
    const fs = require('fs');
    const response = fs.readFileSync('__mocks__/example.json', 'utf-8');
    expect( (await Axs.httpGet()).data ).toEqual( JSON.parse(response));
  });
});
