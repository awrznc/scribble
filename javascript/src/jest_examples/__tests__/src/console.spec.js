const cnsl = require('../../src/console');

describe('#printError', () => {
  it('error が表示される', () => {
    const errorSpyLog = jest.spyOn(console, 'error');
    errorSpyLog.mockImplementation(x => x);
    cnsl.printError();
    expect( errorSpyLog.mock.calls[0][0] ).toBe('print');
    expect( errorSpyLog.mock.calls[1][0] ).toBe('error');
    expect( errorSpyLog.mock.calls[2] ).toBe(undefined);
  });
});