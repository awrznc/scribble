const hello = require('../../src/hello');

describe('#getHelloWorld', () => {
  it('"Hello, World!"が表示される', () => {
    expect(hello.getHelloWorld()).toBe('Hello, World!');
  });
});
