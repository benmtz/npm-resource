import { Given, World } from '@cucumber/cucumber';
import * as yaml from 'yamljs';

Given('resource source is :', function (this: World, docString: string) {
  const source = yaml.parse(docString);
  this.parameters.command.payload.source = source;
});

Given('parameters are empty', function (this: World) {
  this.parameters.command.payload.params = {};
});

Given(
  /checked version is (\d+\.\d+\.\d+)/,
  function (this: World, semver: string) {
    this.parameters.command.payload.version = {
      version: semver,
    };
  }
);

Given('params is :', function (docString) {
  const source = yaml.parse(docString);
  this.parameters.command.payload.params = source;
});
