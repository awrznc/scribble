AWS SAM
===

サーバーレスアプリケーション構築用のフレームワーク。

## Install

https://docs.aws.amazon.com/ja_jp/serverless-application-model/latest/developerguide/serverless-sam-cli-install.html


## Quick Start Example

```bash
$ sam --version
SAM CLI, version 1.46.0

$ sam init

You can preselect a particular runtime or package type when using the `sam init` experience.
Call `sam init --help` to learn more.

Which template source would you like to use?
	1 - AWS Quick Start Templates
	2 - Custom Template Location
Choice: 1

Choose an AWS Quick Start application template
	1 - Hello World Example
	2 - Multi-step workflow
	3 - Serverless API
	4 - Scheduled task
	5 - Standalone function
	6 - Data processing
	7 - Infrastructure event management
	8 - Machine Learning
Template: 1

 Use the most popular runtime and package type? (Python and zip) [y/N]: n

Which runtime would you like to use?
	1 - dotnet6
	2 - dotnet5.0
	3 - dotnetcore3.1
	4 - go1.x
	5 - graalvm.java11 (provided.al2)
	6 - graalvm.java17 (provided.al2)
	7 - java11
	8 - java8.al2
	9 - java8
	10 - nodejs14.x
	11 - nodejs12.x
	12 - python3.9
	13 - python3.8
	14 - python3.7
	15 - python3.6
	16 - ruby2.7
	17 - rust (provided.al2)
Runtime: 4

What package type would you like to use?
	1 - Zip
	2 - Image
Package type: 1

Based on your selections, the only dependency manager available is mod.
We will proceed copying the template using mod.

Project name [sam-app]: example

Cloning from https://github.com/aws/aws-sam-cli-app-templates (process may take a moment)

    -----------------------
    Generating application:
    -----------------------
    Name: example
    Runtime: go1.x
    Architectures: x86_64
    Dependency Manager: mod
    Application Template: hello-world
    Output Directory: .

    Next steps can be found in the README file at ./example/README.md


    Commands you can use next
    =========================
    [*] Create pipeline: cd example && sam pipeline init --bootstrap
    [*] Test Function in the Cloud: sam sync --stack-name {stack-name} --watch


SAM CLI update available (1.50.0); (1.46.0 installed)
To download: https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/serverless-sam-cli-install.html
```