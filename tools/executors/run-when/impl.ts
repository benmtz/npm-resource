import {
  ExecutorContext,
  logger,
  parseTargetString,
  runExecutor,
} from '@nrwl/devkit';
import waitOn = require('wait-on');

export type Json = { [k: string]: any };

interface WaitForTarget extends Json {
  target: string;
  waitOnResource: string;
}

interface Options extends Json {
  target: string;
  runAndWaitForTargets: Array<WaitForTarget>;
}

async function startDevServer(t: WaitForTarget, context: ExecutorContext) {
  logger.debug(`parsing ${t.target}`);
  const { project, target, configuration } = parseTargetString(t.target);
  const _output = runExecutor<{
    success: boolean;
    baseUrl?: string;
  }>({ project, target, configuration }, {}, context);
  logger.info(`ran ${t.target} successfully`);
  await waitOn({
    resources: [t.waitOnResource],
  });
  return;
}

export default async function createBuilder(
  options: Options,
  context: ExecutorContext
) {
  logger.info(`Executing "run-when"...`);
  logger.debug(`Options: ${JSON.stringify(options, null, 2)}`);

  await Promise.all(
    options.runAndWaitForTargets.map((runThis) =>
      startDevServer(runThis, context)
    )
  );
  logger.debug('everything is up we can start main');
  const { project, target, configuration } = parseTargetString(options.target);
  for await (const output of await runExecutor<{
    success: boolean;
  }>({ project, target, configuration }, {}, context)) {
    if (!output.success) {
      throw new Error('failed');
    }
  }

  return { success: true };
}
