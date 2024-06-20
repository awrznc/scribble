import * as vscode from 'vscode';

export function activate(context: vscode.ExtensionContext) {

    context.subscriptions.push(
        vscode.commands.registerCommand('panel.helloWorld', () => {
            const panel = vscode.window.createWebviewPanel(

                // 内部で利用されるID
                'hello',

                // パネル（表示されるタブ）のタイトル
                'Piyo',

                // パネルを表示する場所
                vscode.ViewColumn.One,

                // その他のオプション
                {}
            );

            // 表示されるタブ
            panel.webview.html = getWebviewContent();
        })
    );
}

function getWebviewContent() {
    return `<!DOCTYPE html>
        <html lang="ja">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Hello!</title>
        </head>
        <body>
            <p>hello!</p>
        </body>
        </html>`;
}
