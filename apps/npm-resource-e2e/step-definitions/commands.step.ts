import { Given, Then, When, World } from '@cucumber/cucumber';
import { Run } from '../src/common/run';
import { strict } from 'assert';

Given(
  /we execute the (in|out|check)/,
  function (this: World, commandType: 'in' | 'out' | 'check') {
    this.parameters.command = new Run(this.parameters.type).command(
      commandType
    );
  }
);

When(/we execute the command/, function (this: World) {
  try {
    this.parameters.commandResult = this.parameters.command.execSync();
  } catch (e) {
    this.parameters.commandError = e;
  }
});

Given(/we provide the arg : (\S+)/, function (this: World, arg: string) {
  this.parameters.command.path = arg;
});

Then(
  /The command exited with a code (\d+)/,
  function (this: World, code: number) {
    strict.equal(code, this.parameters.commandError.status);
  }
);

Then(
  'The command threw a message containing {string}',
  function (this: World, message: string) {
    strict.ok(this.parameters.commandError.message.indexOf(message) > -1);
  }
);
