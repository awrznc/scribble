describe('Sample', () => {
  it('check env', () => {

    // 環境変数の読み込み
    const envVars = ServerlessWrapper.getEnv('hello');

    // 表示
    console.log(process.env);
    console.log(envVars);

    expect('');
  });
});
