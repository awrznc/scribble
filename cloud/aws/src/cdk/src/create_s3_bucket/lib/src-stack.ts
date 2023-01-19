import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import * as s3 from 'aws-cdk-lib/aws-s3';

export class SrcStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const property: cdk.aws_s3.BucketProps = {
      bucketName: 'cdk.piyopiyo',

      // これがないと削除時に残ってしまう
      removalPolicy: cdk.RemovalPolicy.DESTROY,
    }
    const s3_object = new s3.Bucket(this, 'cdk.piyopiyo', property);
  }
}
