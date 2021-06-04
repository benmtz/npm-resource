import { BuilderOutput, createBuilder } from '@angular-devkit/architect';
import { Observable } from 'rxjs';
import { json } from '@angular-devkit/core';
import * as childProcess from 'child_process';

interface Options extends json.JsonObject {
  port: string;
  stepsGlob: string;
  publishQuiet: boolean;
  tsconfig: string;
  args: string;
}

const argifyOptions = (options: Options) => {
  const args = ['--require-module', 'ts-node/register'];
  if (options.tsconfig) {
    process.env['TS_NODE_PROJECT'] = options.tsconfig;
  }
  if (options.stepsGlob) {
    args.push('--require', options.stepsGlob);
  }
  if (options.publishQuiet) {
    args.push('--publish-quiet');
  }
  return args;
};

export default createBuilder((_options: Options, context) => {
  context.logger.info(`Executing "cucumber"...`);
  context.logger.info(`Options: ${JSON.stringify(_options, null, 2)}`);

  const child = childProcess.spawn(
    './node_modules/.bin/cucumber-js',
    argifyOptions(_options)
  );
  return new Observable<BuilderOutput>((observer) => {
    child?.stdout.on('data', (data) => {
      context.logger.info(data.toString());
    });
    child?.stderr.on('data', (data) => {
      context.logger.error(data.toString());
    });
    child.on('close', (code) => {
      context.logger.info(`Done.`);
      observer.next({ success: code === 0 });
      observer.complete();
    });
  });
});
