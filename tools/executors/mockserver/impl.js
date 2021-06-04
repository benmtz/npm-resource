'use strict';
exports.__esModule = true;
var architect_1 = require('@angular-devkit/architect');
var rxjs_1 = require('rxjs');
var childProcess = require('child_process');
var argifyOptions = function (options) {
  var args = ['--require-module', 'ts-node/register'];
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
exports['default'] = architect_1.createBuilder(function (_options, context) {
  context.logger.info('Executing "cucumber"...');
  context.logger.info('Options: ' + JSON.stringify(_options, null, 2));
  var child = childProcess.spawn(
    './node_modules/.bin/cucumber-js',
    argifyOptions(_options)
  );
  return new rxjs_1.Observable(function (observer) {
    child === null || child === void 0
      ? void 0
      : child.stdout.on('data', function (data) {
          context.logger.info(data.toString());
        });
    child === null || child === void 0
      ? void 0
      : child.stderr.on('data', function (data) {
          context.logger.error(data.toString());
        });
    child.on('close', function (code) {
      context.logger.info('Done.');
      observer.next({ success: code === 0 });
      observer.complete();
    });
  });
});
