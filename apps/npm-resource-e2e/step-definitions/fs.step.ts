import { Tmpdir } from '../src/common/tmpdir';
import { Given, Then, World } from '@cucumber/cucumber';
import { pathExists } from 'fs-extra';
import { join } from 'path';
import { strict as assert } from 'assert';

Given('into a temporary directory', function (this: World) {
  this.parameters.target = Tmpdir.withRandomName();
  this.parameters.command.path = this.parameters.target.path;
});

Then(
  'the file package.json is in the temporary directory',
  async function (this: World) {
    assert.ok(
      await pathExists(join(this.parameters.target.path, 'package.json'))
    );
  }
);
