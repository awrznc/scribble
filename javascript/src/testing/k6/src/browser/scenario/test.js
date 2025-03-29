// https://jslib.k6.io/
import { browser } from 'k6/browser';
import { describe, expect } from 'https://jslib.k6.io/k6chaijs/4.5.0.1/index.js';

const host = 'server';

export const options = {
    scenarios: {
        ui: {
            executor: 'shared-iterations',
            options: {
                browser: {
                    type: 'chromium',
                },
            },
        },
    },
    // virtual users
    vus: 1,
    // 合否基準
    thresholds: {
        // check の成功率が 100% なら合格
        checks: ['rate === 1.00'],
        // 失敗率が 0% 以下なら合格
        browser_http_req_failed: ['rate <= 0.00']
    },
};

export default async function () {
    const context = await browser.newContext();
    const page = await context.newPage();

    try {
        // Index Page
        {
            // アクセスチェック
            const response = await page.goto(`http://${host}/index.html`);
            const url = await page.url();
            const title = await page.title();
            const status = await response.status();

            // 題名チェック
            const h1 = await page.locator('h1');
            const h1Text = await h1.textContent();

            // リンクチェック
            const a = await page.locator('a');
            const aText = await a.textContent();

            describe('Index Page', () => {
                expect(status, `HTTP Response Status Code ( ${status} )`).to.equal(200);
                expect(url, `url ( ${url} )`).to.equal(`http://${host}/index.html`);
                expect(title, `title ( ${title} )`).to.equal('Index');
                expect(h1Text, `h1 text ( ${h1Text} )`).to.equal('Hello, World!');
                expect(aText, `a text ( ${aText} )`).to.equal('Piyo');
            });

            // ページ遷移
            await Promise.all([page.waitForNavigation(), a.click()]);
        };

        // Piyo Page
        {
            // アクセスチェック
            const url = await page.url();
            const title = await page.title();

            // 題名チェック
            const h1 = await page.locator('h1');
            const h1Text = await h1.textContent();

            // リンクチェック
            const aIndex = await page.locator('a#index');
            const aIndexText = await aIndex.textContent();
            const aReturn = await page.locator('a#return');
            const aReturnText = await aReturn.textContent();

            // テーブルチェック
            // この方法はあまりお勧めできないのでidか何かで判別できるようにしたほうがいい
            // 順番が担保されないようなので、ソートしてから確認した方がいいかも
            // https://grafana.com/docs/k6/latest/javascript-api/k6-browser/page/page-doubledollar/
            const tdTextListResult = (await page.$$('td')).map(async (value) => await value.innerText());
            const tdTextList = (await Promise.all(tdTextListResult)).sort();
            const tdText = JSON.stringify(tdTextList);

            describe('Piyo Page', () => {
                expect(title, `title ( ${title} )`).to.equal('Piyo');
                expect(url, `url ( ${url} )`).to.equal(`http://${host}/puyo.html`);
                expect(h1Text, `h1 text ( ${h1Text} )`).to.equal('Piyo');
                expect(aIndexText, `a index text ( ${aIndexText} )`).to.equal('Index');
                expect(aReturnText, `a return text ( ${aReturnText} )`).to.equal('Return');
                expect(tdText, `td text ( ${tdText} )`).to.equal(JSON.stringify(['1', '11', '2', '22', 'Piyo', 'Puyo']));
            });

            // ページ遷移
            await Promise.all([page.waitForNavigation(), aIndex.click()]);
        };
    } catch (error) {
        describe('Script Error', () => {
            expect(error, `script error ( ${error} )`).to.equal(null);
        });
    } finally {
        await page.close();
        await context.close()
    }
}
